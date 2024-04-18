type JSONRPC = {
  jsonrpc: string;
  id: number;
  result?: any;
  error?: {
    code: number;
    message: string;
    data: string;
  };
}

type Options = {
  [ key: string ]: any;
}

export type {
  JSONRPC,
  Options,
}
