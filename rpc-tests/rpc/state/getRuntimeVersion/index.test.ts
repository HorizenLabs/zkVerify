import { describe } from "@jest/globals";
import evaluateResponse from "../../../utils/evaluateResponse";
import patternGenerator from "../../../utils/patternGenerator";
import state_getRuntimeVersion from "./index";

describe("state_getRuntimeVersion", () => {
  it("Returns the runtime version.", async () => {
    evaluateResponse({
      response: await state_getRuntimeVersion(), 
      pattern: await patternGenerator.buildMainPattern({
        rpcDefinitionPath: "../schemas/definitions/state.yaml",
        rpcName: "state_getRuntimeVersion",
      }),
    });
  });
});