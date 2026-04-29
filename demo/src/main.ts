import init, { FourWinning } from "@swingalytica/gust";
import wasmUrl from "../../pkg/gust_bg.wasm?url";
import "./style.css";

async function main() {
  await init(wasmUrl);
  const game = new FourWinning();
  const board = game.generate_game_board([0, 100], "m");

  return board;
}

await main();

document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <h1>Gust Demo</h1>

  <pre>${JSON.stringify(await main(), null, 2)}</pre>
`;
