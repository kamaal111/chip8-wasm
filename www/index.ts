import {Chip8CPU} from "chip8";

const cpu = Chip8CPU.new()
cpu.cycle()
console.log('cpu', cpu)
