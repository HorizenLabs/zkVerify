const MIN_PRICE = 1000000000;

const ReturnCode = {
    Ok: 1,
    ErrProofVerificationFailed: 2,
    ErrFalseProofVerified: 3,
    ErrValidProofNotPay: 4,
    ErrInvalidProofNotPay: 5,
};

const { init_api, submitProof, getBalance } = require('zkv-lib')
const { VALID_PROOF, VALID_PUBS, VK } = require('./fflonk_data.js');

async function run(nodeName, networkInfo, _args) {
    const api = await init_api(zombie, nodeName, networkInfo);

    // Create a keyring instance
    const keyring = new zombie.Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');

    let balanceAlice = await getBalance(alice);
    console.log('Alice\'s balance: ' + balanceAlice.toHuman());

    if (await submitProof(api.tx.settlementFFlonkPallet, alice, { 'Vk': VK }, VALID_PROOF, VALID_PUBS) == -1) {
        return ReturnCode.ErrProofVerificationFailed;
    };

    let newBalanceAlice = await getBalance(alice);
    console.log('Alice\'s balance after valid proof: ' + newBalanceAlice.toHuman());

    if (balanceAlice - newBalanceAlice <= MIN_PRICE) {
        return ReturnCode.ErrValidProofNotPay;
    }

    balanceAlice = newBalanceAlice;

    if (await submitProof(api.tx.settlementFFlonkPallet, alice, { 'Vk': VK }, VALID_PROOF, 0) !== -1) {
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

module.exports = { run }

