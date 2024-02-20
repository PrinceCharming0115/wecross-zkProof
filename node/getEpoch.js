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

let get_epoch = async () => {
    const contractAddress = "secret153h400h7xpgerq4ws0t574mwhyuggt72pqjqq5";
    const contractCodeHash = "0d132c2ed8a8511d154cc2856f7e85fac5a1a005777a742a35ee47687bf91681"


    let tx0 = await secretjs.query.compute.queryContract({
        contract_address: contractAddress,
        code_hash: contractCodeHash,
        query: {
            get_epoch: {
                id: "2"
            }
        }
    })
    console.log(tx0);
}

get_epoch()