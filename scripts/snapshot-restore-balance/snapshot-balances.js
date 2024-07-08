const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const fs = require('fs');

const WS_ENDPOINT = 'wss://testnet-rpc.zkverify.io';
const DISCARDED_ACCOUNTS = [
        "5D9txxK9DTvgCznTjJo7q1cxAgmWa83CzHvcz8zhBtLgaLBV",
        "5ETuZEyLnfVzQCaDM8aQCcsNnz6xjPKvQCtqynCLqwng8QLd",
        "5D29UEzgStCBTnjKNdkurDNvd3FHePHgTkPEUvjXYvg3brJj",
        "5DiMVAp8WmFyWAwaTwAr7sU4K3brXcgNCBDbHoBWj3M46PiP",
        "5EhREncHsntgJaax9YQphk1xN3LxPu2Rzbz4A3g7Ut8cRXWq",
        "5C84NU2477uHCUF1A8rHb89sP2D2ZsnquPaGa2Htv75FN9gm",
        "5HdZjrmNAkWQhYQUPNv7YRYnT4vyQswjbNm8eXBvULNQz5wH",
        "5HjFLKpiCStQgRm6ZM1fT1R5pLKAqQdUG3uh7pvzaQfhdFuB",
        "5ECktCamcAtBFJirEzvvJmXFxgLMCTAejhqZwLT1Dxn2fwB1",
        "5EZbvFqx3j7ejqBSPWseif8xL3PwoqMQHdMT8rs9qWoHcdR3"
    ]

async function takeSnapshot(api, snapshotBalancesFile) {
    const balances = {};
    const accounts = await api.query.system.account.keys();

    for (const key of accounts) {
        const account = key.args[0].toString();
        if (DISCARDED_ACCOUNTS.includes(account)) continue;
        const { data:  { free: freeBalance, reserved: reservedBalance } } = await api.query.system.account(account);
        const totalBalance = BigInt(freeBalance.toString()) + BigInt(reservedBalance.toString());
        balances[account] = totalBalance.toString();
    }

    const sortedBalances = Object.entries(balances).sort((a, b) => (BigInt(b[1]) < BigInt(a[1]) ? 1 : -1));
    const sortedBalancesObj = sortedBalances.reduce((acc, [account, balance]) => {
        acc[account] = balance;
        return acc;
    }, {});

    fs.writeFileSync(snapshotBalancesFile, JSON.stringify(sortedBalancesObj, null, 2));

    console.log('Snapshot saved.');
}

// Restore balances from the snapshot
async function restoreBalances(api, keyring, snapshotBalancesFile, custodySeed) {
    const balances = JSON.parse(fs.readFileSync(snapshotBalancesFile));
    const custodyAccount = keyring.addFromUri(custodySeed);
    let nonce = (await api.query.system.account(custodyAccount.address)).nonce.toNumber();

    const totalAmount = Object.values(balances).reduce((acc, balance) => acc + BigInt(balance), BigInt(0));
    const { data: { free: custodyFreeBalance } } = await api.query.system.account(custodyAccount.address);

    if (BigInt(custodyFreeBalance.toString()) < totalAmount) {
        console.error(`Insufficient balance in custody account. Required: ${totalAmount.toString()}, Available: ${custodyFreeBalance.toString()}`);
        process.exit(1);
    }

    for (const [account, balance] of Object.entries(balances)) {
        const transfer = api.tx.balances.transferAllowDeath(account, balance);
        try {
            const hash = await transfer.signAndSend(custodyAccount, { nonce });
            console.log(`Transferred to ${account}, transaction hash: ${hash.toHex()}`);
            nonce++;
        } catch (error) {
            console.error(`Failed to transfer to ${account}:`, error);
        }
    }
}

async function main() {
    const provider = new WsProvider(WS_ENDPOINT);
    const api = await ApiPromise.create({ provider });
    const keyring = new Keyring({ type: 'sr25519' });

    const [,, command, arg1, arg2] = process.argv;

    if (command === 'snapshot') {
        if (!arg1) {
            console.error('Please provide the snapshot file name.');
            process.exit(1);
        }
        await takeSnapshot(api, arg1);
    } else if (command === 'restore') {
        if (!arg1 || !arg2) {
            console.error('Please provide the custody seed and snapshot balances file name.');
            process.exit(1);
        }
        await restoreBalances(api, keyring, arg2, arg1);
    } else {
        console.error('Unknown command. Use "snapshot" i.e. "node snapshot-balances.js snapshot snapshot_balances.json" or "restore" i.e. "node snapshot-balances.js restore "//Alice" snapshot_balances.json".');
        process.exit(1);
    }

    process.exit(0);
}

main().catch(console.error);

