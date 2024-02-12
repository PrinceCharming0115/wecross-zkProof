import { useContext } from "react";
import { MsgExecuteContract } from "secretjs";
import { SecretjsContext } from "./SecretjsContext";

let contractCodeHash =
  "31664dde7695fa36fb2f5c41247342eb44d1781c39ed2153c667e5039212a26e";
let contractAddress = "secret1nen4nw848agh3zwwdum3650wz2jaua72jp4389";

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
