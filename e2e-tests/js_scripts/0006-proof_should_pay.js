// This script is used to test the proof verification pay a fee both if it's ok or fails.

const Keccak256 = require('keccak256')

// Hardcoded proof hashes
const VALID_PROOF = "0x283e3f25323d02dabdb94a897dc2697a3b930d8781381ec574af89a201a91d5a2c2808c59f5c736ff728eedfea58effc2443722e78b2eb4e6759a278e9246d600f9c56dc88e043ce0b90c402e96b1f4b1a246f4d0d69a4c340bc910e1f2fd80519e465e01bd7629f175931feed102cb6459a1be7b08018b93c142e961d0352d80b8e5d340df28c2f454c5a2535ca01a230bb945ee24b1171481a9a2c6496fed61cf8878e40adb52dc27da5e79718f118467319d15d64fed460d69d951376ac631a6c44faaec76e296b43fe720d700a63fd530f9064878b5f72f2ffe7458c2f031ac6ed8c1e0758dfb3702ed29bbc0c14b5e727c164b3ade07b9f164af0be54b0143b1a6534b2dcf2bd660e1b5b420d86c0c350fd9d614b639c5df98009f1375e141259679021d0a6a3aa3aae2516bace4a4a651265217ec0ea7c0d7f89b987100abcc93d98ff40bae16eff6c29955f7a37155bb25672b12eb5074dcb7c3e2b001718a257cca21ee593d1ba9f8e91e5168aed8e0b1893e11a6b583d975e747f8008a8c2150a04d8f867945ca1740dc3fc3b2fc4daff61b4725fb294435a1b90101803690ae70fc212b7e929de9a22a4642ef4772546cf93ffd1b1196a3d9113a3009c506755578932ca3630508ca1ed6ee83df5ec9e26cb0b5800a70967a1a93a04d142b6a532935a31d84f75d16929df6d38c3a210ac4f435a8024dfb7e6c1f3246d58038a943f237325b44f03d106e523adfec4324615a2dd09e1e5b9143b411c1cf09ee411cf9864d30df4904099920cee9ae8134d45dfeb29e46115d2e740098674b8fc2ca31fac6fcc9302860654fdc1b522b7e064b0759bc5924f332fa921121b5af880f83fbce02f19dabb8f684593e7322fb80bfc0d054797b1d4eff411b01bf68f81f2032ae4f7fc514bd76ca1b264f3989a92e6b3d74cda4f8a714920e4c02f5a71082a8bcf5be0b5750a244bd040a776ec541dfc2c8ae73180e9240ada5414d66387211eec80d7d9d48498efa1e646d64bb1bf8775b3796a9fd0bf0fdf8244018ce57b018c093e2f75ed77d8dbdb1a7b60a2da671de2efe5f6b9d70d69b94acdfaca5bacc248a60b35b925a2374644ce0c1205db68228c8921d9d9"
const INVALID_PROOF = "0x283e3f25323d02dabdb94a897dc2697a3b930d8781381ec574af89a201a91d5a2c2808c59f5c736ff728eedfea58effc2443722e78b2eb4e6759a278e9246d600f9c56dc88e043ce0b90c402e96b1f4b1a246f4d0d69a4c340bc910e1f2fd80519e465e01bd7629f175931feed102cb6459a1be7b08018b93c142e961d0352d80b8e5d340df28c2f454c5a2535ca01a230bb945ee24b1171481a9a2c6496fed61cf8878e40adb52dc27da5e79718f118467319d15d64fed460d69d951376ac631a6c44faaec76e296b43fe720d700a63fd530f9064878b5f72f2ffe7458c2f031ac6ed8c1e0758dfb3702ed29bbc0c14b5e727c164b3ade07b9f164af0be54b0143b1a6534b2dcf2bd660e1b5b420d86c0c350fd9d614b639c5df98009f1375e141259679021d0a6a3aa3aae2516bace4a4a651265217ec0ea7c0d7f89b987100abcc93d98ff40bae16eff6c29955f7a37155bb25672b12eb5074dcb7c3e2b001718a257cca21ee593d1ba9f8e91e5168aed8e0b1893e11a6b583d975e747f8008a8c2150a04d8f867945ca1740dc3fc3b2fc4daff61b4725fb294435a1b90101803690ae70fc212b7e929de9a22a4642ef4772546cf93ffd1b1196a3d9113a3009c506755578932ca3630508ca1ed6ee83df5ec9e26cb0b5800a70967a1a93a04d142b6a532935a31d84f75d16929df6d38c3a210ac4f435a8024dfb7e6c1f3246d58038a943f237325b44f03d106e523adfec4324615a2dd09e1e5b9143b411c1cf09ee411cf9864d30df4904099920cee9ae8134d45dfeb29e46115d2e740098674b8fc2ca31fac6fcc9302860654fdc1b522b7e064b0759bc5924f332fa921121b5af880f83fbce02f19dabb8f684593e7322fb80bfc0d054797b1d4eff411b01bf68f81f2032ae4f7fc514bd76ca1b264f3989a92e6b3d74cda4f8a714920e4c02f5a71082a8bcf5be0b5750a244bd040a776ec541dfc2c8ae73180e9240ada5414d66387211eec80d7d9d48498efa1e646d64bb1bf8775b3796a9fd0bf0fdf8244018ce57b018c093e2f75ed77d8dbdb1a7b60a2da671de2efe5f6b9d70d69b94acdfaca5bacc248a60b35b925a2374644ce0000000000000000000000"

const MIN_PRICE = 1000000000;

const BlockUntil = {
    InBlock: 'InBlock',
    Finalized: 'Finalized',
};

const ReturnCode = {
    Ok: 1,
    ErrProofVerificationFailed: 2,
    ErrFalseProofVerified: 3,
    ErrValidProofNotPay: 4,
    ErrInvalidProofNotPay: 5,
};

const BLOCK_TIME = 6000; // block time in milliseconds

let api;

async function run(nodeName, networkInfo, args) {
    const { wsUri, userDefinedTypes } = networkInfo.nodesByName[nodeName];
    api = await zombie.connect(wsUri, userDefinedTypes);
    await api.isReady;

    // Create a keyring instance
    const keyring = new zombie.Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');

    let balanceAlice = await getBalance(alice);
    console.log('Alice\'s balance: ' + balanceAlice.toHuman());

    const validProofSubmission = api.tx.settlementFFlonkPallet.submitProof(VALID_PROOF);
    if (await submitAnExtrinsic(validProofSubmission, alice, BlockUntil.InBlock) == -1) {
        return ReturnCode.ErrProofVerificationFailed;
    };

    let newBalanceAlice = await getBalance(alice);
    console.log('Alice\'s balance after valid proof: ' + newBalanceAlice.toHuman());

    if (balanceAlice - newBalanceAlice <= MIN_PRICE) {
        return ReturnCode.ErrValidProofNotPay;
    }

    balanceAlice = newBalanceAlice;

    const invalidProofSubmission = api.tx.settlementFFlonkPallet.submitProof(INVALID_PROOF);

    if (await submitAnExtrinsic(invalidProofSubmission, alice, BlockUntil.InBlock) !== -1) {
        return ReturnCode.ErrFalseProofVerified;
    };

    newBalanceAlice = await getBalance(alice);
    console.log('Alice\'s balance after invalid proof: ' + newBalanceAlice.toHuman());

    if (balanceAlice - newBalanceAlice <= MIN_PRICE) {
        return ReturnCode.ErrInvalidProofNotPay;
    }

    // Any return value different from 1 is considered an error
    return ReturnCode.Ok;
}

async function getBalance(user) {
    return (await api.query.system.account(user.address))["data"]["free"]
}

async function submitAnExtrinsic(extrinsic, signer, blockUntil) {
    let transactionSuccessEvent = false;

    let retVal = await new Promise(async (resolve, reject) => {
        let proofHash;
        const unsub = await extrinsic.signAndSend(signer, ({ events = [], status }) => {
            console.log('Transaction status:', status.type);

            if (status.isInBlock) {
                console.log(`Transaction included at blockhash ${status.asInBlock}`);
                console.log('Events:');

                events.forEach(({ event: { data, method, section }, phase }) => {
                    console.log('\t', phase.toString(), `: ${section}.${method}`, data.toString());
                    if (section == "system" && method == "ExtrinsicSuccess") {
                        transactionSuccessEvent = true;
                    }
                    if (section == "poe" && method == "NewElement") {
                        proofHash = data[0].toString();
                    }
                });
                if (blockUntil === BlockUntil.InBlock) {
                    unsub();
                    if (transactionSuccessEvent) {
                        resolve(proofHash);
                    } else {
                        reject("ExtrinsicSuccess has not been seen");
                    }
                }
            }

            else if (status.isFinalized) {
                console.log(`Transaction finalized at blockhash ${status.asFinalized}`);
                if (blockUntil === BlockUntil.Finalized) {
                    unsub();
                    if (transactionSuccessEvent) {
                        resolve(proofHash);
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
            (proofHash) => {
                console.log("Transaction successfully processed: " + proofHash)
                return proofHash;
            },
            error => {
                return -1;
            }
        );

    return retVal;
}

module.exports = { run }

