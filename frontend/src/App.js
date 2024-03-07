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
            "timestampS": 1709797755
          },
          "signatures": [
            "0xfe5f5d8a9d2e0fb1515ce190d23ef6a8bd962880c24bcec232fa69254bab9e61634deea583794ff7041f0e10e4d550797fd5bab2106c10bec2c0a30e1cd17fe41c",],
        }}
      />
    </div>
  );
}

export default App;
