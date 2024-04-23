import { describe } from "@jest/globals";
import evaluateResponse from "../../../utils/evaluateResponse";
import patternGenerator from "../../../utils/patternGenerator";
import state_getMetadata from "./index";

describe("state_getMetadata", () => {
  it("Returns the state metadata.", async () => {
    evaluateResponse({
      response: await state_getMetadata(), 
      pattern: await patternGenerator.buildStringPattern({
        rpcDefinitionPath: "../schemas/definitions/state.yaml",
        rpcName: "state_getMetadata",
      }),
    });
  });
});