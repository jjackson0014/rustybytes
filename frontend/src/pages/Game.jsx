import { useEffect, useState } from "react";

// WASM example
//export default function Game() {
//  useEffect(() => {
//    import("../game/game.wasm").then((wasm) => {
//      wasm.run();
//    });
//  }, []);
//
//  return <div id="game-container">Loading Tic-Tac-Toe...</div>;
//}

export default function Game() {
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Simulate a delay before loading the game
    setTimeout(() => {
      setLoading(false);
    }, 2000);
  }, []);

  return (
    <div>
      <h1>RustyBytes Tic-Tac-Toe</h1>
      {loading ? (
        <p>Loading game...</p>
      ) : (
        <div style={{ 
          width: "300px", 
          height: "300px", 
          border: "2px solid black", 
          display: "flex", 
          alignItems: "center", 
          justifyContent: "center",
          fontSize: "20px",
          background: "#f0f0f0"
        }}>
          Game Placeholder
        </div>
      )}
    </div>
  );
}
