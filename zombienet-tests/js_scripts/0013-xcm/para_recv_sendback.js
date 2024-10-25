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

const { BN } = require('@polkadot/util');

const ReturnCode = {
    Ok: 1,
    WrongTeleportReceived: 2,
    ExtrinsicUnsuccessful: 3,
};

const { RISC0_VERIFY_CALL } = require('../verify_risc0_data.js')

async function run(nodeName, networkInfo, args) {
    const {wsUri, userDefinedTypes} = networkInfo.nodesByName[nodeName];
    const api = await zombie.connect(wsUri, userDefinedTypes);

    // Define Alice and Bob's addresses
    const ALICE = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
    const ALICE_REMOTE_ORIGIN = '0x7b2ac6587a1931a0b108bb03777f8e552293bd6a6ea3790a5fe14e214f13072b';

    // Build a keyring and import Alice's credential
    // There's no documentation on that, it has been deducted from:
    // javascript/packages/orchestrator/src/test-runner/assertion.ts
    // By the way, zombie contains also the following objects / functions:
    //    ApiPromise, Keyring, WsProvider, util: utilCrypto, connect(), registerParachain()
    const keyring = new zombie.Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');

    const amount = args[0];
    const receiver = args[1];
    const benef = args[1];
    // Collect Alice's and Bob's free balances

    let timeout = 10000;
    let balance_receiver = (await api.query.system.account(receiver))["data"]["free"];
    while (!balance_receiver.eq(new BN(amount, 10))) {
        console.log('Receiver\'s balance: ' + balance_receiver.toHuman());
        console.log('Not matching: ' + amount);

        await new Promise(r => setTimeout(r, 1000));
        timeout -= 1000;
        balance_receiver = (await api.query.system.account(receiver))["data"]["free"];
        if (timeout <= 0) return ReturnCode.WrongTeleportReceived;
    }
    console.log('Received balance: ' + balance_receiver.toHuman());

    // Teleport to Alice's remote origin on relay chain
    const dest = {
        V3: {
            parents: '1',
            interior: {
                Here: '',
            },
        },
    };
    const beneficiary = {
        V3: {
            parents: '0',
            interior: {
                X1: {
                    AccountId32: {
                      network: null,
                      id: ALICE_REMOTE_ORIGIN,
                    },
                }
            },
        },
    };
    const assets = {
        V3: [
            {
                id: {
                    Concrete: {
                        parents: 1,
                        interior: {
                            Here: '',
                        },
                    },
                },
                fun: {
                  Fungible: amount,
                },
            },
        ],
    };

    const fee_asset_item = '0';
    const weight_limit = 'Unlimited';
    // Create an extrinsic, transferring 1 token unit to Bob.
    const teleport = await api.tx.xcmPallet.teleportAssets(dest, beneficiary, assets, fee_asset_item);

    // We sign and submit the extrinsic. We need to surround the execution of the api call
    // around a Promise to block the test until the transaction gets finalized, thus
    // preventing the main function to return before this event happens
    let transaction_success_event = false;
    await new Promise( async (resolve, reject) => {

      const unsub = await teleport.signAndSend(alice, ({ events = [], status }) => {
        console.log('Transaction status:', status.type);

        if (status.isInBlock) {
          console.log(`Transaction included at blockhash ${status.asInBlock}`);
          console.log('Events:');
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
          return ReturnCode.ExtrinsicUnsuccessful;
        }
      );

    // Get the updated balances
    balance_alice = (await api.query.system.account(ALICE))["data"]["free"];
    console.log('Alice\'s balance after tx: ' + balance_alice.toHuman());

    const instr_withdraw = {
      WithdrawAsset: [
        {
          id: {
              Concrete: {
                  parents: 0,
                  interior: {
                      Here: '',
                  },
              },
          },
          fun: {
            Fungible: amount,
          },
        }
      ],
    };
    const instr_buy_execution = {
      BuyExecution:
        {
          fees: {
            id: {
              Concrete: {
                parents: 0,
                interior: {
                    Here: '',
                },
              },
            },
            fun: {
              Fungible: amount,
            },
          },
          weightLimit: { Unlimited: null },
        }
    };
    const instr_transact = {
      Transact: {
        originType: { SovereignAccount: null },
        requireWeightAtMost: '100000000000',
        call: RISC0_VERIFY_CALL,
      }
    };
    const remote_proof_verification = {
      V2: [instr_withdraw, instr_buy_execution, instr_transact]  
    };
    const xcm_transact = await api.tx.xcmPallet.send(dest, remote_proof_verification);

    console.log("######## Sending xcm transact")
    await new Promise( async (resolve, reject) => {
      const unsub = await xcm_transact.signAndSend(alice, ({ events = [], status }) => {
        console.log('Transaction status:', status.type);

        if (status.isInBlock) {
          console.log(`Transaction included at blockhash ${status.asInBlock}`);
          console.log('Events:');
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
          return ReturnCode.ExtrinsicUnsuccessful;
        }
      );

    return ReturnCode.Ok;
}

module.exports = { run }
