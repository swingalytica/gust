import init, { FourWinning } from "@swingalytica/gust";
import wasmUrl from "../../pkg/gust_bg.wasm?url";
import "./style.css";

let game: FourWinning;
let board: any[][];

const PLAYERS = [
  { id: "1", pos: 0, name: "Player 1", color: "#e74c3c", data: [] },
  { id: "2", pos: 1, name: "Player 2", color: "#f1c40f", data: [] },
];

async function setup() {
  await init({ module_or_path: wasmUrl });
  game = new FourWinning();
  PLAYERS.forEach((p) => game.add_player(p));
  board = game.generate_game_board(10, 276, "m");
  game.start(new Date().toISOString());
  render();
}

function render() {
  const players = game.get_players() as typeof PLAYERS;

  document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
    <header>
      <h1>4Winning</h1>
      <p class="subtitle">G[olf Games in R]ust &mdash; Demo</p>
    </header>

    <div class="players">
      ${players
        .map(
          (p) => `
        <div class="player-card" data-id="${p.id}">
          <span class="dot" style="background:${p.color}"></span>
          <span class="player-name">${p.name}</span>
          <span class="shot-count">${p.data.length} shots</span>
        </div>
      `,
        )
        .join("")}
    </div>

    <div class="board-wrapper">
      <div class="col-labels">
        ${board[0]
          .filter((c: any) => c.col)
          .map((c: any) => `<div class="col-label">${c.col.toUpperCase()}</div>`)
          .join("")}
      </div>
      <div class="board">
        ${board
          .map(
            (row: any[], rowIdx: number) => `
          <div class="board-row">
            <div class="row-label">${rowIdx + 1}</div>
            ${row
              .filter((c: any) => c.col)
              .map(
                (cell: any) => `
              <div class="cell ${cell.player_id ? "taken" : "empty"}"
                   data-coord="${cell.col}${cell.row}"
                   style="--cell-color: ${cell.player_id ? cell.color : "transparent"}">
                <span class="cell-text">${cell.text}</span>
              </div>
            `,
              )
              .join("")}
          </div>
        `,
          )
          .join("")}
      </div>
    </div>

    <button id="reset">Reset Game</button>
  `;

  document.querySelectorAll(".cell.empty").forEach((el) => {
    el.addEventListener("click", () => {
      const coord = (el as HTMLElement).dataset.coord!;
      game.click_cell(coord);
      board = game.get_board();

      const winner_id = game.get_winner_id();
      if (winner_id) {
        const players = game.get_players() as typeof PLAYERS;
        const winner = players.find((p) => p.id === winner_id);
        alert(`${winner?.name} wins!`);
      }

      render();
    });
  });

  document.querySelector("#reset")!.addEventListener("click", () => {
    game = new FourWinning();
    PLAYERS.forEach((p) => game.add_player({ ...p, data: [] as string[] }));
    board = game.generate_game_board(10, 276, "m");
    game.start(new Date().toISOString());
    render();
  });
}

await setup();
