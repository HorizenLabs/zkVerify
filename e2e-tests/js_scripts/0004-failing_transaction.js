async function run(nodeName, networkInfo, args) {
    const {wsUri, userDefinedTypes} = networkInfo.nodesByName[nodeName];
    const api = await zombie.connect(wsUri, userDefinedTypes);

    // Define Alice and Bob's addresses
    const ALICE = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
    const BOB   = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';

    // Build a keyring and import Alice's credential
    const keyring = new zombie.Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');

    // Collect Alice's and Bob's free balances
    let balance_alice = (await api.query.system.account(ALICE))["data"]["free"];
    let balance_bob   = (await api.query.system.account(BOB))["data"]["free"];
    console.log('Alice\'s balance: ' + balance_alice.toHuman());
    console.log('Bob\'s balance:   ' + balance_bob.toHuman());

    // Create a extrinsic, transferring to Bob one token more than Alice can afford...
    const transfer = await api.tx.balances.transferAllowDeath(BOB, '1152921504606846977');

    // Sign and submit the extrinsic.
    let transaction_success_event = false;
    await new Promise( async (resolve, reject) => {

      const unsub = await transfer.signAndSend(alice, ({ events = [], status }) => {
        console.log('Transaction status:', status.type);

        if (status.isInBlock) {
          console.log(`Transaction included at blockhash ${status.asInBlock}`);
          console.log('Events:');

          events.forEach(({ event: { data, method, section }, phase }) => {
            console.log('\t', phase.toString(), `: ${section}.${method}`, data.toString());
            if (section == "system" && method == "ExtrinsicSuccess") {
              transaction_success_event = true;
            }
            else if (section == "system" && method == "ExtrinsicFailed") {
              transaction_success_event = false;
              //console.log(data[1]);
              reject("Extrinsic failed");
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
          transaction_success_event = false;
          reject("Transaction status.isError");
        }

      });

    })
      .then(
        () => {
          console.log("Transaction successfully finalized and included in a block")
        },
        error => {
          console.log("Transaction has failed: " + error);
        }
      );
    
    // This sucks big time, as return values are parsed as u64 - and nothing else - by the
    // zndsl interpreter, hence I cannot use negative values to indicate possible errors
    // or unexpected states...
    // In this particular test, I originally wanted to return the updated Alice balance if
    // the transaction were successful, or -1 in all the other cases. I had to resort to
    // just return a boolean flag, instead...
    if (!transaction_success_event) {
      console.log("Returning 0");
      return 0;
    }

    // Get the updated balances
    balance_alice = (await api.query.system.account(ALICE))["data"]["free"];
    balance_bob   = (await api.query.system.account(BOB))["data"]["free"];
    console.log('Alice\'s balance after tx: ' + balance_alice.toHuman());
    console.log('Bob\'s balance after tx:   ' + balance_bob.toHuman());

    // See the other comment...
    return 1;
}

module.exports = { run }