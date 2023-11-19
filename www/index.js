import { Chip8 } from "chip8";

import { buildGameSelector, subscribeToSelectedGame } from "./gameSelector";
import { buildDisplay } from "./display";

const chip8 = Chip8.new();

buildGameSelector(chip8);
buildDisplay(chip8);
subscribeToSelectedGame((gameName) => chip8.load_rom(gameName));
