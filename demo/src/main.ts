import init, { Exact } from "@swingalytica/gust";
import wasmUrl from "../../pkg/gust_bg.wasm?url";

let game: any;
let board: any[][];

async function setup() {
  await init({ module_or_path: wasmUrl });
  game = new Exact();
  board = game.get_game_board();
  console.log(board);
}

await setup();
