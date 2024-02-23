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
    const contractAddress = "secret17t02h2lxw4yw2e0cdp02042lm08j85jhw8t6a4";
    const contractCodeHash = "96b419c550bde0e88be6e93629da7e6978d83751e857273f424bb899469834a6"
    const owner = "0x76f6b994e78079940634f8c1c856f8a5b883259a"

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