import { describe } from "@jest/globals";
import evaluateResponse from "../../../utils/evaluateResponse";
import patternGenerator from "../../../utils/patternGenerator";
import chain_getBlockHash from "./index";

describe("chain_getBlockHash", () => {
  it("Returns the hash associated with a block.", async () => {
    evaluateResponse({
      response: await chain_getBlockHash(), 
      pattern: await patternGenerator.buildStringPattern({
        rpcDefinitionPath: "../schemas/definitions/chain.yaml",
        rpcName: "chain_getBlockHash",
      }),
    });
  });
});