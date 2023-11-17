import * as React from "react";
import Head from "next/head";
import { Chip8CPU } from "chip8";

import GameSelector from "@/components/GameSelector";

export default function Home() {
  React.useEffect(() => {
    const chip8 = Chip8CPU.new();
    console.log("chip8", chip8);
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
        <GameSelector />
      </main>
    </>
  );
}
