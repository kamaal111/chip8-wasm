const BUFFER_SIZE = 10; // px
const GRID_COLOR = "#CCCCCC";
const ON_BUFFER_COLOR = "#000000";
const OFF_BUFFER_COLOR = "#FFFFFF";

const chip8Display = document.getElementById("chip8-display");
const chip8DisplayContext = chip8Display.getContext("2d");

function drawDisplay({ width, height }) {
  chip8DisplayContext.beginPath();
  chip8DisplayContext.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i < width; i += 1) {
    const xMovement = i * (BUFFER_SIZE + 1) + 1;
    chip8DisplayContext.moveTo(xMovement, 0);
    chip8DisplayContext.lineTo(xMovement, (BUFFER_SIZE + 1) * height + 1);
  }

  // Horizontal lines.
  for (let i = 0; i <= height; i += 1) {
    chip8DisplayContext.moveTo(0, i * (BUFFER_SIZE + 1) + 1);
    chip8DisplayContext.lineTo(
      (BUFFER_SIZE + 1) * width + 1,
      i * (BUFFER_SIZE + 1) + 1
    );
  }

  chip8DisplayContext.stroke();
}

function getDisplayBufferIndex(row, column, width) {
  return row * width + column;
}

export function drawDisplayBuffers(displayBuffer, { width, height }) {
  chip8DisplayContext.beginPath();
  for (let row = 0; row < height; row += 1) {
    for (let column = 0; column < width; column += 1) {
      const index = getDisplayBufferIndex(row, column, width);
      const buffer = displayBuffer[index];
      if (buffer === 0) {
        chip8DisplayContext.fillStyle = OFF_BUFFER_COLOR;
      } else {
        chip8DisplayContext.fillStyle = ON_BUFFER_COLOR;
      }
      chip8DisplayContext.fillRect(
        column * (BUFFER_SIZE + 1) + 1,
        row * (BUFFER_SIZE + 1) + 1,
        BUFFER_SIZE,
        BUFFER_SIZE
      );
    }
  }

  chip8DisplayContext.stroke();
}

export function buildDisplay({ width, height }) {
  const displayWidth = (BUFFER_SIZE + 1) * width + 1;
  const displayHeight = (BUFFER_SIZE + 1) * height + 1;

  chip8Display.width = displayWidth;
  chip8Display.height = displayHeight;

  drawDisplay({ width, height });
}
