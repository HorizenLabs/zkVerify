import fetchAPI from "../../../utils/fetchAPI";
import { JSONRPC } from "../../../utils/types";
import fixtures from "../../../fixtures";

const state_getRuntimeVersion = async (blockHash = null): Promise<JSONRPC> => {
    const params = blockHash ? [blockHash] : [];

    return await fetchAPI({
        options: {
            id: fixtures.id,
            jsonrpc: fixtures.jsonrpc,
            method: "state_getRuntimeVersion",
            params: params,
        },
    });
};

export default state_getRuntimeVersion;