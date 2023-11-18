import * as React from "react";
import { type Chip8Emulator } from "chip8";

function GameSelector({ emulator }: { emulator: Chip8Emulator }) {
  const [games, setGames] = React.useState<Array<string>>([]);

  React.useEffect(() => {
    setGames(emulator.get_game_names());
  }, []);

  return (
    <>
      <label>Choose a game:</label>
      <select name="games" id="games">
        {games.map((game) => {
          return (
            <option value={game} key={game}>
              {game}
            </option>
          );
        })}
      </select>
    </>
  );
}

export default GameSelector;
