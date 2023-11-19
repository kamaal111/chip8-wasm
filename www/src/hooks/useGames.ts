import * as React from "react";
import { type Chip8Emulator } from "chip8";

function useGames(emulator: Chip8Emulator) {
  const [games, setGames] = React.useState<Array<string>>([]);
  const [selectedGame, setSelectedGame] = React.useState<string | null>(null);

  React.useEffect(() => {
    const games = emulator.get_game_names();
    if (games.length > 0) {
      setGames(games);
    }
  }, []);

  React.useEffect(() => {
    if (games.length > 0 && selectedGame == null) {
      setSelectedGame(games[0]);
    } else if (games.length === 0 && selectedGame != null) {
      setSelectedGame(null);
    }
  }, [games]);

  React.useEffect(() => {
    if (selectedGame != null) {
      try {
        emulator.load_rom(selectedGame);
      } catch (error) {
        setGames(games.filter((game) => game != selectedGame));
        // TODO: Handle error to user
      }
    }
  }, [selectedGame]);

  return { games, selectedGame };
}

export default useGames;
