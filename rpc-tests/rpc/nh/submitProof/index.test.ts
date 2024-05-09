require('dotenv').config();
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');

const requiredEnvVariables = ['FFLONK_PROOF', 'WEBSOCKET', 'PRIVATE_KEY'];
requiredEnvVariables.forEach(envVar => {
    if (!process.env[envVar]) {
        throw new Error(`Required environment variable ${envVar} is not set.`);
    }
});

const wsProvider = new WsProvider(process.env.WS_URL);

const contractABI = [
    {
        "constant": true,
        "inputs": [],
        "name": "latestAttestationId",
        "outputs": [
            {
                "name": "",
                "type": "uint256"
            }
        ],
        "payable": false,
        "stateMutability": "view",
        "type": "function"
    }
];

describe('Proof Submission and Event Handling', () => {
    let api;
    let keyring;
    let userAccount;

    beforeAll(async () => {
        api = await ApiPromise.create({ provider: wsProvider });
        keyring = new Keyring({ type: 'sr25519' });
        userAccount = keyring.addFromUri(process.env.PRIVATE_KEY);
    });

    afterAll(async () => {
        await wsProvider.disconnect();
    });

    test('should successfully accept a Fflonk proof and emit a NewAttestationEvent event', async () => {
        const proofHash = process.env.FFLONK_PROOF;
        console.log('Submitting proof...');
        
        // Send Tx
        const submitProof = api.tx.settlementFFlonkPallet.submitProof(proofHash);
        await submitProof.signAndSend(userAccount);
        console.log('Proof submitted successfully.');

        // Listen for new blocks and check for the NewAttestation event
        console.log('Listening for NewAttestation event...');
        const eventData = await listenForNewAttestationEvent(api);
        console.log('NewAttestation event received:', eventData.toString());
        
        // Check event data
        const [attestationId, proofsAttestation] = eventData;
        expect(Number.isInteger(attestationId.toNumber())).toBeTruthy();
        expect(proofsAttestation.toString()).toMatch(/^0x[a-fA-F0-9]{64}$/);
    }, 300000);

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
});
