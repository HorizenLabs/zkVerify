import { describe } from "@jest/globals";
import evaluateResponse from "../../../utils/evaluateResponse";
import patternGenerator from "../../../utils/patternGenerator";
import chain_subscribeNewHeads from "./index";

describe("chain_subscribeNewHeads", () => {
  it("Retrieves the best header via subscription", async () => {
    evaluateResponse({
      response: await chain_subscribeNewHeads(),
      pattern: await patternGenerator.buildMainPattern({
        rpcDefinitionPath: "../schemas/definitions/chain.yaml",
        rpcName: "chain_subscribeNewHeads",
      }),
    });
  });
});