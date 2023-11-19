export function buildGameSelector(chip8) {
  const games = chip8.get_game_names();
  const gamesSelect = document.getElementById("games");
  if (gamesSelect == null) throw new Error("No game select found");

  for (const game of games) {
    const gameOption = document.createElement("option");
    gameOption.value = game.toLowerCase();
    gameOption.innerHTML = game;
    gamesSelect.appendChild(gameOption);
  }
}
