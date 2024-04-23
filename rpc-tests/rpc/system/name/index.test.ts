import { describe } from "@jest/globals";
import evaluateResponse from "../../../utils/evaluateResponse";
import patternGenerator from "../../../utils/patternGenerator";
import system_name from "./index";

describe("system_name", () => {
  it("Retrieves the node name", async () => {
    evaluateResponse({
      response: await system_name(), 
      pattern: await patternGenerator.buildObjectPattern({
        rpcDefinitionPath: "../schemas/definitions/system.yaml",
        rpcName: "system_name",
      }),
    });
  });
});