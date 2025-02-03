import { useEffect } from "react";

export default function Game() {
  useEffect(() => {
    import("../game/game.wasm").then((wasm) => {
      wasm.run(); // Example: Adjust based on your Bevy game's WASM function
    });
  }, []);

  return <div id="game-container">Loading Tic-Tac-Toe...</div>;
}
