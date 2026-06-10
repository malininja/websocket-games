import "./tic-tac-toe.css";

type TileData = "X" | "O" | null;

type BoardData = [
  [TileData, TileData, TileData],
  [TileData, TileData, TileData],
  [TileData, TileData, TileData],
];

function renderBoard(boardData: BoardData) {
  for (const [i, row] of boardData.entries()) {
    for (const [j, tileData] of row.entries()) {
      const cell = document.querySelector<HTMLButtonElement>(
        `[data-row="${i}"][data-column="${j}"]`,
      );

      cell!.textContent = tileData;
    }
  }
}

const board = document.querySelector<HTMLDivElement>("#board")!;

board.addEventListener("click", (e) => {
  const cell = (e.target as HTMLElement).closest<HTMLButtonElement>(".cell");

  if (!cell) {
    return;
  }

  const row = Number(cell.dataset.row);
  const column = Number(cell.dataset.column);

  const msg: ClientMessage = { position: [row, column] };

  ws.send(JSON.stringify(msg));
});

// create grid elements
for (let i = 0; i < 3; i += 1) {
  for (let j = 0; j < 3; j += 1) {
    const cell = document.createElement("button");
    cell.type = "button";
    cell.className = "cell";
    cell.dataset.row = String(i);
    cell.dataset.column = String(j);
    board.append(cell);
  }
}

type ClientMessage = { position: [number, number] };

type ServerMessage = { board: BoardData };

const ws = new WebSocket("ws://localhost:3000/tic-tac-toe");

ws.addEventListener("open", () => console.log("Websocket opened!"));
ws.addEventListener("close", () => console.log("Websocket closed!"));
ws.addEventListener("error", () => console.error("websocket error"));
ws.addEventListener("message", (ev) => {
  console.log("websocket message", ev.data);
  const data = JSON.parse(ev.data) as ServerMessage;
  renderBoard(data.board);
});
