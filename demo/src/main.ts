import init, { FourWinning } from "@swingalytica/gust";
import wasmUrl from "../../pkg/gust_bg.wasm?url";
import "./style.css";

async function main() {
  await init(wasmUrl);
  const game = new FourWinning();
  game.add_player({ id: "1", pos: 0, name: "Player 1", color: "red", data: [] });
  game.add_player({ id: "2", pos: 0, name: "Player 2", color: "yellow", data: [] });
  const board = game.generate_game_board(10, 276, "meters");
  console.log(game.get_players());
  return board;
}

const board = await main();

// Generate a table to display the game board
document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <h1>Gust Demo</h1>

  <table>
    <tbody>
      ${board
        .map(
          (row) => `
        <tr>
          ${row
            .map(
              (cell) => `
            <td>${cell.text}</td>
          `,
            )
            .join("")}
        </tr>
      `,
        )
        .join("")}
    </tbody>
  </table>
`;
