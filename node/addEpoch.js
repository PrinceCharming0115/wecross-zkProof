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
    const contractAddress = "secret1f4r560x76whq2p2pu460tqug3ym7qxf2ln8xpj"
    const contractCodeHash = "137b30ce8969bb0cb4937092838eb597b3dde195acb2c91a07d7ac95c4c7b4c8"
    const owner = "0xce1e47c94ec43012af8243090152463b0cea7974"

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