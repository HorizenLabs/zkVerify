import fetchAPI from "../../../utils/fetchAPI";
import { JSONRPC } from "../../../utils/types";
import fixtures from "../../../fixtures";

const chain_subscribeNewHeads = async (): Promise<JSONRPC> => {
    const params = [];

    return await fetchAPI({
        options: {
            id: fixtures.id,
            jsonrpc: fixtures.jsonrpc,
            method: "chain_subscribeNewHeads",
            params: params,
        },
    });
};

export default chain_subscribeNewHeads;