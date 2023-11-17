import * as React from "react";

const GAMES = ["PONG"];

function GameSelector() {
  const [games, setGames] = React.useState<Array<string>>([]);

  React.useEffect(() => {
    loadGames();
  }, []);

  async function loadGames() {
    const validGames: Array<string> = [];
    for (const game of GAMES) {
      const response = await fetch(`games/${game}`);
      if (!response.ok) continue;

      validGames.push(game);
    }

    setGames(validGames);
  }

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
