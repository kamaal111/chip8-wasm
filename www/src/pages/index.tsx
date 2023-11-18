import { Chip8CPU } from "chip8";
import Head from "next/head";

import GameSelector from "@/components/GameSelector";
import Chip8Display from "@/components/Chip8Display";

import styles from "@/styles/pages/home.module.css";

const chip8 = Chip8CPU.new();

export default function Home() {
  return (
    <>
      <Head>
        <title>Chip 8</title>
        <meta name="description" content="Chip 8 emulator" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main>
        <div className={styles.home}>
          <GameSelector />
          <Chip8Display chip8CPU={chip8} />
        </div>
      </main>
    </>
  );
}
