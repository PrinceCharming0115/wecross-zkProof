import { useContext } from "react";
import { MsgExecuteContract } from "secretjs";
import { SecretjsContext } from "./SecretjsContext";

let contractCodeHash =
  "21126865630e9a582049fb9286e8fcf7f9363b61ff92f6bcb369a392d6d89094";
let contractAddress = "secret1ffjsywxtdwfjr634dmx8jz3yjqzf26fnsccqtj";

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
