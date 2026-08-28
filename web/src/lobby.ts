import "./lobby.css";

enum LobbyError {
  AlreadyStarted = "AlreadyStarted",
  PlayerInActiveGame = "PlayerInActiveGame",
  InvalidAction = "InvalidAction",
  GameDoesntExist = "GameDoesntExist",
  InvalidMessage = "InvalidMessage",
  InvalidUser = "InvalidUser",
  Unauthorized = "Unauthorized",
}

type Player = string | null;

type Game = {
  id: string;
  players: [Player, Player];
};

type ServerMessage = {
  games: Game[];
  requested_by: string;
  username: string | null;
  error?: LobbyError;
};

enum GameAction {
  Join = "Join",
  Remove = "Remove",
}

type ClientMessage = {
  game_id?: string;
  action: GameAction;
};

const ws = new WebSocket("ws://localhost:3000/lobby");

ws.addEventListener("open", () => {
  console.log("Websocket opened.");
  init();
});
ws.addEventListener("close", (e) =>
  console.error("Websocket closed.", e.code, e.wasClean),
);
ws.addEventListener("error", () => console.error("Websocket error."));
ws.addEventListener("message", (ev) => {
  console.log("websocket message", ev.data);
  const data = JSON.parse(ev.data) as ServerMessage;

  if (data.error) {
    console.error(`error: ${data.error}`);

    if (data.error === LobbyError.Unauthorized) {
      window.location.href = "/login";
      return;
    }

    return;
  }

  if (isPlayerInGame(data.username, data.games)) {
    window.location.href = "/tic-tac-toe";
    return;
  }

  renderGames(data.username, data.games);
});

function init() {
  const createButton = document.querySelector(
    "#create-game",
  )! as HTMLButtonElement;
  createButton.addEventListener("click", () => {
    const message: ClientMessage = { action: GameAction.Join };
    ws.send(JSON.stringify(message));
  });
}

function isPlayerInGame(playerName: string | null, games: Game[]): boolean {
  if (!playerName) {
    return false;
  }

  for (const game of games) {
    if (
      game.players.filter((i) => i !== null).length === 2 &&
      game.players.includes(playerName)
    ) {
      return true;
    }
  }

  return false;
}

function renderGames(playerName: string | null, games: Game[]) {
  if (!playerName) {
    return;
  }

  const availableGamesSection = document.querySelector(
    "#available-games",
  )! as HTMLDivElement;
  availableGamesSection.replaceChildren();

  const inprogressGamesSection = document.querySelector(
    "#inprogress-games",
  )! as HTMLDivElement;
  inprogressGamesSection.replaceChildren();

  for (const game of games) {
    const card = createGameCard(playerName, game);

    if (isJoinable(game)) {
      availableGamesSection.append(card);
    } else {
      inprogressGamesSection.append(card);
    }
  }
}

function createGameCard(playerName: string, game: Game): HTMLDivElement {
  const joinable = isJoinable(game);

  const card = document.createElement("div");
  card.className = `game-card ${joinable ? "joinable" : "inprogress"}`;

  const icon = document.createElement("div");
  icon.className = "game-icon";
  icon.textContent = "♟";

  const playersText = document.createElement("span");
  playersText.className = "game-players";
  playersText.textContent = game.players[0];

  if (game.players[1]) {
    playersText.textContent += ` vs ${game.players[1]}`;
  }

  card.append(icon, playersText);

  if (joinable) {
    const removeButton = document.createElement("button");
    removeButton.className = "remove-btn";
    removeButton.title = "Remove game";

    if (game.players[0] === playerName) {
      removeButton.textContent = "X";
    }

    removeButton.addEventListener("click", (e) => {
      e.stopPropagation();

      const msg: ClientMessage = {
        game_id: game.id,
        action: GameAction.Remove,
      };
      ws.send(JSON.stringify(msg));
    });

    card.append(removeButton);

    card.addEventListener("click", () => {
      const msg: ClientMessage = { game_id: game.id, action: GameAction.Join };
      ws.send(JSON.stringify(msg));
    });
  }

  return card;
}

function isJoinable(game: Game): boolean {
  return game.players.filter((p) => p !== null).length === 1;
}
