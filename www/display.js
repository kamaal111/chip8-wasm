const BUFFER_SIZE = 10; // px
const GRID_COLOR = "#CCCCCC";
const ON_BUFFER_COLOR = "#000000";
const OFF_BUFFER_COLOR = "#FFFFFF";

function drawDisplay(context, { width, height }) {
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

function getDisplayBufferIndex(row, column, width) {
  return row * width + column;
}

function drawDisplayBuffers(context, displayBuffer, { width, height }) {
  context.beginPath();
  for (let row = 0; row < height; row += 1) {
    for (let column = 0; column < width; column += 1) {
      const index = getDisplayBufferIndex(row, column, width);
      const buffer = displayBuffer[index];
      if (buffer === 0) {
        context.fillStyle = OFF_BUFFER_COLOR;
      } else {
        context.fillStyle = ON_BUFFER_COLOR;
      }
      context.fillRect(
        column * (BUFFER_SIZE + 1) + 1,
        row * (BUFFER_SIZE + 1) + 1,
        BUFFER_SIZE,
        BUFFER_SIZE
      );
    }
  }

  context.stroke();
}

export function buildDisplay(chip8) {
  const chip8Display = document.getElementById("chip8-display");
  if (chip8Display == null) throw new Error("No chip 8 display found");

  const chip8DisplayContext = chip8Display.getContext("2d");
  const chip8DisplayDimensions = {
    width: chip8.get_display_width(),
    height: chip8.get_display_height(),
  };

  const displayWidth = (BUFFER_SIZE + 1) * chip8DisplayDimensions.width + 1;
  const displayHeight = (BUFFER_SIZE + 1) * chip8DisplayDimensions.height + 1;

  chip8Display.width = displayWidth;
  chip8Display.height = displayHeight;

  drawDisplay(chip8DisplayContext, chip8DisplayDimensions);
  const displayBuffer = chip8.get_display_buffer();
  drawDisplayBuffers(
    chip8DisplayContext,
    displayBuffer,
    chip8DisplayDimensions
  );
}
