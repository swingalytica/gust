import init, { FourWinning, Shot } from "@swingalytica/gust";
import wasmUrl from "../../pkg/gust_bg.wasm?url";
import "./style.css";

async function main() {
  await init(wasmUrl);
  const game = new FourWinning();

  const shot = new Shot(100, 20);
  game.add_shot(0, shot);

  console.log(game.current_player());
}

document.querySelector<HTMLDivElement>("#app")!.innerHTML = `

`;

await main();
