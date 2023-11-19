import { type Chip8Emulator } from "chip8";

import useGames from "@/hooks/useGames";

function GameSelector({ emulator }: { emulator: Chip8Emulator }) {
  const { games, selectedGame } = useGames(emulator);

  return (
    <>
      <label>Choose a game:</label>
      <select name="games" value={selectedGame ?? undefined}>
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
