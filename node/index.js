import { SecretNetworkClient, Wallet } from "secretjs";
import * as fs from "fs";
import dotenv from "dotenv";
dotenv.config();

const wallet = new Wallet(process.env.MNEMONIC);
const owner = process.env.OWNER
const contract_wasm = fs.readFileSync("../reclaim_cosmwasm.wasm.gz");

const secretjs = new SecretNetworkClient({
    chainId: "pulsar-3",
    url: "https://api.pulsar.scrttestnet.com",
    wallet: wallet,
    walletAddress: wallet.address,
});


let upload_contract = async () => {
    let tx = await secretjs.tx.compute.storeCode(
        {
            sender: wallet.address,
            wasm_byte_code: contract_wasm,
            source: "",
            builder: "",
        },
        {
            gasLimit: 4_000_000,
        }
    );

    const codeId = Number(
        tx.arrayLog.find((log) => log.type === "message" && log.key === "code_id")
            .value
    );

    console.log("codeId: ", codeId);

    const contractCodeHash = (
        await secretjs.query.compute.codeHashByCodeId({ code_id: codeId })
    ).code_hash;
    console.log(`Contract hash: ${contractCodeHash}`);

};

upload_contract();

let instantiate_contract = async () => {
    // Create an instance of the Counter contract, providing a starting count
    const instantiateMsg = { owner: owner };
    const codeId = 3997
    const contractCodeHash = "3a9c6ac3d954fe86a980b8fc02b0914f57113d06660e52b03d527ef3300025ea"
    let tx = await secretjs.tx.compute.instantiateContract(
      {
        code_id: codeId,
        sender: wallet.address,
        code_hash: contractCodeHash,
        init_msg: instantiateMsg,
        label: "Reclaim" + Math.ceil(Math.random() * 10000),
      },
      {
        gasLimit: 400_000,
      }
    );
  
    //Find the contract_address in the logs
    const contractAddress = tx.arrayLog.find(
      (log) => log.type === "message" && log.key === "contract_address"
    ).value;
  
    console.log(contractAddress);
  };
  
  instantiate_contract();