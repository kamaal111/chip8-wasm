import { Chip8 } from "chip8";

import { buildGameSelector, subscribeToSelectedGame } from "./gameSelector";
import { buildDisplay, drawDisplayBuffers } from "./display";

const chip8 = Chip8.new();
let animationID;

buildGameSelector(chip8);
const chip8DisplayDimensions = {
  width: chip8.get_display_width(),
  height: chip8.get_display_height(),
};
buildDisplay(chip8DisplayDimensions);
drawDisplayBuffers(chip8.get_display_buffer(), chip8DisplayDimensions);
subscribeToSelectedGame((gameName) => chip8.load_rom(gameName));

function play() {
  animationID = requestAnimationFrame((_animationFrame) => {
    while (!chip8.get_draw_flag()) {
      chip8.cycle();
    }

    drawDisplayBuffers(chip8.get_display_buffer(), chip8DisplayDimensions);
    chip8.end_cycle();
    play();
  });
}

play();
