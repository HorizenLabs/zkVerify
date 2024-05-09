import { describe } from "@jest/globals";
import evaluateResponse from "../../../utils/evaluateResponse";
import patternGenerator from "../../../utils/patternGenerator";
import system_properties from "./index";

describe("system_properties", () => {
  it("Returns a custom set of properties as a JSON object.", async () => {
    evaluateResponse({
      response: await system_properties(), 
      pattern: await patternGenerator.buildMainPattern({
        rpcDefinitionPath: "../schemas/definitions/system.yaml",
        rpcName: "system_properties",
      }),
    });
  });
});