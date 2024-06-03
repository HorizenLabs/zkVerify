import { ApiPromise, WsProvider } from '@polkadot/api';
import { EventRecord } from '@polkadot/types/interfaces';

export async function createApi(provider: WsProvider): Promise<ApiPromise> {
    const timeout = new Promise<never>((_, reject) =>
        setTimeout(() => reject(new Error(`Failed to connect to the WebSocket URL: ${process.env.WEBSOCKET}`)), 5000)
    );
    return await Promise.race([ApiPromise.create({ provider }), timeout]);
}

export function handleEvents(events: EventRecord[], callback: (data: any[]) => void): void {
    events.forEach(({ event: { data, method, section } }) => {
        if (section === 'poe' && method === 'NewElement') {
            callback(data);
        }
    });
}

export async function waitForAttestationId(attestation_id: string | null): Promise<void> {
    while (!attestation_id) {
        console.log("Waiting for attestation_id to be set...");
        await new Promise(resolve => setTimeout(resolve, 1000));
    }
}

export async function waitForNewAttestation(api: ApiPromise, timeoutDuration: number, attestation_id: string | null, startTime: number): Promise<[number, string]> {
    return new Promise(async (resolve, reject) => {
        const timeout = setTimeout(() => {
            reject("Timeout expired");
        }, timeoutDuration);

        const interval = setInterval(() => {
            console.log(`Waiting for NewAttestation event... (elapsed time: ${(Date.now() - startTime) / 1000} seconds)`);
        }, 15000);

        const unsubscribe = await api.query.system.events((events: EventRecord[]) => {
            events.forEach((record) => {
                const { event } = record;
                const types = event.typeDef;

                if (event.section === "poe" && event.method === "NewAttestation") {
                    const currentAttestationId = event.data[0].toString();
                    console.log(`Detected NewAttestation event with id: ${currentAttestationId}`);
                    if (currentAttestationId === attestation_id) {
                        clearTimeout(timeout);
                        clearInterval(interval);
                        unsubscribe();
                        console.log(`Matched NewAttestation event with ProofVerified event Attestation Id ${attestation_id}:`);
                        event.data.forEach((data, index) => {
                            console.log(`\t${types[index].type}: ${data.toString()}`);
                        });
                        resolve([parseInt(event.data[0].toString()), event.data[1].toString()]);
                    }
                }
            });
        }) as unknown as () => void;
    });
}

export async function waitForNodeToSync(api: ApiPromise): Promise<void> {
    let isSyncing = true;
    while (isSyncing) {
        const health = await api.rpc.system.health();
        isSyncing = health.isSyncing.isTrue;
        if (isSyncing) {
            await new Promise(resolve => setTimeout(resolve, 1000));
        }
    }
}
