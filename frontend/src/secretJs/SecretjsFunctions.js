import { useContext } from "react";
import { MsgExecuteContract } from "secretjs";
import { SecretjsContext } from "./SecretjsContext";

let contractCodeHash =
  "57f12f617e7b4bf202675b94da2e486042d4d584b79d5e07ece8a432e8a4b42d";
let contractAddress = "secret1z7v94lkjmhvv58zcqydgwzqp27umdmhgfx27q7";

const SecretjsFunctions = () => {
  const { secretjs, secretAddress } = useContext(SecretjsContext);

  let verify_proof = async (claimInfo, signedClaim) => {
    const proof = {
      claimInfo: claimInfo,
      signedClaim: signedClaim,
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
