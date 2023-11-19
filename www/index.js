import { Chip8 } from "chip8";

import { buildGameSelector } from "./gameSelector";
import { buildDisplay } from "./display";

const chip8 = Chip8.new();

buildGameSelector(chip8);
buildDisplay(chip8);
