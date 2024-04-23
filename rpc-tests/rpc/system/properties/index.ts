import fetchAPI from "../../../utils/fetchAPI";
import { JSONRPC } from "../../../utils/types";
import fixtures from "../../../fixtures";

const system_properties = async (): Promise<JSONRPC> => {
    const params = [];

    return await fetchAPI({
        options: {
            id: fixtures.id,
            jsonrpc: fixtures.jsonrpc,
            method: "system_properties",
            params: params,
        },
    });
};

export default system_properties;