const MIN_PRICE = 1000000000;

const ReturnCode = {
    Ok: 1,
    ErrProofVerificationFailed: 2,
    ErrCannotDisable: 3,
    ErrVerifiedOnDisable: 4,
    ErrValidProofNotPay: 5,
    ErrDisableProofTooCostly: 6,
};

const { init_api, submitProof, getBalance, receivedEvents, submitExtrinsic } = require('zkv-lib')
const { PROOF, PUBS, VK } = require('./fflonk_data.js');

// Call verify on a disable verifier should cost at most FACTOR times the cost of the proof.
const FACTOR = 100;

async function run(nodeName, networkInfo, _args) {
    const api = await init_api(zombie, nodeName, networkInfo);

    // Create a keyring instance
    const keyring = new zombie.Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');

    let balanceAlice = await getBalance(alice);
    console.log('Alice\'s balance: ' + balanceAlice.toHuman());

    if (!receivedEvents(await submitProof(api.tx.settlementFFlonkPallet, alice, { 'Vk': VK }, PROOF, PUBS))) {
        return ReturnCode.ErrProofVerificationFailed;
    };

    let newBalanceAlice = await getBalance(alice);
    console.log('Alice\'s balance after valid proof: ' + newBalanceAlice.toHuman());

    let paidBalanceOnVerify = balanceAlice.sub(newBalanceAlice);

    console.log('Alice paid for a valid proof: ' + paidBalanceOnVerify);

    const disableTx = api.tx.settlementFFlonkPallet.disable(true);
    const sudoDisableTx = api.tx.sudo.sudo(disableTx)

    if (!receivedEvents(await submitExtrinsic(sudoDisableTx, alice, BlockUntil.InBlock))) {
        return ReturnCode.ErrCannotDisable;
    };

    balanceAlice = await getBalance(alice);
    console.log('Alice\'s balance before verify a proof on disabled verifier: ' + balanceAlice.toHuman());

    // This should fails
    if (receivedEvents(await submitProof(api.tx.settlementFFlonkPallet, alice, { 'Vk': VK }, PROOF, PUBS))) {
        return ReturnCode.ErrVerifiedOnDisable;
    };

    newBalanceAlice = await getBalance(alice);

    console.log('Alice\'s balance after verify a proof on disabled verifier: ' + newBalanceAlice.toHuman());

    let paidBalanceOnDisable = balanceAlice.sub(newBalanceAlice);

    console.log('Alice paid for a disable verifier: ' + paidBalanceOnDisable);

    if (paidBalanceOnDisable > paidBalanceOnVerify / FACTOR) {
        console.log(`ERROR: Alice should pay at most ${FACTOR} times what she paid for a valid verify`);
        return ReturnCode.ErrDisableProofTooCostly;
    }
    console.log(`INFO: Alice paid less than ${FACTOR} times`);

    // Any return value different from 1 is considered an error
    return ReturnCode.Ok;
}

module.exports = { run }

