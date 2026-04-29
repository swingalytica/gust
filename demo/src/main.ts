import init, { FourWinning } from "@swingalytica/gust";
import wasmUrl from "../../pkg/gust_bg.wasm?url";
import "./style.css";

async function main() {
  await init(wasmUrl);
  const game = new FourWinning();
  game.add_player({ id: "1", pos: 0, name: "Player 1", color: "red", data: ["a"] });
  game.add_player({ id: "2", pos: 0, name: "Player 2", color: "yellow", data: [] });
  const board = game.generate_game_board(10, 276, "meters");
  console.log(board);
  game.update_player("1", { name: "Player 1 Updated", color: "blue" });
  return board;
}

const board = await main();

// Generate a table to display the game board
// Determine board dimensions
const numRows = board.length;

// Derive column labels from the first row (preserving order, including empty cols so the right info column is kept)
const colLabels: string[] = [];
if (numRows > 0) {
  // take all cells except the very first (which is the row text/label)
  for (let i = 1; i < board[0].length; i++) {
    // include empty strings as labels for info columns on the right
    const col = board[0][i]?.col ?? "";
    colLabels.push(col);
  }
}
const numCols = colLabels.length;

// Helper to get a row label text (e.g. "1 meters")
function getRowText(rowArr: any[]) {
  return rowArr[0]?.text ?? "";
}

// New helper: derive the numeric row label from the first non-empty cell's row property,
// falling back to a simple ascending index if absent.
function getRowNumber(rowArr: any[], rowIndex: number) {
  const firstNonEmpty = rowArr.find((c) => c.col && c.col !== "");
  if (firstNonEmpty && typeof firstNonEmpty.row === "number") return firstNonEmpty.row;
  return rowIndex + 1;
}

// Generate table with row/col labels
document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <h1>Gust Demo</h1>
  <table>
    <tbody>
      ${board
        .map((row, rowIndex) => {
          const rowNumber = getRowNumber(row, rowIndex); // use source order (not reversed)
          const rowText = getRowText(row);
          // take only the cells that correspond to our derived column labels:
          const dataCells = row.slice(1, 1 + numCols);
          return `
        <tr>
          <td style="font-weight:bold;">${rowNumber}</td>
          <td style="font-weight:bold;">${rowText}</td>
          ${dataCells
            .map(
              (cell) => `
            <td style="background:${cell.color || "transparent"};">${cell.text}</td>
          `,
            )
            .join("")}
        </tr>
      `;
        })
        .join("")}
      <tr>
        <td></td>
        <td></td>
        ${colLabels.map((col) => `<td style="font-weight:bold;">${col}</td>`).join("")}
      </tr>
    </tbody>
  </table>
`;
