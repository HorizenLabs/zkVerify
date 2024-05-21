require('dotenv').config();
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');

const proofs = {
    fflonk: {
        proofHashEnv: 'FFLONK_PROOF',
        pallet: 'settlementFFlonkPallet',
        invalidProofHashEnv: 'INVALID_FFLONK_PROOF'
    },
    boojum: {
        proofHashEnv: 'BOOJUM_PROOF',
        pallet: 'settlementZksyncPallet',
        invalidProofHashEnv: 'INVALID_BOOJUM_PROOF'
    }
};

const requiredEnvVariables = ['WEBSOCKET', 'PRIVATE_KEY'];
Object.values(proofs).forEach(({ proofHashEnv, invalidProofHashEnv }) => {
    requiredEnvVariables.push(proofHashEnv, invalidProofHashEnv);
});

requiredEnvVariables.forEach(envVar => {
    if (!process.env[envVar]) {
        throw new Error(`Required environment variable ${envVar} is not set.`);
    }
});

describe('Proof Submission and Events', () => {
    let api;
    let provider;
    let account;

    beforeAll(async () => {
        provider = new WsProvider(process.env.WEBSOCKET);
        api = await ApiPromise.create({ provider });

        const keyring = new Keyring({ type: 'sr25519' });
        account = keyring.addFromUri(process.env.PRIVATE_KEY);
    });

    afterAll(async () => {
        await api.disconnect();
    });

    async function submitAndLogTransaction(submitProof, proofType) {
        return new Promise((resolve, reject) => {
            submitProof.signAndSend(account, ({ status, dispatchError }) => {
                if (status.isInBlock) {
                    console.log(`${proofType} transaction ${submitProof.hash.toString()} included at blockHash ${status.asInBlock.toString()}`);
                }
                if (status.isFinalized) {
                    if (dispatchError) {
                        console.log(`Invalid ${proofType} transaction failed as expected.`);
                        resolve(true);
                    } else {
                        console.log(`${proofType} transaction finalized successfully.`);
                        resolve(false);
                    }
                }
            }).catch(error => {
                console.error(`${proofType} transaction submission failed with an error: ${error}`);
                reject(error);
            });
        });
    }

    Object.entries(proofs).forEach(([proofType, { proofHashEnv, pallet, invalidProofHashEnv }]) => {
        test(`should successfully accept a ${proofType} proof and emit a NewAttestationEvent event`, async () => {
            const proofHash = process.env[proofHashEnv];
            const submitProof = api.tx[pallet].submitProof(proofHash);

            const result = await submitAndLogTransaction(submitProof, proofType);
            expect(result).toBe(false);

            console.log(`Listening for ${proofType} NewAttestation event...`);
            const eventData = await listenForNewAttestationEvent(api);
            console.log(`${proofType} NewAttestation event received: ${eventData.toString()}`);

            const [attestationId, proofsAttestation] = eventData;
            expect(Number.isInteger(attestationId.toNumber())).toBeTruthy();
            expect(proofsAttestation.toString()).toMatch(/^0x[a-fA-F0-9]{64}$/);
        }, 300000);

        test(`should reject invalid ${proofType} proof upon finalization`, async () => {
            const invalidProofHash = process.env[invalidProofHashEnv];
            const submitProof = api.tx[pallet].submitProof(invalidProofHash);

            const result = await submitAndLogTransaction(submitProof, proofType);
            expect(result).toBe(true);
        }, 300000);
    });
});

async function listenForNewAttestationEvent(api) {
    return new Promise((resolve, reject) => {
        api.derive.chain.subscribeFinalizedHeads(async header => {
            const events = await api.query.system.events.at(header.hash);
            const attestationEvents = events.filter(({ event }) => api.events.poe.NewAttestation.is(event));
            if (attestationEvents.length > 0) {
                resolve(attestationEvents[0].event.data);
            }
        });
    });
}
