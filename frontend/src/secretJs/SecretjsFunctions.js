import { useContext } from "react";
import { MsgExecuteContract } from "secretjs";
import { SecretjsContext } from "./SecretjsContext";

let contractCodeHash =
  "034fc6bf3aa07e55599342780bd4d9c9937895cd6ae2162ceb8bf08a2602a549";
let contractAddress = "secret1av9jhzrs330jql4s0uz7jzr0t0pug3zaur7u43";

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
