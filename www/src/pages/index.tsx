import * as React from "react";
import Head from "next/head";
import { Chip8CPU } from "chip8";

const GAMES = ["PONG"];

async function listGames() {
  for (const game of GAMES) {
    const pong = await fetch(`games/${game}`);
    console.log("pong", pong);
  }
}

export default function Home() {
  React.useEffect(() => {
    console.log("render 🐸");
    const chip8 = Chip8CPU.new();
    console.log("chip8", chip8);
    listGames();
  }, []);

  return (
    <>
      <Head>
        <title>Chip 8</title>
        <meta name="description" content="Chip 8 emulator" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main>
        <label>Choose a game:</label>
        <select name="games" id="games">
          {GAMES.map((game) => {
            return (
              <option value={game} key={game}>
                {game}
              </option>
            );
          })}
        </select>
      </main>
    </>
  );
}
