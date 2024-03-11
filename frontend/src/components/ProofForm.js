import RicherModal from "./RicherModal";
import { WalletIcon } from "@heroicons/react/24/outline";
import { SecretjsContext } from "../secretJs/SecretjsContext";
import { SecretjsFunctions } from "../secretJs/SecretjsFunctions";
import { useContext, useState } from "react";

const realProof = {
  "identifier": "0xbd0f6ba9f99fe302576ee7cd469abf2787f21e9fedb05e3c963783f7e0fe8129",
  "claimData": {
    "provider": "http",
    "parameters": "{\"body\":\"\",\"method\":\"GET\",\"responseMatches\":[{\"type\":\"contains\",\"value\":\"_steamid\\\">Steam ID: 76561199564944093</div>\"}],\"responseRedactions\":[{\"jsonPath\":\"\",\"regex\":\"_steamid\\\">Steam ID: (.*)</div>\",\"xPath\":\"id(\\\"responsive_page_template_content\\\")/div[@class=\\\"page_header_ctn\\\"]/div[@class=\\\"page_content\\\"]/div[@class=\\\"youraccount_steamid\\\"]\"}],\"url\":\"https://store.steampowered.com/account/\"}",
    "owner": "0x6f37e34642693a0c433dcf5998ec3e30b62be9b9",
    "timestampS": 1707292621,
    "context": "{\"contextAddress\":\"0x0\",\"contextMessage\":\"\"}",
    "identifier": "0xbd0f6ba9f99fe302576ee7cd469abf2787f21e9fedb05e3c963783f7e0fe8129",
    "epoch": 2
  },
  "signatures": [
    "0xe6fb44b8162f42994a57626342d518f7fb032acf952ab952209808da183a54a64e522d9edd1783ee95f2124484e87be40c12512cbb19eecf5c9140f3353716371c"
  ],
  "witnesses": [
    {
      "id": "0x244897572368eadf65bfbc5aec98d8e5443a9072",
      "url": "https://reclaim-node.questbook.app"
    }
  ],
  "extractedParameterValues": {
    "CLAIM_DATA": "76561199564944093"
  }
}

function ProofForm(props) {
  const { secretAddress, connectWallet } = useContext(SecretjsContext);
  const { verify_proof } =
    SecretjsFunctions();

  const [proof, setProof] = useState(props.proof);
  const [richerModalOpen, setRicherModalOpen] = useState(false);
  const [showRicherButton, setShowRicherButton] = useState(true);

  const transformProof = (proof) => {
    const claimInfoBuilder = new Map([
      ["context", proof["claimData"]["context"]],
      ["parameters", proof["claimData"]["parameters"]],
      ["provider", proof["claimData"]["provider"]],
    ]);
    const claimInfo = Object.fromEntries(claimInfoBuilder);
    const claimBuilder = new Map([
      ["epoch", proof["claimData"]["epoch"]],
      ["identifier", proof["claimData"]["identifier"]],
      ["owner", proof["claimData"]["owner"]],
      ["timestampS", proof["claimData"]["timestampS"]],
    ]);
    const signedClaim = {
      claim: Object.fromEntries(claimBuilder),
      signatures: proof["signatures"],
    };
    console.log({ claimInfo, signedClaim })
    return { claimInfo, signedClaim };
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      await verify_proof(
        transformProof(JSON.parse(proof))
      );
      setRicherModalOpen(true);
      setShowRicherButton(false);
    } catch (error) {
      console.log(error)
      alert("Please approve the transaction in keplr.");
    }
  };

  return (
    <>
      <div className="flex min-h-full flex-1 flex-col justify-center px-6 py-12 lg:px-8">
        <div className="sm:mx-auto sm:w-full sm:max-w-sm">
          <div className="flex justify-end -mb-4 ">
            <WalletIcon
              onClick={connectWallet}
              className="h-10 w-10 text-white hover:text-indigo-500  "
            />
          </div>
          <h2 className=" -mt-8 -mb-12 text-center text-2xl font-bold leading-9 tracking-tight text-white">
            {secretAddress ? secretAddress.slice(0, 10) + "...." + secretAddress.slice(41, 45) : "Please connect wallet"}
          </h2>
        </div>

        <br></br>

        <div className="mt-10 sm:mx-auto sm:w-full sm:max-w-sm ">
          <form onSubmit={handleSubmit} className="space-y-2">
            <br></br>
            <div className="border-4 rounded-lg p-2 ">
              <div className="flex items-center justify-between ">
                <label className="block text-sm font-medium leading-6 text-white">
                  Submit Proof
                </label>
              </div>
              <div className="mt-2">
                <textarea
                  type="text"
                  value={proof}
                  rows={25}
                  cols={50}
                  onChange={(e) => setProof(e.target.value)}
                  placeholder={JSON.stringify(realProof)}
                  required
                  className="block w-full rounded-md border-0 bg-white/5
                py-1.5 text-white shadow-sm ring-1 ring-inset ring-white/10
                focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:text-sm
                sm:leading-6"
                />
              </div>
            </div>

            <div className="pt-4">
              {showRicherButton &&
                <button
                  type="submit"
                  id="richer-submit"
                  onClick={handleSubmit}
                  className="flex w-32 mx-auto justify-center rounded-md bg-indigo-500 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-400 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500"
                >
                  Submit Proof
                </button>
              }
            </div>
          </form>

          <p className="mt-10 text-center text-sm text-gray-400">
            Built on{" "}
            <a
              href="https://docs.scrt.network/secret-network-documentation/"
              className="font-semibold leading-6 text-indigo-400 hover:text-indigo-300"
            >
              Secret.
            </a>
          </p>
        </div>
      </div>
      <RicherModal
        richerModalOpen={richerModalOpen}
        setRicherModalOpen={setRicherModalOpen}
      />
    </>
  );
}

export default ProofForm;
