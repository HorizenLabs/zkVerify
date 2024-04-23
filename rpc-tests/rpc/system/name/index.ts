import fetchAPI from "../../../utils/fetchAPI";
import { JSONRPC } from "../../../utils/types";
import fixtures from "../../../fixtures";

const system_name = async (): Promise<JSONRPC> => {
    const params = [];

    return await fetchAPI({
        options: {
            id: fixtures.id,
            jsonrpc: fixtures.jsonrpc,
            method: "system_name",
            params: params,
        },
    });
};

export default system_name;