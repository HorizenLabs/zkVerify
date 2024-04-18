import fetchAPI from "../../../utils/fetchAPI";
import { JSONRPC } from "../../../utils/types";
import fixtures from "../../../fixtures";

const chain_getBlock = async (blockHash = null): Promise<JSONRPC> => {
    const params = blockHash ? [blockHash] : [];

    return await fetchAPI({
        options: {
            id: fixtures.id,
            jsonrpc: fixtures.jsonrpc,
            method: "chain_getBlock",
            params: params,
        },
    });
};

export default chain_getBlock;