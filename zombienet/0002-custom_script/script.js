async function run(nodeName, networkInfo, args) {
    const {wsUri, userDefinedTypes} = networkInfo.nodesByName[nodeName];
    const api = await zombie.connect(wsUri, userDefinedTypes);
    console.log('##### Argument passed is: ' + args[0]);
    const localpeerid = await api.rpc.system.localPeerId(); // Returns the base-58 encoded peer id
    console.log('##### Local peer id: ' + localpeerid);
    return 2;
}

module.exports = { run }