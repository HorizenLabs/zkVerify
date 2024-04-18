import { describe } from "@jest/globals";
import evaluateResponse from "../../../utils/evaluateResponse";
import patternGenerator from "../../../utils/patternGenerator";
import chain_getBlock from "./index";

describe("chain_getBlock", () => {
  it("Returns a block.", async () => {
    evaluateResponse({
      response: await chain_getBlock(process.env.BLOCK_HASH), 
      pattern: await patternGenerator.buildMainPattern({
        rpcDefinitionPath: "../schemas/definitions/chain.yaml",
        rpcName: "chain_getBlock",
      }),
    });
  });
});