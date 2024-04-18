import fetchAPI from "../../../utils/fetchAPI";
import { JSONRPC } from "../../../utils/types";
import fixtures from "../../../fixtures";

const chain_getBlockHash = async (blockNumber?: number): Promise<JSONRPC> => {
    const params = typeof blockNumber === 'number' ? [blockNumber] : [];

    return await fetchAPI({
        options: {
            id: fixtures.id,
            jsonrpc: fixtures.jsonrpc,
            method: "chain_getBlockHash",
            params: params,
        },
    });
};

export default chain_getBlockHash;
