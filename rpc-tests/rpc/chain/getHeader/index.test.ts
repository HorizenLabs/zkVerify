import { describe } from "@jest/globals";
import evaluateResponse from "../../../utils/evaluateResponse";
import patternGenerator from "../../../utils/patternGenerator";
import chain_getHeader from "./index";

describe("chain_getHeader", () => {
  it("Returns an object representing the header information of the requested block", async () => {
    evaluateResponse({
      response: await chain_getHeader(process.env.BLOCK_HASH), 
      pattern: await patternGenerator.buildMainPattern({
        rpcDefinitionPath: "../schemas/definitions/chain.yaml",
        rpcName: "chain_getHeader",
      }),
    });
  });
});