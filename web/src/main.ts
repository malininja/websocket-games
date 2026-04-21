const log = document.querySelector<HTMLPreElement>("#log")!;
const input = document.querySelector<HTMLInputElement>("#input")!;
const send = document.querySelector<HTMLButtonElement>("#send")!;

const ws = new WebSocket("ws://localhost:3000/ws");

ws.addEventListener("open", () => append("[open]"));
ws.addEventListener("message", (e) => append(`> ${e.data}`));
ws.addEventListener("close", () => append("[close]"));
ws.addEventListener("error", () => append("[error]"));

send.addEventListener("click", () => {
	if (ws.readyState !== WebSocket.OPEN) return;
	ws.send(input.value);
	append(`< ${input.value}`);
	input.value = "";
});

function append(line: string) {
	log.textContent += `${line}\n`;
}
