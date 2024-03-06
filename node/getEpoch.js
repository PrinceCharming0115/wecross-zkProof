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
    const contractAddress = process.env.CONTRACT_ADDRESS;
    const contractCodeHash = process.env.CONTRACT_CODE_HASH;


    let tx0 = await secretjs.query.compute.queryContract({
        contract_address: contractAddress,
        code_hash: contractCodeHash,
        query: {
            get_epoch: {
                id: "1"
            }
        }
    })
    console.log(tx0);
}

get_epoch()