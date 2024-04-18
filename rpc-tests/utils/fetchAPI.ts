import axios from "axios";
import {JSONRPC, Options,} from "./types";
require("dotenv").config();

function requiresAuth(method) {
  return method.startsWith('debug_');
}

async function fetchAPI({
                            httpMethod = "post",
                            options,
                            url = process.env.RPC_URL,
                        }: {
    httpMethod?: string;
    options: Options;
    url?: string;
}): Promise<JSONRPC> {

    const config: any = {};

    if (requiresAuth(options.method)) {
        config.auth = {
            username: process.env.RPC_USERNAME,
            password: process.env.RPC_PASSWORD
        };
    }

    try {
        const response = httpMethod === "post"
            ? await axios.post(url, options, config)
            : await axios[httpMethod](url, config);
        return response.data;
    } catch(error) {
        throw new Error(error);
    }
}

export default fetchAPI;
