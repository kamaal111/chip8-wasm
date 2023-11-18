import * as React from "react";

type Game = { name: string; data: ArrayBuffer };

const GAMES = ["PONG"];

function GameSelector() {
  const [games, setGames] = React.useState<Array<Game>>([]);

  React.useEffect(() => {
    loadGames();
  }, []);

  async function loadGames() {
    const validGames: Array<Game> = [];
    for (const game of GAMES) {
      const response = await fetch(`games/${game}`);
      if (!response.ok) continue;

      const responseBlob = await response.blob();
      const responseBlobBuffer = await responseBlob.arrayBuffer();
      validGames.push({ name: game, data: responseBlobBuffer });
    }

    setGames(validGames);
  }

  return (
    <>
      <label>Choose a game:</label>
      <select name="games" id="games">
        {games.map(({ name }) => {
          return (
            <option value={name} key={name}>
              {name}
            </option>
          );
        })}
      </select>
    </>
  );
}

export default GameSelector;
