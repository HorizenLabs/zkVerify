// This script is used to test the proofPath RPC call.
// It also shows how to properly register custom data types and RPC calls
// to Polkadot.js, in order to use its interface to interact with the blockchain.
// Finally, it also demonstrate:
// - how to submit and extrinisic and wait for its inclusion ina block
// - how to wait for a specific event to be emitted
// Both operations are performed through the use of polkadot.js observer pattern
// and promise-based async/await syntax.

const Keccak256 = require('keccak256')

// Hardcoded proof hashes
const proof_01 = "0x283e3f25323d02dabdb94a897dc2697a3b930d8781381ec574af89a201a91d5a2c2808c59f5c736ff728eedfea58effc2443722e78b2eb4e6759a278e9246d600f9c56dc88e043ce0b90c402e96b1f4b1a246f4d0d69a4c340bc910e1f2fd80519e465e01bd7629f175931feed102cb6459a1be7b08018b93c142e961d0352d80b8e5d340df28c2f454c5a2535ca01a230bb945ee24b1171481a9a2c6496fed61cf8878e40adb52dc27da5e79718f118467319d15d64fed460d69d951376ac631a6c44faaec76e296b43fe720d700a63fd530f9064878b5f72f2ffe7458c2f031ac6ed8c1e0758dfb3702ed29bbc0c14b5e727c164b3ade07b9f164af0be54b0143b1a6534b2dcf2bd660e1b5b420d86c0c350fd9d614b639c5df98009f1375e141259679021d0a6a3aa3aae2516bace4a4a651265217ec0ea7c0d7f89b987100abcc93d98ff40bae16eff6c29955f7a37155bb25672b12eb5074dcb7c3e2b001718a257cca21ee593d1ba9f8e91e5168aed8e0b1893e11a6b583d975e747f8008a8c2150a04d8f867945ca1740dc3fc3b2fc4daff61b4725fb294435a1b90101803690ae70fc212b7e929de9a22a4642ef4772546cf93ffd1b1196a3d9113a3009c506755578932ca3630508ca1ed6ee83df5ec9e26cb0b5800a70967a1a93a04d142b6a532935a31d84f75d16929df6d38c3a210ac4f435a8024dfb7e6c1f3246d58038a943f237325b44f03d106e523adfec4324615a2dd09e1e5b9143b411c1cf09ee411cf9864d30df4904099920cee9ae8134d45dfeb29e46115d2e740098674b8fc2ca31fac6fcc9302860654fdc1b522b7e064b0759bc5924f332fa921121b5af880f83fbce02f19dabb8f684593e7322fb80bfc0d054797b1d4eff411b01bf68f81f2032ae4f7fc514bd76ca1b264f3989a92e6b3d74cda4f8a714920e4c02f5a71082a8bcf5be0b5750a244bd040a776ec541dfc2c8ae73180e9240ada5414d66387211eec80d7d9d48498efa1e646d64bb1bf8775b3796a9fd0bf0fdf8244018ce57b018c093e2f75ed77d8dbdb1a7b60a2da671de2efe5f6b9d70d69b94acdfaca5bacc248a60b35b925a2374644ce0c1205db68228c8921d9d9"
const proof_02 = "0x2ecc31435ec6d6963d463c38ea5662d9c94a67e441e7bc611598ebcc59f571880768291fd5d95fcf02bce7e4fde1f048b843bbffab1f242904e82d443a4ebb61150c3a4afdbb62d034320da390e3585a30ba13f4df73798b78e5a75655d3350d19fca02cc5838405f9ae4177ac7117971af2cb5006d7a46436f644410d6e7c52099f803c0f18d4b44fe22f3100d1fc80ccb7309fa7168f51bc64f3fc0f1dbd240b0573f3593238e56b23e75246d9d0f6f6a5cf824700667e3482ca9fecf74cdc0b308e6a8f69dccb9ca00d540543441f7030928da766406a152427bdf31b44051b6b5198b34006f9ac34c6c857e450cc11f5c6b6d21119fe283738581c0ad8bd0f8cbdbc574f64da884f6a02e00669f3eef10138266f3d7fa278aef1b1c60171005c0c2b8b2429c5003c5ab24af44cb1ab81cdc96dcaf6004a0f74406bb10f45233b13015cef8c40c491a7770efd0a8d8a64186d4f3827e74972bfc25b11f1f002550a5e253c923c5783026c7439601595477f1a212de449c64a8ae5e2fc0313127bf9cd5146217e531196ce65ccef3249375450d6932151f923c39e6a73588223d90f5bf230eee5a6cb6463f161602cc37fe538e2954ebef695b926b76e3fae299c60c1952aa4b1f246204ac7c22c0156ede30aeb73444ee40d69c0f131fa471fdca090abfd38541c88ee73624657a695155748643f7834b80d1c0481079e670033817252b24575a4e6007f08f37c34462d5e9fd50b1e83ec8cfc86149400d519e224ee11831ac393e3a09730be6f385ae5c9e14446fde5069fea751fb6b48211c85268b8017de7981eb1bd78526bc20d5f863ad3abe249728ca7b75b2146c1254465d6100a911213d95f800779e74f6701b1dfa0b6660642108fd2c7cd2f131d9163eeebe9d8aabdf8d37fde4451f762be478d117688e0a6ed2648dbe025e82a4b13ee629a73d1efa6f269747506058746aa589bb961c1385bb2b30e0086f010ef87535f2137a04f19fe5aa7c4f348c32ce6f5b0b45bb503895673a8a51d7f1b0228693fbfb38be718b04c9fdf116a97d7f30e670db84d21bb0d12fc57645415950a3fab52ee1557ac7b895deeca2eb27bacfc3b9e26a39b1875149680611d"

// Custom types and RPC calls
// This one defines the metadata for the return value of proofPath RPC call
newTypes = {
    MerkleProof: {
        root: 'H256',
        proof: 'Vec<H256>',
        number_of_leaves: 'u32',
        leaf_index: 'u32',
        leaf: 'H256',
    }
};

const BlockUntil = {
    InBlock: 'InBlock',
    Finalized: 'Finalized',
};

const ReturnCode = {
    Ok: 1,
    ErrProofVerificationFailed: 2,
    ErrNoAttestation: 3,
    ErrAttProofVerificationFailed: 4,
    ErrWrongAttestationTiming: 5,
};

const BLOCK_TIME = 6000; // block time in milliseconds

// This one defines the metadata for the arguments and return value of proofPath RPC call
userDefinedRpc = {
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


async function run(nodeName, networkInfo, args) {
    const {wsUri, userDefinedTypes} = networkInfo.nodesByName[nodeName];
    const provider = new zombie.WsProvider(wsUri);

    // Passing user defined types and RPC calls, instead of userDefinedTypes
    // Eventually, it is possible to merge the two objects, but this is not
    // mandatory for this test
    const api = new zombie.ApiPromise({ provider, types: newTypes, rpc: userDefinedRpc });
    await api.isReady;

    // Create a keyring instance
    const keyring = new zombie.Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');

    // Create the proof submission extrinsics...
    let proof_hashes_array = [];
    const proof_1_submission = api.tx.settlementFFlonkPallet.submitProof(proof_01);
    const proof_2_submission = api.tx.settlementFFlonkPallet.submitProof(proof_02);
    // ...and submit them
    const hash01 = await submit_an_extrinsic(proof_1_submission, alice, BlockUntil.InBlock);
    const proof_included_timestamp = Date.now();
    const hash02 = await submit_an_extrinsic(proof_2_submission, alice, BlockUntil.InBlock);

    // Save the proof hash only if the extrinsic has been successfully included in a block
    if (hash01 == -1 || hash02 == -1) {
        return ReturnCode.ErrProofVerificationFailed;
    }
    proof_hashes_array.push(hash01);
    proof_hashes_array.push(hash02);

    // Wait for the next attestation ID to be emitted
    const expected_att_timeout = BLOCK_TIME * 10;
    const expected_att_timeout_delta = BLOCK_TIME * 2;
    const interesting_att_id = await wait_for_new_attestation(api, expected_att_timeout * 2);
    const att_timestamp = Date.now();
    if (interesting_att_id == -1) {
        console.log("Something went wrong while waiting for a new attestation");
        return ReturnCode.ErrNoAttestation;
    } else {
        var published_root = interesting_att_id.data[1];
        console.log("A new attestation has been published: ");
        interesting_att_id.data.forEach((data) => {
            console.log(`\t\t\t${data.toString()}`);
        });
    }

    // Check that the attestation was received in the expected time window
    if (att_timestamp < proof_included_timestamp + expected_att_timeout ||
        att_timestamp > proof_included_timestamp + (expected_att_timeout + expected_att_timeout_delta)) {
        console.log("Attestation not received in the expected time window");
        return ReturnCode.ErrWrongAttestationTiming;
    }

    // For each proof, get its Merkle path and evaluate the root
    const att_id = parseInt(interesting_att_id.data['id']);
    const poe_res01 = await api.rpc.poe.proofPath(att_id, proof_hashes_array[0]);
    const poe_res02 = await api.rpc.poe.proofPath(att_id, proof_hashes_array[1]);

    console.log('##### proofPath RPC returned (proof 1): ' + JSON.stringify(poe_res01));
    console.log('##### proofPath RPC returned (proof 2): ' + JSON.stringify(poe_res02));

    // Reconstruct the root from the returned proof
    const proof_01_verification = await verify_proof(poe_res01, published_root);
    console.log("Proof 01 verification: " + proof_01_verification);
    if (!proof_01_verification) {
        return ReturnCode.ErrAttProofFailedVerification;
    }

    const proof_02_verification = await verify_proof(poe_res02, published_root);
    console.log("Proof 02 verification: " + proof_02_verification);
    if (!proof_02_verification) {
        return ReturnCode.ErrAttProofFailedVerification;
    }

    // Any return value different from 1 is considered an error
    return ReturnCode.Ok;
}

async function submit_an_extrinsic(extrinsic, signer, block_until) {
    let transaction_success_event = false;

    let ret_val = await new Promise( async (resolve, reject) => {
        let proof_hash;
        const unsub = await extrinsic.signAndSend(signer, ({ events = [], status }) => {
            console.log('Transaction status:', status.type);

            if (status.isInBlock) {
                console.log(`Transaction included at blockhash ${status.asInBlock}`);
                console.log('Events:');

                events.forEach(({ event: { data, method, section }, phase }) => {
                    console.log('\t', phase.toString(), `: ${section}.${method}`, data.toString());
                    if (section == "system" && method == "ExtrinsicSuccess") {
                        transaction_success_event = true;
                    }
                    if (section == "poe" && method == "NewElement") {
                        proof_hash = data[0].toString();
                    }
                });
                if (block_until === BlockUntil.InBlock) {
                    unsub();
                    if (transaction_success_event) {
                        resolve(proof_hash);
                    } else {
                        reject("ExtrinsicSuccess has not been seen");
                    }
                }
            }

            else if (status.isFinalized) {
                console.log(`Transaction finalized at blockhash ${status.asFinalized}`);
                if (block_until === BlockUntil.Finalized) {
                    unsub();
                    if (transaction_success_event) {
                        resolve(proof_hash);
                    } else {
                        reject("ExtrinsicSuccess has not been seen");
                    }
                }
            }

            else if (status.isError) {
                unsub();
                reject("Transaction status.isError");
            }
        });
    })
    .then(
        (proof_hash) => {
            console.log("Transaction successfully processed: " + proof_hash)
            return proof_hash;
        },
        error => {
            return -1;
        }
    );

    return ret_val;
}

// Wait for the next attestaion id to be published
async function wait_for_new_attestation(api, timeout) {

    const ret_val = await new Promise( async (resolve, reject) => {
        // Subscribe to system events via storage
        timeout = setTimeout(function() { unsubscribe(); reject("Timeout expired"); }, timeout);
        const unsubscribe = await api.query.system.events((events) => {
            console.log(`\nReceived ${events.length} events:`);

            // Loop through the Vec<EventRecord>
            events.forEach((record) => {
                // Extract the phase, event and the event types
                const { event, phase } = record;
                const types = event.typeDef;

                // Show what we are busy with
                console.log(`\t${event.section}:${event.method}:: (phase=${phase.toString()})`);

                if ((event.section == "poe") && (event.method == "NewAttestation")) {
                    clearTimeout(timeout);
                    unsubscribe();
                    resolve(event);
                }

                // Loop through each of the parameters, displaying the type and data
                event.data.forEach((data, index) => {
                    console.log(`\t\t\t${types[index].type}: ${data.toString()}`);
                });
            });
        });
    }).then(
        (our_best_event) => {
            console.log("A new attestation has been published")
            return our_best_event;
        },
        error => {
            console.log("An error happened when waiting for the new attestation to be published.")
            return -1;
        }
    );

    return ret_val;
}

function stripHexPrefix(input_str) {
    return input_str.toString().replace(/^0x/, '');
}

function verify_proof(proof, published_root) {
    let position = proof['leaf_index'];
    let width = proof['number_of_leaves'];
    let hash = Keccak256(proof['leaf'].toString('hex')).toString('hex');
    proof['proof'].forEach(function (p) {
        p = stripHexPrefix(p);
        if (position % 2 == 1 || position + 1 == width) {
            hash = Keccak256('0x' + p + hash).toString('hex');
        } else {
            hash = Keccak256('0x' + hash + p).toString('hex');
        }
        position = Math.floor(position / 2);
        width = Math.floor((width - 1) / 2) + 1;
    });

    return stripHexPrefix(published_root) == hash;
}

module.exports = { run }

