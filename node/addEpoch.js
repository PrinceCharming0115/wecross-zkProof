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

let add_epoch = async () => {
    const contractAddress = "secret153h400h7xpgerq4ws0t574mwhyuggt72pqjqq5";
    const contractCodeHash = "0d132c2ed8a8511d154cc2856f7e85fac5a1a005777a742a35ee47687bf91681"
    const owner = "0x42055047b1805df32bb833910eb639afc4cdacd9"

    let tx = await secretjs.tx.compute.executeContract(
        {
            sender: wallet.address,
            contract_address: contractAddress,
            msg: {
                add_epoch: {
                    witness: [{ address: owner, host: "" }],
                    minimum_witness: 1,
                }
            },
            code_hash: contractCodeHash,
        },
        { gasLimit: 100_000 }
    );

};

add_epoch();