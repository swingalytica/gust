# Gust — G[olf Games in R]ust

Gust is a free Open Source golf games engine focused on Golf Training Engagement and Fun.

**Gust is in early development. The API is not stable yet and may change without a major version bump.**

## Installation

```bash
npm install @swingalytica/gust
```

## Games

- **4Winning** — Connect Four on distance zones
- **Exact** — Consistency and repeatability _(coming soon)_
- **Precision** — Get as close to the target as possible _(coming soon)_
- **Sinking Ships** — Battleship on the range _(coming soon)_

## Usage

```typescript
import init, { FourWinning } from "@swingalytica/gust";

await init({ module_or_path: wasmUrl });

const game = new FourWinning();
game.add_player({ id: "1", name: "Player 1", color: "#e74c3c", pos: 0, data: [] });
game.generate_game_board(10, 276, "m");
game.start(new Date().toISOString());
game.click_cell("a1");
```

## License

AGPL-3.0 — see [LICENSE](LICENSE). Commercial licensing: <info@swingalytica.com>
