import parseYaml from "../utils/parseYaml";

function getType($ref) {
  const $refDirName = $ref.split("/");
  const type = $refDirName[$refDirName.length - 1];
  console.log(`Resolved type from $ref: ${type}`);
  return type;
}

function getRpcDefinition(rpcDefinitionPath, rpcName) {
  const definitions = parseYaml(rpcDefinitionPath);
  const definition = definitions.find(item => item.name === rpcName);
  console.log(`RPC Definition for ${rpcName}: ${JSON.stringify(definition)}`);
  return definition;
}

function getSchema() {
  const baseTypes = parseYaml("../schemas/base-types.yaml");
  const block = parseYaml("../schemas/block.yaml");
  const header = parseYaml("../schemas/header.yaml");

  const combinedSchema = {
    ...baseTypes,
    ...block,
    ...header,
  };
  console.log(`Combined Schema: ${JSON.stringify(combinedSchema)}`);
  return combinedSchema;
}

function isBaseType(type) {
  const baseTypes = parseYaml("../schemas/base-types.yaml");
  const isBase = baseTypes.hasOwnProperty(type);
  console.log(`Is ${type} a base type? ${isBase}`);
  return isBase;
}

function getPattern(type) {
  const schema = getSchema();
  const schemaType = schema[type];
  console.log(`Fetching pattern for type ${type}: ${JSON.stringify(schemaType)}`);

  if (schemaType && schemaType.pattern) {
    return new RegExp(schemaType.pattern);
  }

  if (schemaType && schemaType.properties) {
    return iterateObjectProperties(schemaType.properties);
  }

  console.error(`No pattern or properties found for type ${type}`);
  return null;
}

function iterateObjectProperties(properties) {
  const pattern = {};
  console.log(`Iterating over properties: ${JSON.stringify(properties)}`);
  for (const key in properties) {
    const propertyValue = properties[key];
    console.log(`Processing property ${key}: ${JSON.stringify(propertyValue)}`);

    if (propertyValue.$ref) {
      pattern[key] = getPattern(getType(propertyValue.$ref));
    } else if (propertyValue.type === 'array' && propertyValue.items) {
      pattern[key] = [getPattern(getType(propertyValue.items.$ref || propertyValue.items.type))];
      console.log(`Array pattern for ${key}: ${JSON.stringify(pattern[key])}`);
    } else if (propertyValue.type === 'object' && propertyValue.properties) {
      pattern[key] = iterateObjectProperties(propertyValue.properties);
      console.log(`Object pattern for ${key}: ${JSON.stringify(pattern[key])}`);
    } else {
      console.error(`Unsupported property definition for ${key}: ${JSON.stringify(propertyValue)}`);
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
  return iterateObjectProperties(rpcDefinition.result.schema.properties);
}

async function buildMainPattern({ rpcDefinitionPath, rpcName }) {
  const rpcDefinition = getRpcDefinition(rpcDefinitionPath, rpcName);
  return getPattern(getType(rpcDefinition.result.schema.$ref));
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
