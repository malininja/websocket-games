import "./tic-tac-toe.css";

const board = document.querySelector<HTMLDivElement>("#board")!;

board.addEventListener("click", (e) => {
	const cell = (e.target as HTMLElement).closest<HTMLButtonElement>(".cell");

	if (!cell) {
		return;
	}

	console.log("row =", cell.dataset.row);
	console.log("column =", cell.dataset.column);
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
