import "./App.css";
import ProofForm from "./components/ProofForm";
import { useState } from "react";

function App() {

  return (
    <div className="App">
      <ProofForm
        claimInfo={{
          "provider": "http",
          "parameters": "{\"body\":\"\",\"geoLocation\":\"in\",\"method\":\"GET\",\"responseMatches\":[{\"type\":\"contains\",\"value\":\"_steamid\\\">Steam ID: 76561199632643233</div>\"}],\"responseRedactions\":[{\"jsonPath\":\"\",\"regex\":\"_steamid\\\">Steam ID: (.*)</div>\",\"xPath\":\"id(\\\"responsive_page_template_content\\\")/div[@class=\\\"page_header_ctn\\\"]/div[@class=\\\"page_content\\\"]/div[@class=\\\"youraccount_steamid\\\"]\"}],\"url\":\"https://store.steampowered.com/account/\"}",
          "context": "{\"contextAddress\":\"user's address\",\"contextMessage\":\"for acmecorp.com on 1st january\"}",
        }}
        signedClaim={{
          "claim": {
            "identifier": "0x531322a6c34e5a71296a5ee07af13f0c27b5b1e50616f816374aff6064daaf55",
            "owner": "0x597b40d79e93509832ec13ec4eb8c3f316c11b4f",
            "epoch": 2,
            "timestamp_s": 1709797755
          },
          "signatures": [{
            signature: "fe5f5d8a9d2e0fb1515ce190d23ef6a8bd962880c24bcec232fa69254bab9e61634deea583794ff7041f0e10e4d550797fd5bab2106c10bec2c0a30e1cd17fe4",
            recovery_param: 1,
          }],
        }}
      />
    </div>
  );
}

export default App;




const oproof =
{
  "proof": {
    "claimInfo": {
      "context": "",
      "parameters": "{\"uid\":\"673906874713\"}",
      "provider": "uidai-uid"
    },
    "signedClaim": {
      "claim": {
        "epoch": 2,
        "identifier": "0xafb5c7415e79bbf42b122d3c0d02d7b8da9deb04df933b95318b57483d587ae3",
        "owner": "0xdFb1dCADeeEC3273Fb2C50563312D1d5f7347615",
        "timestampS": "1697188555"
      },
      "signatures": [
        "0x17a4133c87ebe482a33607486b5014b9cc92890cdd862db405dbcaf1b96112f829a87d411d8fd25fcd408c021e87e345457d251f8b8afdb13476ca89b8aa80c31b"
      ]
    }
  }
}

const nproof = {
  "identifier": "0x531322a6c34e5a71296a5ee07af13f0c27b5b1e50616f816374aff6064daaf55",
  "claimData": {
    "provider": "http",
    "parameters": "{\"body\":\"\",\"geoLocation\":\"in\",\"method\":\"GET\",\"responseMatches\":[{\"type\":\"contains\",\"value\":\"_steamid\\\">Steam ID: 76561199632643233</div>\"}],\"responseRedactions\":[{\"jsonPath\":\"\",\"regex\":\"_steamid\\\">Steam ID: (.*)</div>\",\"xPath\":\"id(\\\"responsive_page_template_content\\\")/div[@class=\\\"page_header_ctn\\\"]/div[@class=\\\"page_content\\\"]/div[@class=\\\"youraccount_steamid\\\"]\"}],\"url\":\"https://store.steampowered.com/account/\"}",
    "owner": "0x597b40d79e93509832ec13ec4eb8c3f316c11b4f",
    "timestampS": 1709797755,
    "context": "{\"contextAddress\":\"user's address\",\"contextMessage\":\"for acmecorp.com on 1st january\"}",
    "identifier": "0x531322a6c34e5a71296a5ee07af13f0c27b5b1e50616f816374aff6064daaf55",
    "epoch": 2
  },
  "signatures": [
    "0xfe5f5d8a9d2e0fb1515ce190d23ef6a8bd962880c24bcec232fa69254bab9e61634deea583794ff7041f0e10e4d550797fd5bab2106c10bec2c0a30e1cd17fe41c"
  ],
  "witnesses": [
    {
      "id": "0x244897572368eadf65bfbc5aec98d8e5443a9072",
      "url": "https://reclaim-node.questbook.app"
    }
  ],
  "extractedParameterValues": {
    "CLAIM_DATA": "76561199632643233"
  }
}