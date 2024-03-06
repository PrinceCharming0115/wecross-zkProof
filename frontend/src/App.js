import "./App.css";
import ProofForm from "./components/ProofForm";
import { useState } from "react";

function App() {
  
  return (
    <div className="App">
      <ProofForm
        claimInfo={{
          "provider": "provider",
          "parameters": "param",
          "context": "{}",
      }}
        signedClaim={{
          "claim": {
              "identifier": "0xa6db2030140d1a1297ea836cf1fb0a1b467c5c21499dc0cd08dba63d62a6fdcc",
              "owner": "0xe70415eb011253b6721d4f9149dd525d6afe370f",
              "epoch": "1",
              "timestamp_s": 1709553706
          },
          "signatures": [{
            signature: "d8076039793e7014a9fd746b8531530d52d66c4c622e346ca1f157323348cd5e53cd95e00f88e211b6fddc3bee78c8fbbecc4469000620764f611e4d2b7dabde",
            recovery_param: 0,
        }],
      }}
      />
    </div>
  );
}

export default App;
