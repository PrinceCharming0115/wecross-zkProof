import { SecretNetworkClient, Wallet } from "secretjs";
import dotenv from "dotenv";
dotenv.config();

const wallet = new Wallet(process.env.MNEMONIC);

const secretjs = new SecretNetworkClient({
    chainId: "pulsar-3",
    url: "https://api.pulsar.scrttestnet.com",
    wallet: wallet,
    walletAddress: wallet.address,
});

let verify_proof = async () => {
    const contractAddress = "secret1f4r560x76whq2p2pu460tqug3ym7qxf2ln8xpj"
    const contractCodeHash = "137b30ce8969bb0cb4937092838eb597b3dde195acb2c91a07d7ac95c4c7b4c8"
    const owner = "0xce1e47c94ec43012af8243090152463b0cea7974"

    const claimInfo = {
        "context": "{}",
        "parameters": "",
        "provider": "provider"
    }
    const identifier = [234,165,224,92,0,74,219,255,255,218,124,108,95,139,212,74,61,176,217,65,12,155,247,201,163,84,156,217,129,127,47,161]
    const signatures = [[252,249,213,116,11,237,17,35,73,107,134,56,153,101,165,228,12,112,198,118,149,135,241,26,156,136,52,80,200,66,174,37,61,25,42,125,180,69,155,43,108,53,221,198,31,79,174,126,229,51,150,196,198,136,60,97,229,150,47,114,134,240,226,132]]
    const signedClaim = {
        "claim": {
            "epoch": 1,
            "identifier": identifier,
            "owner": owner,
            "timestamp_s": 1571797419
        },
        "signatures": signatures
    }

    let tx2 = await secretjs.tx.compute.executeContract(
        {
            sender: wallet.address,
            contract_address: contractAddress,
            msg: {
                verify_proof: {
                    claim_info: claimInfo,
                    signed_claim: signedClaim,
                }
            },
            code_hash: contractCodeHash,
        },
        { gasLimit: 100_000 }
    );

    console.log(tx2)
};

verify_proof();