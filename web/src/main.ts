const log = document.querySelector<HTMLPreElement>("#log")!;
const input = document.querySelector<HTMLInputElement>("#input")!;
const send = document.querySelector<HTMLButtonElement>("#send")!;

const ws = new WebSocket("ws://localhost:3000/ws");

type ClientMsg = { type: "chat"; text: string };

type ServerMsg =
	| { type: "chat"; text: string }
	| { type: "system"; text: string };

ws.addEventListener("open", () => append("[open]"));
ws.addEventListener("message", (e) => {
	const msg = JSON.parse(e.data) as ServerMsg;
	switch (msg.type) {
		case "chat": {
			append(`chat: ${msg.text}`);
			break;
		}
		case "system": {
			append(msg.text);
			break;
		}
		default: {
			const exhaustive: never = msg;
			throw new Error(`Unknown message: ${JSON.stringify(exhaustive)}`);
		}
	}
});
ws.addEventListener("close", () => append("[close]"));
ws.addEventListener("error", () => append("[error]"));

send.addEventListener("click", () => {
	if (ws.readyState !== WebSocket.OPEN) return;

	const msg: ClientMsg = { type: "chat", text: input.value };
	ws.send(JSON.stringify(msg));
	input.value = "";
});

function append(line: string) {
	log.textContent += `${line}\n`;
}
