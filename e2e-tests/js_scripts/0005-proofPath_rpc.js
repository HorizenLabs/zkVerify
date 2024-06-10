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
const PROOF_FFLONK = "0x283e3f25323d02dabdb94a897dc2697a3b930d8781381ec574af89a201a91d5a2c2808c59f5c736ff728eedfea58effc2443722e78b2eb4e6759a278e9246d600f9c56dc88e043ce0b90c402e96b1f4b1a246f4d0d69a4c340bc910e1f2fd80519e465e01bd7629f175931feed102cb6459a1be7b08018b93c142e961d0352d80b8e5d340df28c2f454c5a2535ca01a230bb945ee24b1171481a9a2c6496fed61cf8878e40adb52dc27da5e79718f118467319d15d64fed460d69d951376ac631a6c44faaec76e296b43fe720d700a63fd530f9064878b5f72f2ffe7458c2f031ac6ed8c1e0758dfb3702ed29bbc0c14b5e727c164b3ade07b9f164af0be54b0143b1a6534b2dcf2bd660e1b5b420d86c0c350fd9d614b639c5df98009f1375e141259679021d0a6a3aa3aae2516bace4a4a651265217ec0ea7c0d7f89b987100abcc93d98ff40bae16eff6c29955f7a37155bb25672b12eb5074dcb7c3e2b001718a257cca21ee593d1ba9f8e91e5168aed8e0b1893e11a6b583d975e747f8008a8c2150a04d8f867945ca1740dc3fc3b2fc4daff61b4725fb294435a1b90101803690ae70fc212b7e929de9a22a4642ef4772546cf93ffd1b1196a3d9113a3009c506755578932ca3630508ca1ed6ee83df5ec9e26cb0b5800a70967a1a93a04d142b6a532935a31d84f75d16929df6d38c3a210ac4f435a8024dfb7e6c1f3246d58038a943f237325b44f03d106e523adfec4324615a2dd09e1e5b9143b411c1cf09ee411cf9864d30df4904099920cee9ae8134d45dfeb29e46115d2e740098674b8fc2ca31fac6fcc9302860654fdc1b522b7e064b0759bc5924f332fa921121b5af880f83fbce02f19dabb8f684593e7322fb80bfc0d054797b1d4eff411b01bf68f81f2032ae4f7fc514bd76ca1b264f3989a92e6b3d74cda4f8a714920e4c02f5a71082a8bcf5be0b5750a244bd040a776ec541dfc2c8ae73180e9240ada5414d66387211eec80d7d9d48498efa1e646d64bb1bf8775b3796a9fd0bf0fdf8244018ce57b018c093e2f75ed77d8dbdb1a7b60a2da671de2efe5f6b9d70d69b94acdfaca5bacc248a60b35b925a2374644ce0c1205db68228c8921d9d9"
const PROOF_ZKSYNC = "0x02c6cf2fd56edca1f17f406cceef3de1c99bba6e499ed96ef4f453af011257c420944a838b2cd133a414ae6882fd8cc0dfb7daa14540d796ab937f65479beaca1fb7b349b2a6dc4edfc8191e31ddc0b342840dc575ad213473529611e15261e8020c09be65a4d571cadbb39b0737777c365af77b4702d6e1a4e0340abb1cb8c3221cc01cc33c432ab679319c724544616069b0d6f4df5f537ec36887deead9631fc36d5da22c35d8d83eb74ccc2afa4a83d2d6c604998ac86e653f1307d016200e01dd9bbcfa860fe26eca3f159b473fa073fce20ef5354c25d52e5e9c4bc2930b5ae2e3e19c47907074ef77fc0e113920e9f702ad0f7f1789c696a47849ebcb21db13fcf4fc3cc99f9879514cb5a3ac5b672a4343b915833be0cb9c4281e1810a376c40d30b54d2c82d98e26d93f4d2fa5010ef0973f4c9ddc5eb83074b2fdf011214912fffecc3507d741e4164d049963f4e22dfefc659a2d4122e141f8f8700cf13591e41e00c27c19f05546c874287a483df746fd1c5f66b955f5caf1fc00928a89a4c924f98bd2bb78a704a7879f15799dcf7e94d2f465c33b65358519606f57ff3f11aee64bdffac49821dda7e029a281519e0f6a44302bd822d69e08d1797df980a6a223e0b455ad79df6ee836ac09486e3c4ce28ee870249e5d1db8f1bf81479df3717fee0f378da47910f1177685a7de078eb5dc2ae65d1ff321cdf2b3c88144fd8079426e8c39efb62913aac7cf198d6a557c9c55f448d65d8aa492a54cd2ae2e57b5ce3918aa3a75f827e8511fa6196d83e0fa77f45e789fa73cd2773b310f717b8af7bfc3456f6e008f9f8c2286808e4430d8d1b0260a5a0f08616887cc329cd4754a0994979552a26b055541d89419c083bb4bb5de0939716b6235a83962376096cac86e2f3497e16083fc0f126305a5b5d822f79b65411e6a0250b0c229cb9efa1d8f7b64754f21fc2d81d8c122d8cc57eafc2b4b2d2b02b262b65157804674d8d5da0a9c18d1d1f48c75ac8a8196bd52cb789b0b2947dbf63258d968097930fc5abd8e36b9aa1b28c8038a1f87292212ca2c0a55673e2a0480f380acabf71e994271a65230015428d1fb0fa29944c4215f070ccfe537dfe37065db5ba5c90ae76cab0e69e2a5f61d238d52b936769a3f7ed6bd98bafe4d15c17548ede6302f4d806e3217b0035927359463fdaf1ca86c439db078959f3f6aa2de55a8662d700be14b546e2099289b221f7bdf8e8d078547d9996f82f13f9e529e3c758071eab1259735092d4fac514b9bd3b87242350a0497e537ef96ac4241265632779c8a98844dea0cb1496e49fb2ab2f50d9533050c840fd2c9155d4e807a69fdafeca7e7aabdfbe234170d106eb0bc2b6e3a3d0c27fcbb8ec611aa7861d57b0926ca97b7137aceeae7c061cdb619a893fce4a77187948db00828b51e70cfbdb9f6b06aaea8b037452a37aa113c75f8a0d8755f69de8e9dbdaff5dc9742b3723cee611e17f0b5f45389e3794d499698df78583610371d6fb780ab8fb080085c1e5e3312cd0cfdf1c440ce0778f84e49f9ebe6217025d6e0a3caa019dc713390dd68b9d7e2971c85dcef20f0fd39e653d03a15d43920502ab4aaea724d4283bffa5d557519aface6622844659eb8704aba1eb7d1440e9838e5ca42aaf4824ed9174f5cae88f196a15a07fabca68c0a76cb22749d5b96a3f30eba226061d1fc0ccaf6d01858bc5096ce8c231e78e52df028888ce52d1803edd0924c08cde09ec0d1241c98d7bedb141e8abe63b5645fd6bf3b143c42004f91a4d4a4cd2480d333ed34a878fcdde8e16b6ebe9c70237f1d856c0e37e4d9aec479cdb4c8e9316284c2edd3202941fdedd81a6ee4fa6735cac981f8cc1a5609a27bb774b5901281497fb2be671c9dac31aad3c122f3859a9f838f8543c7fc2bab27e84dc4b6a2343c5416c38c8dcbbb56f1e3ccf31644ab66ebe86e77cec68836d3771d7e3a800000000a45a2ec20c3f34f4c69cea200fdf39cc78ff50092f7cb1e2894f4d35"

const PROOF_GROTH16 = `{
    "a": "0x976e8832975ade909192a185fb553f7f66d7ff0b58b2ac69e63635632213011f2fad7e996a95ecdbdf251a2526c7c856f894035765fd8c6e6ebde0bd25f9660d",
    "b": "0x5bc1574562bdb6279caa6e0fe6c228aea9b4ed14d7411f080e5a365d86c30c1901a3f19010881db71db8d73af7ffb80303455625bbd34a8e7e3e3a2d2e194324a86a07c4faf9ba2d96c52af5dc265958b2a9d98823461828fa9d0a65d3830f19fee8146afff5565b27514ab317b08647624a49804081542994ebd7b6e6b20d14",
    "c": "0x538bf8dbeaaaff652d564afe07733ea37c07adf360174a700330a1e4f1c6030b589f8f49709d6d626a822ce2bcb020bfde05c2ad11dd1bf7107088af967be4a4"
}`;
const VK_GROTH16 = `{
    "curve": "Bn254",
    "alphaG1": "0xf23ecc6fdae0957b6f9901baa097ec1192a97795a65ef10147345343eb4901183096f9296b8d74135878afea791ad1e053c33460fefb392c61925bb086a3dda5",
    "betaG2": "0xa17eb8514763a6f1bb824ee9da47097c8529e799f026f544e8e5bdb565f027007313fe210c046dca53e3ecbe79fe12a6dcbadec7e6e370854c49c7768a9088091512d8f91c6c6f2e78b0438ecb511fbd63e0235534d09a0b1643222d841a130cd3b32b17890c6e832aca76c4e28cb31cab8876cf0550881d115edaa9e39da4ad",
    "gammaG2": "0xc57b18d336c2bfd4693a08c7ad91d82c9bc761f569273f15d0b3d3b341f0e11cdf8728fb8d2375eeba14f081b7ed4cb67f7c10197ea90cbb5012bfb8ee820001485d3dd137e7baf0594b73c7b954fa60f0bf5344299d80349ad3a44e2fef962365a8188bf3e4b3769246ef2fb123c5354e868ed667953f513ff72d042678cd02",
    "deltaG2": "0x66bdd7020e111de2367423d630c6b046a1d23ef4aa4983f4476d87bf705b4328ffe5147b93264bf90e0ed74585f43910b43bf0188d86cbd236ea687d0ff7e22d3f2f288e408e98937c1febcbe43874c5ce465bde5cbd6e9628138c26a656dd222d493505af528ff9e12dcd0bbdefa5c97fb502440cfa097045abef314456050a",
    "gammaAbcG1": [
        "0x2c3c89c560512b2d0b08da1e848f41d6ca559d1b58df315625e95ab0310e3b0f4976fe82316d238aa35b63cdff2f0ef108b9d76c6b45f1eb57dbdfcbe663dc9d",
        "0x254fe8f76591c219562ede7a5807212abc9427bdb012a9145fe48fe49077711d36bef432122d026d20ed95a2c1e3d7f0c63e6349e112d6786722f40fa6589811"
    ]
}`;
const INPUTS_GROTH16 = `["0xa75d1fe3e7eb2f0bd2d88886c679582b85a74ee4a6b77b2d07617b85089da420"]`;

const ReturnCode = {
    Ok: 1,
    ErrProofVerificationFailed: 2,
    ErrNoAttestation: 3,
    ErrAttProofVerificationFailed: 4,
    ErrWrongAttestationTiming: 5,
};

const { init_api, BLOCK_TIME, submitProof, submitExtrinsic, waitForNewAttestation, receivedEvents } = require('zkv-lib')


async function run(nodeName, networkInfo, args) {
    const api = await init_api(zombie, nodeName, networkInfo);

    // Create a keyring instance
    const keyring = new zombie.Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');

    // Create the proof submission extrinsics...
    let proofHashesArray = [];

    const eventsFFlonk = await submitProof(api.tx.settlementFFlonkPallet, alice, PROOF_FFLONK, null);
    const proofIncludedTimestamp = Date.now();
    const eventsZksync = await submitProof(api.tx.settlementZksyncPallet, alice, PROOF_ZKSYNC);

    const proof_groth16 = JSON.parse(PROOF_GROTH16);
    const vk_groth16 = JSON.parse(VK_GROTH16);
    const inputs_groth16 = JSON.parse(INPUTS_GROTH16);
    const validProofSubmission = api.tx.settlementGroth16Pallet.submitProof(proof_groth16, vk_groth16, inputs_groth16);
    const eventsGroth16 = await submitExtrinsic(validProofSubmission, alice, BlockUntil.InBlock, (event) => event.section == "poe" && event.method == "NewElement");

    if (!receivedEvents(eventsFFlonk)|| !receivedEvents(eventsZksync) || !receivedEvents(eventsGroth16)) {
        return ReturnCode.ErrProofVerificationFailed;
    }
    proofHashesArray.push(eventsFFlonk[0].data[0]);
    proofHashesArray.push(eventsZksync[0].data[0]);
    proofHashesArray.push(eventsGroth16[0].data[0]);

    // Wait for the next attestation ID to be emitted
    const EXPECTED_ATT_TIMEOUT = BLOCK_TIME * 10;
    const EXPECTED_ATT_TIMEOUT_DELTA = BLOCK_TIME * 3;
    const interestingAttId = await waitForNewAttestation(api, EXPECTED_ATT_TIMEOUT * 2);
    const attTimestamp = Date.now();
    if (interestingAttId == -1) {
        console.log("Something went wrong while waiting for a new attestation");
        return ReturnCode.ErrNoAttestation;
    } else {
        var publishedRoot = interestingAttId.data[1];
        console.log("A new attestation has been published: ");
        interestingAttId.data.forEach((data) => {
            console.log(`\t\t\t${data.toString()}`);
        });
    }

    // Check that the attestation was received in the expected time window
    if (attTimestamp < proofIncludedTimestamp + EXPECTED_ATT_TIMEOUT ||
        attTimestamp > proofIncludedTimestamp + (EXPECTED_ATT_TIMEOUT + EXPECTED_ATT_TIMEOUT_DELTA)) {
        console.log(`Attestation not received in the expected time window`);
        console.log(`${attTimestamp} < ${proofIncludedTimestamp + EXPECTED_ATT_TIMEOUT}`)
        console.log(`${attTimestamp} > ${proofIncludedTimestamp + (EXPECTED_ATT_TIMEOUT + EXPECTED_ATT_TIMEOUT_DELTA)}`);
        return ReturnCode.ErrWrongAttestationTiming;
    }

    // For each proof, get its Merkle path and evaluate the root
    const attId = parseInt(interestingAttId.data['id']);
    const poeFflonk = await api.rpc.poe.proofPath(attId, proofHashesArray[0]);
    const poeZksync = await api.rpc.poe.proofPath(attId, proofHashesArray[1]);
    const poeGroth16 = await api.rpc.poe.proofPath(attId, proofHashesArray[1]);

    console.log('##### proofPath RPC returned (proof fflonk): ' + JSON.stringify(poeFflonk));
    console.log('##### proofPath RPC returned (proof zksync): ' + JSON.stringify(poeZksync));
    console.log('##### proofPath RPC returned (proof groth16): ' + JSON.stringify(poeGroth16));

    // Reconstruct the root from the returned proof
    const proofFflonkVerification = await verifyProof(poeFflonk, publishedRoot);
    console.log("Proof fflonk verification: " + proofFflonkVerification);
    if (!proofFflonkVerification) {
        return ReturnCode.ErrAttProofFailedVerification;
    }

    const proofZksyncVerification = await verifyProof(poeZksync, publishedRoot);
    console.log("Proof zksyn verification: " + proofZksyncVerification);
    if (!proofZksyncVerification) {
        return ReturnCode.ErrAttProofFailedVerification;
    }
    const proofGroth16Verification = await verifyProof(poeGroth16, publishedRoot);
    console.log("Proof groth16 verification: " + proofGroth16Verification);
    if (!proofGroth16Verification) {
        return ReturnCode.ErrAttProofFailedVerification;
    }

    // Any return value different from 1 is considered an error
    return ReturnCode.Ok;
}

function stripHexPrefix(input_str) {
    return input_str.toString().replace(/^0x/, '');
}

function verifyProof(proof, publishedRoot) {
    let position = parseInt(proof['leaf_index'], 10);
    let width = parseInt(proof['number_of_leaves'], 10);
    let hash = Keccak256(proof['leaf'].toString('hex')).toString('hex');
    i = 0
    proof['proof'].forEach(function (p) {
        p = stripHexPrefix(p);
        if (position % 2 == 1 || position + 1 == width /*|| hash.startsWith("faaf")*/) {
            hash = Keccak256('0x' + p + hash).toString('hex');
        } else {
            hash = Keccak256('0x' + hash + p).toString('hex');
        }
        position = parseInt(Math.floor(position / 2), 10);
        width = parseInt(Math.floor((width - 1) / 2) + 1, 10);
        i++;
    });

    let variable = stripHexPrefix(publishedRoot)
    return variable == hash;
}

module.exports = { run }

