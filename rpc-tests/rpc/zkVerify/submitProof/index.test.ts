import 'dotenv/config';
import { ApiPromise, WsProvider, SubmittableResult } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import { describe, test, expect, beforeAll, afterAll } from '@jest/globals';
import { proofs } from '../proofs';
import { createApi, handleEvents, waitForAttestationId, waitForNewAttestation, waitForNodeToSync } from '../utils';

const requiredEnvVariables: string[] = ['WEBSOCKET', 'PRIVATE_KEY'];

requiredEnvVariables.forEach((envVar) => {
    if (!process.env[envVar]) {
        throw new Error(`Required environment variable ${envVar} is not set.`);
    }
});

describe('Proof Submission and Event Handling', () => {
    let api: ApiPromise;
    let provider: WsProvider;
    let startTime: number;

    beforeAll(async () => {
        provider = new WsProvider(process.env.WEBSOCKET as string);
        api = await createApi(provider);
        await waitForNodeToSync(api);
    });

    afterAll(async () => {
        if (provider) {
            await provider.disconnect();
        }
        if (api) {
            await api.disconnect();
        }
    });

    Object.entries(proofs).forEach(([proofType, { pallet, validProof, invalidProof }]) => {
        test(`should successfully accept a ${proofType} proof, emit a NewAttestation event`, async () => {
            startTime = Date.now();
            const keyring = new Keyring({ type: 'sr25519' });
            const account = keyring.addFromUri(process.env.PRIVATE_KEY as string);

            const submitProof = api.tx[pallet].submitProof(validProof);

            let proof_leaf: string | null = null;
            let attestation_id: string | null = null;

            await new Promise<void>((resolve, reject) => {
                submitProof.signAndSend(account, async ({ events, status, dispatchError }: SubmittableResult) => {
                    if (status.isInBlock) {
                        console.log(`Transaction included in block (elapsed time: ${(Date.now() - startTime) / 1000} seconds)`);
                        handleEvents(events, (data: any[]) => {
                            proof_leaf = data[0].toString();
                            attestation_id = data[1].toString();
                            console.log(`Proof Verified:\n  - Attestation Id: ${attestation_id}\n  - Proof Leaf: ${proof_leaf}`);
                        });

                        if (dispatchError) {
                            console.error(`Invalid ${proofType} transaction:`, dispatchError.toString());
                            reject(dispatchError);
                        } else {
                            resolve();
                        }
                    }

                    if (status.isFinalized) {
                        console.log(`Block containing ${proofType} proof transaction finalized (elapsed time: ${(Date.now() - startTime) / 1000} seconds)`);
                    }
                });
            });

            await waitForAttestationId(attestation_id);

            console.log(`Waiting for NewAttestation event...`);
            const eventData = await waitForNewAttestation(api, 360000, attestation_id, startTime);
            const [attestationId, proofsAttestation] = eventData;
            expect(Number.isInteger(attestationId)).toBeTruthy();
            expect(proofsAttestation).toMatch(/^0x[a-fA-F0-9]{64}$/);
        }, 300000);

        test(`should reject invalid ${proofType} proof upon finalization`, async () => {
            const keyring = new Keyring({ type: 'sr25519' });
            const account = keyring.addFromUri(process.env.PRIVATE_KEY as string);

            const submitProof = api.tx[pallet].submitProof(invalidProof);

            const result = await new Promise<boolean>((resolve, reject) => {
                submitProof.signAndSend(account, ({ status, dispatchError }: SubmittableResult) => {
                    if (status.isInBlock) {
                        console.log(`Transaction included at blockHash ${status.asInBlock.toString()}`);
                    }
                    if (status.isFinalized) {
                        if (dispatchError) {
                            console.error('Transaction failed as expected due to an error.');
                            resolve(true);
                        } else {
                            console.error('Transaction finalized without error, test failed.');
                            reject(new Error('Transaction was expected to fail but did not.'));
                        }
                    }
                }).catch(error => {
                    console.error(`Transaction submission failed with an error: ${error}`);
                    reject(error);
                });
            });

            expect(result).toBeTruthy();
        }, 300000);
    });
});
