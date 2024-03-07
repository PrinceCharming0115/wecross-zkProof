import { useContext } from "react";
import { MsgExecuteContract } from "secretjs";
import { SecretjsContext } from "./SecretjsContext";

let contractCodeHash =
  "5516c3d7c45f21a75eebf4e3657c8b54b97b65ee3328edb72d6508f8550919bf";
let contractAddress = "secret12faxc5tv8u0rj86dud4m2fgfufwzsz4henmez5";

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
