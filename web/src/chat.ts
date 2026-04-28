const log = document.querySelector<HTMLPreElement>("#log")!;
const roomInput = document.querySelector<HTMLInputElement>("#room_input")!;
const roomSend = document.querySelector<HTMLButtonElement>("#room_send")!;
const chatInput = document.querySelector<HTMLInputElement>("#chat_input")!;
const chatSend = document.querySelector<HTMLButtonElement>("#chat_send")!;

const ws = new WebSocket("ws://localhost:3000/ws");

type ClientMsg =
	| { type: "chat"; text: string }
	| { type: "join"; room: string };

type ServerMsg =
	| { type: "chat"; text: string }
	| { type: "system"; text: string }
	| { type: "error"; text: string };

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
		case "error": {
			append(`error: ${msg.text}`);
			break;
		}
		default: {
			const exhaustive: never = msg;
			const errMsg = `Unknown message: ${JSON.stringify(exhaustive)}`;
			append(`error: ${errMsg}`);
			throw new Error(errMsg);
		}
	}
});
ws.addEventListener("close", () => append("[close]"));
ws.addEventListener("error", () => append("[error]"));

roomSend.addEventListener("click", () => {
	if (ws.readyState !== WebSocket.OPEN) return;

	const msg: ClientMsg = { type: "join", room: roomInput.value };
	ws.send(JSON.stringify(msg));
});

chatSend.addEventListener("click", () => {
	if (ws.readyState !== WebSocket.OPEN) return;

	const msg: ClientMsg = { type: "chat", text: chatInput.value };
	ws.send(JSON.stringify(msg));
	chatInput.value = "";
});

function append(line: string) {
	log.textContent += `${line}\n`;
}
