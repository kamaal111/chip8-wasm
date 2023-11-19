const gamesSelect = document.getElementById("games");

export function buildGameSelector(chip8) {
  const games = chip8.get_game_names();
  for (const gameName of games) {
    const gameOption = document.createElement("option");
    gameOption.value = gameName;
    gameOption.innerHTML = gameName;
    gamesSelect.appendChild(gameOption);
  }

  chip8.load_rom(games[0]);
}

export function subscribeToSelectedGame(callback) {
  gamesSelect.addEventListener("change", (event) => {
    for (const option of event.target) {
      if (!option.selected) continue;

      callback(option.value);
    }
  });
}
