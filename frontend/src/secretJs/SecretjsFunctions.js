import { useContext } from "react";
import { MsgExecuteContract } from "secretjs";
import { SecretjsContext } from "./SecretjsContext";

let contractCodeHash =
  "85a48dca9466955ae898f470a16a8cd4f784535794a2e19734031b6bc19d85e7";
let contractAddress = "secret1pn2rgadfzsv85cmwudktgfxgp3v50k376jzz6m";

const SecretjsFunctions = () => {
  const { secretjs, secretAddress } = useContext(SecretjsContext);

  let verify_proof = async (claimInfo, signedClaim) => {
    const proof = {
      claim_info: claimInfo,
      signed_claim: signedClaim,
    }

    console.log(proof)

    let tx = await secretjs.tx.compute.executeContract(
      {
        sender: secretAddress,
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

    console.log(tx);
  };

  return {
    verify_proof
  };
};

export { SecretjsFunctions };
