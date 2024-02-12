import { useContext } from "react";
import { MsgExecuteContract } from "secretjs";
import { SecretjsContext } from "./SecretjsContext";

let contractCodeHash =
  "3a9c6ac3d954fe86a980b8fc02b0914f57113d06660e52b03d527ef3300025ea";
let contractAddress = "secret1j3x08m7te9ckzlrtrwrdq4uz85acmwjafwrtsw";

const SecretjsFunctions = () => {
  const { secretjs, secretAddress } = useContext(SecretjsContext);

  let verify_proof = async (claimInfo, signedClaim) => {
    console.log(claimInfo, signedClaim)
    let tx = await secretjs.tx.compute.executeContract(
      {
        sender: secretAddress,
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

    console.log(tx);
  };

  return {
    verify_proof
  };
};

export { SecretjsFunctions };
