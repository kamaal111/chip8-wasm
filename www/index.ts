import { Chip8CPU } from "chip8";

const games = ["PONG"];
const gamesSelect = document.getElementById("games");
if (gamesSelect == null) throw new Error("No game select found");

for (const game of games) {
  const gameOption = document.createElement("option");
  gameOption.value = game.toLowerCase();
  gameOption.innerHTML = game;
  gamesSelect.appendChild(gameOption);
}

const cpu = Chip8CPU.new();
console.log("cpu", cpu);
