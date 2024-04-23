import parseYaml from "../utils/parseYaml";

function getType($ref) {
  const $refDirName = $ref.split("/");
  return $refDirName[$refDirName.length - 1];
}

function getRpcDefinition(rpcDefinitionPath, rpcName) {
  return parseYaml(rpcDefinitionPath).find(item => item.name === rpcName);
}

function getSchema() {
  const baseTypes = parseYaml("../schemas/base-types.yaml");
  const block = parseYaml("../schemas/block.yaml");
  const runtime = parseYaml("../schemas/runtime.yaml");

  return {
    ...baseTypes,
    ...block,
    ...runtime,
  }
}

function isBaseType(type) {
  return Object
    .keys(parseYaml("../schemas/base-types.yaml"))
    .find(baseType => type === baseType);
}

function getPattern(type) {
  const schema = getSchema();
  const schemaType = schema[type];
  if (isBaseType(type) && schemaType) return new RegExp(schemaType.pattern);

  const { properties } = schemaType;
  if (properties) return iterateObjectProperties(properties);

  let combinedPattern = {};
  let combinedProperties;
  const { allOf, oneOf } = schemaType;
  if (allOf || oneOf) {
    combinedProperties = allOf || oneOf;
    combinedProperties.forEach((item) => {
      if (item.properties) combinedPattern = { ...iterateObjectProperties(item.properties) }
    });
    return combinedPattern;
  }

  return null;
}

function iterateObjectProperties(properties) {
  const pattern = {};
  for (const key in properties) {
    const propertyValue = properties[key];
    const { items, anyOf } = propertyValue;

    if (propertyValue.$ref) {
      // string
      pattern[key] = getPattern(getType(propertyValue.$ref));
      continue;
    }
    
    if (items) {
      if (items.items) {
        // array.array
        pattern[key] = [[getPattern(getType(propertyValue.items.items.$ref))]];
        continue;
      }

      // array
      pattern[key] = [getPattern(getType(propertyValue.items.$ref))];
      continue;
    }

    if (anyOf) {
      pattern[key] = { any: [] };
      anyOf.forEach((item) => {
        if (item.items) {
          const anyOfItemType = getType(item.items.$ref);
          const anyOfItemTypePattern = getPattern(anyOfItemType);
          if (isBaseType(anyOfItemType) && anyOfItemTypePattern) {
            // { any: [...string] }
            pattern[key].any.push(anyOfItemTypePattern);
          }

          const schema = getSchema();
          const { oneOf } = schema[anyOfItemType];
          if (oneOf) {
            let oneOfProperties = {};
            oneOf.forEach((item) => {
              const oneOfItemType = getType(item.$ref);
              oneOfProperties = { ...oneOfProperties, ...getPattern(oneOfItemType) };
            })
            // { any: [...object] }
            pattern[key].any.push(oneOfProperties);
          }
        }
      });
      continue;
    }
  }

  return pattern;
}

async function buildArrayPattern({ rpcDefinitionPath, rpcName }) {
  const rpcDefinition = getRpcDefinition(rpcDefinitionPath, rpcName);
  return [getPattern(getType(rpcDefinition.result.schema.items.$ref))];
}

async function buildObjectPattern({ rpcDefinitionPath, rpcName }) {
  const rpcDefinition = getRpcDefinition(rpcDefinitionPath, rpcName);
  const { properties } = rpcDefinition.result.schema;
  return iterateObjectProperties(properties);
}

async function buildMainPattern({ rpcDefinitionPath, rpcName }) {
  const rpcDefinition = getRpcDefinition(rpcDefinitionPath, rpcName);
  const { $ref } = rpcDefinition.result.schema;
  return getPattern(getType($ref));
}

async function buildSingleObjectPattern({ type }) {
  return getPattern(getType(type));
}

async function buildStringPattern({ rpcDefinitionPath, rpcName }) {
  const rpcDefinition = getRpcDefinition(rpcDefinitionPath, rpcName);
  return getPattern(getType(rpcDefinition.result.schema.$ref));
}

export default {
  getSchema,
  buildArrayPattern,
  buildMainPattern,
  buildObjectPattern,
  buildStringPattern,
  buildSingleObjectPattern,
};
