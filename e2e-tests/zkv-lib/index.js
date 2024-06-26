// Custom types and RPC calls
// This one defines the metadata for the return value of proofPath RPC call
zkvTypes = {
    MerkleProof: {
        root: 'H256',
        proof: 'Vec<H256>',
        number_of_leaves: 'u32',
        leaf_index: 'u32',
        leaf: 'H256',
    },
};

// This one defines the metadata for the arguments and return value of proofPath RPC call
zkvRpc = {
    poe: {
        proofPath: {
            description: 'Get the Merkle root and path of a stored proof',
            params: [
                {
                    name: 'root_id',
                    type: 'u64'
                },
                {
                    name: 'proof_hash',
                    type: 'H256'
                },
                {
                    name: 'at',
                    type: 'BlockHash',
                    isOptional: true
                }
            ],
            type: 'MerkleProof'
        }
    }
};

BlockUntil = {
    InBlock: 'InBlock',

    Finalized: 'Finalized',
};

exports.BlockUntil = BlockUntil;

let api = null;

exports.BLOCK_TIME = 6000; // block time in milliseconds
exports.init_api = async (zombie, nodeName, networkInfo) => {
    if (api === null) {
        const { wsUri } = networkInfo.nodesByName[nodeName];
        const provider = new zombie.WsProvider(wsUri);
        api = new zombie.ApiPromise({ provider, types: zkvTypes, rpc: zkvRpc });
        await api.isReady;
    }
    return api;
}

exports.submitProof = async (pallet, signer, ...verifierArgs) => {
    const validProofSubmission = pallet.submitProof(...verifierArgs);
    return await submitExtrinsic(validProofSubmission, signer, BlockUntil.InBlock, (event) => event.section == "poe" && event.method == "NewElement");
}

// Wait for the next attestaion id to be published
exports.waitForNewAttestation = async (api, timeout) => {

    const retVal = await new Promise(async (resolve, reject) => {
        // Subscribe to system events via storage
        timeout = setTimeout(function () { unsubscribe(); reject("Timeout expired"); }, timeout);
        const unsubscribe = await api.query.system.events((events) => {
            console.log(`\nReceived ${events.length} events: `);

            // Loop through the Vec<EventRecord>
            events.forEach((record) => {
                // Extract the phase, event and the event types
                const { event, phase } = record;
                const types = event.typeDef;

                // Show what we are busy with
                console.log(`\t${event.section}: ${event.method}:: (phase = ${phase.toString()})`);

                if ((event.section == "poe") && (event.method == "NewAttestation")) {
                    clearTimeout(timeout);
                    unsubscribe();
                    resolve(event);
                }

                // Loop through each of the parameters, displaying the type and data
                event.data.forEach((data, index) => {
                    console.log(`\t\t\t${types[index].type}: ${data.toString()} `);
                });
            });
        });
    }).then(
        (ourBestEvent) => {
            console.log("A new attestation has been published")
            return ourBestEvent;
        },
        _error => {
            console.log("An error happened when waiting for the new attestation to be published.")
            return -1;
        }
    );

    return retVal;
}

exports.registerVk = async (pallet, signer, vk) => {
    return await submitExtrinsic(pallet.registerVk(vk), signer, BlockUntil.InBlock,
        (event) => event.section == "settlementFFlonkPallet" && event.method == "VkRegistered"
    )
}

exports.submitExtrinsic = async (extrinsic, signer, blockUntil, filter) => {
    return await submitExtrinsic(extrinsic, signer, blockUntil, filter);
}

async function submitExtrinsic(extrinsic, signer, blockUntil, filter) {
    let transactionSuccessEvent = false;
    let done = false;
    if (filter === undefined) {
        console.log("No filtering");
        filter = (_event) => true;
    }

    let retVal = await new Promise(async (resolve, reject) => {
        const unsub = await extrinsic.signAndSend(signer, ({ events: records = [], status }) => {
            if (status.isInBlock) {
                console.log(`Transaction included at blockhash ${status.asInBlock}`);
                records.forEach(({ event: { method, section } }) => {
                    if (section == "system" && method == "ExtrinsicSuccess") {
                        transactionSuccessEvent = true;
                    }
                });
                if (blockUntil === BlockUntil.InBlock) {
                    done = true;
                }
            }
            else if (status.isFinalized) {
                console.log(`Transaction finalized at blockhash ${status.asFinalized}`);
                if (blockUntil === BlockUntil.Finalized) {
                    done = true;
                }
            }
            else if (status.isError) {
                done = true;
                console.log("ERROR: Transaction status.isError");
            }
            if (done) {
                unsub();
                if (transactionSuccessEvent) {
                    resolve(records);
                } else {
                    reject("ExtrinsicSuccess has not been seen");
                }
            }
        });
    }).then(
        (records) => {
            console.log("Transaction successfully processed: " + records)
            return records.map((record) => record.event).filter(filter)
        },
        _error => {
            return -1;
        }
    );

    return retVal;
}

exports.receivedEvents = (events) => {
    return Array.isArray(events) && events.length > 0;
}

exports.getBalance = async (user) => {
    return await getBalance(user);
}

async function getBalance(user) {
    return (await api.query.system.account(user.address))["data"]["free"]
}
