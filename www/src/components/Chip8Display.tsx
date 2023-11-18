import * as React from "react";
import { type Chip8CPU } from "chip8";

import styles from "@/styles/components/chip8Display.module.css";

const BUFFER_SIZE = 10; // px
const GRID_COLOR = "#CCCCCC";

function drawDisplay(
  context: CanvasRenderingContext2D,
  { width, height }: { width: number; height: number }
) {
  context.beginPath();
  context.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i < width; i += 1) {
    const xMovement = i * (BUFFER_SIZE + 1) + 1;
    context.moveTo(xMovement, 0);
    context.lineTo(xMovement, (BUFFER_SIZE + 1) * height + 1);
  }

  // Horizontal lines.
  for (let i = 0; i <= height; i += 1) {
    context.moveTo(0, i * (BUFFER_SIZE + 1) + 1);
    context.lineTo((BUFFER_SIZE + 1) * width + 1, i * (BUFFER_SIZE + 1) + 1);
  }

  context.stroke();
}

function Chip8Display({ chip8CPU }: { chip8CPU: Chip8CPU }) {
  const chip8DisplayRef = React.useRef<HTMLCanvasElement | null>(null);

  const chip8DisplayDimensions = {
    width: chip8CPU.get_display_width(),
    height: chip8CPU.get_display_height(),
  };

  React.useEffect(() => {
    const chip8DisplayCanvas = chip8DisplayRef.current;
    const chip8DisplayCanvasContext = chip8DisplayCanvas?.getContext("2d");
    if (chip8DisplayCanvasContext) {
      drawDisplay(chip8DisplayCanvasContext, chip8DisplayDimensions);
    }
  }, []);

  const displayWidth = (BUFFER_SIZE + 1) * chip8DisplayDimensions.width + 1;
  const displayHeight = (BUFFER_SIZE + 1) * chip8DisplayDimensions.height + 1;

  return (
    <div>
      <canvas
        className={styles.displayCanvas}
        ref={chip8DisplayRef}
        width={displayWidth}
        height={displayHeight}
      />
    </div>
  );
}

export default Chip8Display;
