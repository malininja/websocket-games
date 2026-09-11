import "./login.css";
import config from "./config";

const usernameInput = document.querySelector("input")! as HTMLInputElement;
const loginButton = document.querySelector("button")! as HTMLButtonElement;

loginButton.addEventListener("click", async () => {
  const username = usernameInput.value.trim();

  if (!username) {
    return;
  }

  try {
    const response = await fetch(`http://${config.apiUrl}/login`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ username }),
      credentials: "include",
    });

    if (response.ok) {
      window.location.href = "/lobby";
    } else {
      alert("Login error");
    }
  } catch (error) {
    console.error("Login error:", error);
    alert("Login error");
  }
});
