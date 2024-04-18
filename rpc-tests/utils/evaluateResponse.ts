import { expect } from "@jest/globals";
import fixtures from "../fixtures";
require("dotenv").config();

// A robust version of the "typeof" operator
// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/typeof#custom_method_that_gets_a_more_specific_type
function type(value) {
  if (value === null) {
    console.log("Detected type: null");
    return "null";
  }

  const baseType = typeof value;
  if (!["object", "function"].includes(baseType)) {
    console.log(`Detected primitive type: ${baseType}`);
    return baseType;
  }

  const tag = value[Symbol.toStringTag];
  if (typeof tag === "string") {
    console.log(`Detected tag type: ${tag}`);
    return tag;
  }

  if (baseType === "function" && Function.prototype.toString.call(value).startsWith("class")) {
    console.log(`Detected class type: class`);
    return "class";
  }

  const className = value.constructor.name;
  if (typeof className === "string" && className !== "") {
    console.log(`Detected constructor name type: ${className}`);
    return className;
  }

  console.log(`Fallback type detection, using base type: ${baseType}`);
  return baseType;
}


function testString(value, pattern) {
  console.log(`Testing string: Value = ${value}, Pattern = ${pattern}`);
  if (pattern) {
    expect(value).toMatch(pattern);
  } else {
    testNull(value);
  }
}

function testNumber(value) {
  console.log(`Testing number: Value = ${value}`);
  expect(value).not.toBeNaN();
}

function testNull(value) {
  console.log(`Testing for null: Value = ${value}`);
  expect(value).toBeNull();
}

function testBoolean(type) {
  expect(type).toBe("boolean");
}

function unsupportedType(type) {
  throw new Error(
    `Unsupported type. Expected "array", "object", "string", "number", "null", or "boolean" but received: "${type.toLowerCase()}".`
  );
}

function iterateArrayItems(value, pattern) {
  console.log(`Iterating array items: ${JSON.stringify(value)}, Pattern: ${JSON.stringify(pattern)}`);
  value.forEach(item => {
    if (pattern && pattern.any) {
      pattern.any.forEach((anyPattern) => {
        reduceValue(item, anyPattern);
      });
    } else {
      reduceValue(item, pattern[0]);
    }
  });
}

function iterateObjectProperties(value, pattern) {
  for (const key in value) {
    reduceValue(value[key], (pattern && pattern[key]) || null);
  }
}

function reduceValue(value, pattern) {
  console.log(`Reducing value: Type = ${type(value)}, Value = ${JSON.stringify(value)}, Pattern = ${pattern}`);
  switch(type(value)) {
    case "Array":
      iterateArrayItems(value, pattern);
      break;
    case "Object":
      iterateObjectProperties(value, pattern);
      break;
    case "string":
      testString(value, pattern);
      break;
    case "number":
      testNumber(value);
      break;
    case "null":
      testNull(value);
      break;
    case "boolean":
      testBoolean(value);
      break;
    default:
      unsupportedType(type(value));
  }
}

function evaluateResponse({ response, pattern, expectNullResult = false }) {
  const { jsonrpc, id, result: value, error } = response;
  if (error) throw new Error(`Error: ${JSON.stringify(error, null, 2)}`);

  expect(jsonrpc).toBe(fixtures.jsonrpc);
  expect(id).toBe(fixtures.id);

  if (expectNullResult) {
    expect(value).toBeNull();
  } else {
    if (value === null) {
      throw new Error('Unexpected null result');
    }

    reduceValue(value, pattern);
  }

  if (process.env.DEV_MODE === "true") {
    console.log("response:", response, "\n", "pattern:",pattern);
  }
}


export default evaluateResponse;
