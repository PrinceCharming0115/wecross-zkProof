import { SecretNetworkClient, Wallet } from "secretjs";
import * as fs from "fs";
import dotenv from "dotenv";
dotenv.config();

const wallet = new Wallet(process.env.MNEMONIC); // YOUR passphrase
const contract_wasm = fs.readFileSync("../reclaim_cosmwasm.wasm.gz");
const owner = process.env.OWNER // YOUR address corresponding to the MNEMONIC above

const secretjs = new SecretNetworkClient({
    chainId: "pulsar-3",
    url: "https://api.pulsar.scrttestnet.com",
    wallet: wallet,
    walletAddress: wallet.address,
});


let codeId, contractCodeHash, contractAddress


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

    codeId = Number(
        tx.arrayLog.find((log) => log.type === "message" && log.key === "code_id")
            .value
    );

    console.log("codeId: ", codeId);

    contractCodeHash = (
        await secretjs.query.compute.codeHashByCodeId({ code_id: codeId })
    ).code_hash;
    console.log(`Contract hash: ${contractCodeHash}`);

};

await upload_contract();

let instantiate_contract = async () => {
    const instantiateMsg = { owner: owner };
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
    contractAddress = tx.arrayLog.find(
        (log) => log.type === "message" && log.key === "contract_address"
    ).value;

    console.log(contractAddress);
};

await instantiate_contract();

let add_epoch = async () => {
    const owner1 = "0x0000000000000000000000000000000000000000"
    const owner2 = "0x244897572368eadf65bfbc5aec98d8e5443a9072"

    let tx1 = await secretjs.tx.compute.executeContract(
        {
            sender: wallet.address,
            contract_address: contractAddress,
            msg: {
                add_epoch: {
                    witness: [{ address: owner1, host: "" }],
                    minimum_witness: "1",
                }
            },
            code_hash: contractCodeHash,
        },
        { gasLimit: 100_000 }
    );
    // console.log(tx)

    let tx2 = await secretjs.tx.compute.executeContract(
        {
            sender: wallet.address,
            contract_address: contractAddress,
            msg: {
                add_epoch: {
                    witness: [{ address: owner2, host: "https://reclaim-node.questbook.app" }],
                    minimum_witness: "1",
                }
            },
            code_hash: contractCodeHash,
        },
        { gasLimit: 100_000 }
    );
};

await add_epoch();

let verify_proof = async () => {
    const owner = "0x597b40d79e93509832ec13ec4eb8c3f316c11b4f"

    const claimInfo = {
        "provider": "http",
        "parameters": "{\"body\":\"\",\"geoLocation\":\"in\",\"method\":\"GET\",\"responseMatches\":[{\"type\":\"contains\",\"value\":\"_steamid\\\">Steam ID: 76561199632643233</div>\"}],\"responseRedactions\":[{\"jsonPath\":\"\",\"regex\":\"_steamid\\\">Steam ID: (.*)</div>\",\"xPath\":\"id(\\\"responsive_page_template_content\\\")/div[@class=\\\"page_header_ctn\\\"]/div[@class=\\\"page_content\\\"]/div[@class=\\\"youraccount_steamid\\\"]\"}],\"url\":\"https://store.steampowered.com/account/\"}",
        "context": "{\"contextAddress\":\"user's address\",\"contextMessage\":\"for acmecorp.com on 1st january\"}",
    }

    const identifier = "0x531322a6c34e5a71296a5ee07af13f0c27b5b1e50616f816374aff6064daaf55"

    const signedClaim = {
        "claim": {
            "identifier": identifier,
            "owner": owner,
            "epoch": 2,
            "timestampS": 1709797755
        },
        "signatures": ["0xfe5f5d8a9d2e0fb1515ce190d23ef6a8bd962880c24bcec232fa69254bab9e61634deea583794ff7041f0e10e4d550797fd5bab2106c10bec2c0a30e1cd17fe41c"],
    }


    const proof = {
        claimInfo: claimInfo,
        signedClaim: signedClaim
    }

    let tx = await secretjs.tx.compute.executeContract(
        {
            sender: wallet.address,
            contract_address: contractAddress,
            msg: {
                verify_proof: {
                    proof: proof
                }
            },
            code_hash: contractCodeHash,
        },
        { gasLimit: 100_000 }
    );

    console.log(tx)
};

await verify_proof();