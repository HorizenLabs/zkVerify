import { describe } from "@jest/globals";
import evaluateResponse from "../../../utils/evaluateResponse";
import patternGenerator from "../../../utils/patternGenerator";
import chain_getFinalizedHead from "./index";

describe("chain_getFinalizedHead", () => {
  it("Returns a block.", async () => {
    evaluateResponse({
      response: await chain_getFinalizedHead(), 
      pattern: await patternGenerator.buildMainPattern({
        rpcDefinitionPath: "../schemas/definitions/chain.yaml",
        rpcName: "chain_getFinalizedHead",
      }),
    });
  });
});