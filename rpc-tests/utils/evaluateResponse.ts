import { expect } from "@jest/globals";
import fixtures from "../fixtures";
import debug from 'debug';
require("dotenv").config();

const log = debug('app:log');

if (process.env.DEV_MODE === 'true') {
  debug.enable('app:log');
} else {
  debug.disable();
}

// A robust version of the "typeof" operator
// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/typeof#custom_method_that_gets_a_more_specific_type
function type(value) {
  if (value === null) {
    log("Detected type: null");
    return "null";
  }

  const baseType = typeof value;
  if (!["object", "function"].includes(baseType)) {
    log(`Detected primitive type: ${baseType}`);
    return baseType;
  }

  const tag = value[Symbol.toStringTag];
  if (typeof tag === "string") {
    return tag;
  }

  if (baseType === "function" && Function.prototype.toString.call(value).startsWith("class")) {
    log(`Detected class type: class`);
    return "class";
  }

  const className = value.constructor.name;
  if (typeof className === "string" && className !== "") {
    return className;
  }

  log(`Fallback type detection, using base type: ${baseType}`);
  return baseType;
}

function testString(value, pattern) {
  log(`Testing string: Value = ${value}, Pattern = ${pattern}`);
  if (pattern) {
    expect(value).toMatch(pattern);
  } else {
    testNull(value);
  }
}

function testNumber(value) {
  log(`Testing number: Value = ${value}`);
  expect(value).not.toBeNaN();
}

function testNull(value) {
  log(`Testing for null: Value = ${value}`);
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
  log(`Iterating array items: ${JSON.stringify(value)}, Pattern: ${JSON.stringify(pattern)}`);
  value.forEach((item, index) => {
    if (Array.isArray(pattern)) {
      if (pattern.length === 1 && Array.isArray(pattern[0])) {
        // Tuple
        if (pattern[0].length !== item.length) {
          throw new Error(`Tuple pattern does not match item structure at index ${index}`);
        }
        item.forEach((element, idx) => {
          reduceValue(element, pattern[0][idx]);
        });
      } else {
        // Regular array
        reduceValue(item, pattern[0]);
      }
    }
  });
}

function iterateObjectProperties(value, pattern) {
  for (const key in value) {
    reduceValue(value[key], (pattern && pattern[key]) || null);
  }
}

function reduceValue(value, pattern) {
  log(`Reducing value: Type = ${type(value)}, Value = ${JSON.stringify(value)}, Pattern = ${JSON.stringify(pattern)}`);
  switch(type(value)) {
    case "Array":
      if (Array.isArray(pattern[0])) {
        // Handling for tuple arrays
        iterateArrayItems(value, pattern[0]);
      } else {
        // Regular array handling
        iterateArrayItems(value, pattern);
      }
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

  log(`response: ${JSON.stringify(response, null, 2)}\npattern: ${JSON.stringify(pattern, null, 2)}`);
}

export default evaluateResponse;
 