import fetchAPI from "../../../utils/fetchAPI";
import { JSONRPC } from "../../../utils/types";
import fixtures from "../../../fixtures";

const chain_getFinalizedHead = async (): Promise<JSONRPC> => {
    return await fetchAPI({
        options: {
            id: fixtures.id,
            jsonrpc: fixtures.jsonrpc,
            method: "chain_getFinalizedHead",
            params: [],
        },
    });
};

export default chain_getFinalizedHead;
