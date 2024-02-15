import "./App.css";
import ProofForm from "./components/ProofForm";
import { useState } from "react";

function App() {
  
  return (
    <div className="App">
      <ProofForm
        claimInfo={{
          "context": "",
          "parameters": "{\"uid\":\"673906874713\"}",
          "provider": "uidai-uid"
        }}
        signedClaim={{
          "claim": {
            "epoch": 2,
            "identifier": "0xafb5c7415e79bbf42b122d3c0d02d7b8da9deb04df933b95318b57483d587ae3",
            "owner": "0xdFb1dCADeeEC3273Fb2C50563312D1d5f7347615",
            "timestamp_s": 1697188555
          },
          "bytes": [
            "0x17a4133c87ebe482a33607486b5014b9cc92890cdd862db405dbcaf1b96112f829a87d411d8fd25fcd408c021e87e345457d251f8b8afdb13476ca89b8aa80c31b"
          ]
        }}
      />
    </div>
  );
}

export default App;
