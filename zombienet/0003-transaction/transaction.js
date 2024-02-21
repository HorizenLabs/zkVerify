// There are a few global variables that are declared in zombienet js test executor but 
// they are not properly documented. So far, I found these:
// - zombie <- there a comment along the test about that
// - window
// - document <- alias for window.document, LOL!
//
// All the api.* calls refer to the polkadot.js api list, which is available here:
// https://polkadot.js.org/docs/substrate
// As now, I am unsure if we can also use the Polkadot / Kusama apis as well
//
// args can be passed from the .zndsl file as a comma-separated list of values, surrounded
// by double quotes

async function run(nodeName, networkInfo, args) {
    const {wsUri, userDefinedTypes} = networkInfo.nodesByName[nodeName];
    const api = await zombie.connect(wsUri, userDefinedTypes);

    // This is not really used by this test, but shows how to get the current block number
    const current_block_number = await api.query.system.number();
    console.log('Current block number: ' + current_block_number);

    // // This also is not used, but shows how to interact with Granpa pallet
    // const granpa_current_set_id = await api.query.grandpa.currentSetId();
    // const granpa_authorities = await api.query.grandpa.authorities();

    // Define Alice and Bob's addresses
    const ALICE = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
    const BOB   = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';

    // Build a keyring and import Alice's credential
    // There's no documentation on that, it has been deducted from:
    // javascript/packages/orchestrator/src/test-runner/assertion.ts
    // By the way, zombie contains also the following objects / functions:
    //    ApiPromise, Keyring, WsProvider, util: utilCrypto, connect(), registerParachain()
    const keyring = new zombie.Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');

    // Collect Alice's and Bob's free balances
    let balance_alice = (await api.query.system.account(ALICE))["data"]["free"];
    let balance_bob   = (await api.query.system.account(BOB))["data"]["free"];
    console.log('Alice\'s balance: ' + balance_alice.toHuman());
    console.log('Bob\'s balance:   ' + balance_bob.toHuman());

    // Create an extrinsic, transferring 1 token unit to Bob.
    const transfer = await api.tx.balances.transferAllowDeath(BOB, 1);

    // We sign and submit the extrinsic. We need to surround the execution of the api call
    // around a Promise to block the test until the transaction gets finalized, thus
    // preventing the main function to return before this event happens
    let transaction_success_event = false;
    await new Promise( async (resolve, reject) => {

      const unsub = await transfer.signAndSend(alice, ({ events = [], status }) => {
        console.log('Transaction status:', status.type);

        if (status.isInBlock) {
          console.log(`Transaction included at blockhash ${status.asInBlock}`);
          console.log('Events:');

          // Be aware that when a transaction status is isFinalized, it means it is included,
          // but it may still have failed - for instance if you try to send a larger amount
          // that you have free, the transaction is included in a block, however from a
          // end-user perspective the transaction failed since the transfer did not occur.
          // In these cases a system.ExtrinsicFailed event will be available in the events array.
          events.forEach(({ event: { data, method, section }, phase }) => {
            console.log('\t', phase.toString(), `: ${section}.${method}`, data.toString());
            if (section == "system" && method == "ExtrinsicSuccess") {
              transaction_success_event = true;
            }
          });

        }

        else if (status.isFinalized) {
          console.log(`Transaction finalized at blockhash ${status.asFinalized}`);
          unsub();
          if (transaction_success_event) {
            resolve();
          } else {
            reject("ExtrinsicSuccess has not been seen");
          }
        }

        else if (status.isError) {
          unsub();
          reject("Transaction status.isError");
        }

      });

    })
      .then(
        () => {
          console.log("Transaction successfully finalized and included in a block")
        },
        error => {
          return -1;
        }
      );

    // Get the updated balances
    balance_alice = (await api.query.system.account(ALICE))["data"]["free"];
    balance_bob   = (await api.query.system.account(BOB))["data"]["free"];
    console.log('Alice\'s balance after tx: ' + balance_alice.toHuman());
    console.log('Bob\'s balance after tx:   ' + balance_bob.toHuman());

    return balance_alice;
}

module.exports = { run }