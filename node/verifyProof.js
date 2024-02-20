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
    const contractAddress = "secret153h400h7xpgerq4ws0t574mwhyuggt72pqjqq5";
    const contractCodeHash = "0d132c2ed8a8511d154cc2856f7e85fac5a1a005777a742a35ee47687bf91681"

    const owner = "0x42055047b1805df32bb833910eb639afc4cdacd9"

    const claimInfo = {
        "context": "{}",
        "parameters": "",
        "provider": "provider"
    }
    const identifier = [234, 165, 224, 92, 0, 74, 219, 255, 255, 218, 124, 108, 95, 139, 212, 74, 61, 176, 217, 65, 12, 155, 247, 201, 163, 84, 156, 217, 129, 127, 47, 161];
    const signatures = [([113, 222, 76, 71, 123, 251, 43, 136, 75, 174, 65, 178, 0, 25, 210, 157, 1, 24, 180, 16, 166, 75, 141, 152, 31, 61, 229, 219, 60, 171, 241, 233, 13, 17, 134, 24, 78, 48, 161, 98, 181, 31, 221, 158, 109, 71, 105, 137, 245, 194, 201, 17, 103, 226, 244, 77, 232, 189, 114, 161, 179, 250, 234, 223,], 1)]
    const signedClaim = {
        "claim": {
            "epoch": 2,
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
                    calim_info: claimInfo,
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