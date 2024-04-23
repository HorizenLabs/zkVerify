import fetchAPI from "../../../utils/fetchAPI";
import { JSONRPC } from "../../../utils/types";
import fixtures from "../../../fixtures";

const state_getMetadata = async (blockHash = null): Promise<JSONRPC> => {
    const params = blockHash ? [blockHash] : [];

    return await fetchAPI({
        options: {
            id: fixtures.id,
            jsonrpc: fixtures.jsonrpc,
            method: "state_getMetadata",
            params: params,
        },
    });
};

export default state_getMetadata;