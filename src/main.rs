use std::fs::File;
use std::io::Read;
use std::io::prelude::*;
use std::env;

mod display;
use display::Display;

const CHIP8_FONTSET:[u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

fn main() {
    // setupGraphics();
    // setupInput();

    let mut main_chip: Chip = Chip::init();

    main_chip.load_game("pong");

    // main_chip.emulate_cycle();
}

struct Chip {
    op_code: u16,
    memory: [u8; 4096],
    V: [u8; 16],
    index_register: usize,
    program_counter: usize,
    stack: [u16; 16],
    stack_pointer: usize,
    delay_timer: u8,
    sound_timer: u8,
    key: [u16; 16],
    pub display: Box<Display>
}

impl Chip {

    fn init() -> Chip {
        let loaded_memory = Chip::load_fontset();
        Chip {
            op_code: 0,
            program_counter: 0x200,
            index_register: 0x200,
            memory: loaded_memory,
            V: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            key: [0; 16],
            stack: [0; 16],
            display: Box::new(Display::new())
        }
    }

    fn load_fontset() -> [u8; 4096] {
        let mut loaded_memory = [0u8; 4096];
        for i in 0..80 {
            loaded_memory[i] = CHIP8_FONTSET[i];
        }
        loaded_memory
    }
    fn emulate_cycle(&mut self) {
        // Fetch Opcode
        self.fetch_opcode();
        self.execute_opcode();
        // Decode Opcode

        // Execute Opcode

        // Update Timers
    }

    fn load_game(&mut self, game_name: &'static str) {
        let mut path = env::current_dir().unwrap();
        path.push(game_name.trim());
        let mut reader = File::open(&path).unwrap();
        let mut game_data = Vec::new();
        reader.read_to_end(&mut game_data).expect("Failure to read file");

        self.load_to_memory(&mut game_data);
    }

    fn load_to_memory(&mut self, game_data: &mut Vec<u8>) {

        for (i, byte) in game_data.iter().enumerate() {
            self.memory[self.program_counter] = byte.clone();
            self.program_counter += 1;
        }

        self.program_counter = 0x200;
    }

    fn fetch_opcode(&mut self) {
        self.op_code = (self.memory[self.program_counter] as u16) << 8 | (self.memory[self.program_counter + 1] as u16);
    }

    fn execute_opcode(&mut self) {
        match self.op_code & 0xf000 {
            0x0000 => {
                match self.op_code & 0x000F {
                    0x0000 => {
                        // Clear Display 
                    },
                    0x000E => {
                        self.stack_pointer -= 1;
                        self.program_counter = self.stack[self.stack_pointer] as usize;
                    },
                    _ => {
                        println!("Not Implemented: {}", self.op_code);
                    }
                }
            },
            0x1000 => {
                // Jump to Address
                self.program_counter = self.op_nnn() as usize;
            },
            0x2000 => {
                // Call Subroutine
                self.stack[self.stack_pointer] = self.program_counter as u16;
                self.stack_pointer += 1;
                self.program_counter = self.op_nnn() as usize;
            },
            0x3000 => {
                // Skips next instruction if VX == NN
                self.program_counter += if self.V[self.op_x()] == self.op_nn() { 4 } else { 2 }
            },
            0x4000 => {
                // Skips next instruction if VX != NN
                self.program_counter += if self.V[self.op_x()] != self.op_nn() { 4 } else { 2 }
            },
            0x5000 => {
                // Skips next instruction if VX equals VY
                self.program_counter += if self.V[self.op_x()] == self.V[self.op_y()] { 4 } else { 2 }
            },
            0x6000 => {
                // Set VX to NN
                self.V[self.op_x()] = self.op_nn();
                self.program_counter += 2;
            },
            0x7000 => {
                // Add NN to VX
                self.V[self.op_x()] += self.op_nn();
                self.program_counter += 2;
            },
            0x8000 => {
                match self.op_code & 0x000F {
                    0 => {
                        // Move
                    },
                    1 => {
                        // OR
                    },
                    2 => {
                        // AND
                    },
                    3 => {
                        // Xor
                    },
                    4 => {
                        // Add
                    },
                    5 => {
                        // Sub
                    },
                    6 => {
                        // Shift Right
                    },
                    7 => {
                        //Reverse Sub
                    },
                    0xE => {
                        // Shift Left
                    },
                    _ => {
                        println!("Unrecognized op_code: {}", self.op_code);
                    }
                }
            },
            0x9000 => {
                // Skip if VX != VY
                self.program_counter += if self.V[self.op_x()] != self.V[self.op_y()] { 4 } else { 2 }
            },
            0xA000 => {
                //Load index_counter
            },
            0xB000 => {
                //Jump + Zero
            },
            0xC000 => {
                // Random
            },
            0xD000 => {
                // Draw
            },
            0xE000 => {
                let V = self.V[self.op_x()] as usize;
                self.program_counter += match self.op_code & 0x00FF {
                    0x9E => {
                        // Skip if Pressed
                        0
                    },
                    0xA1 => {
                        // Skip if not Pressed
                        0
                    },
                    _ => {
                        println!("op_code not recognized: {}", self.op_code);
                        0
                    }
                }
            },
            0xF000 => {
                match self.op_code & 0x00FF {
                    0x07 => {
                        // Load Delay Timer
                    },
                    0x0A => {
                        // Wait for Keypress
                    },
                    0x15 => {
                        // Set delay_timer
                    },
                    0x18 => {
                        // Set sound_timer
                    },
                    0x1E => {
                        // Add to index_counter
                    },
                    0x29 => {
                        // Load sprite
                    },
                    0x33 => {
                        // BCD Representation
                    },
                    0x55 => {
                        // Store Registers
                    },
                    0x65 => {
                        // Load Registers
                    },
                    _ => {
                        println!("Unknown op_code: {}", self.op_code);
                    }
                }
            },
            _ => {
                println!("Op_code doesn't exist: {}", self.op_code);
            }
        }
    }

    fn op_x(&self) -> usize {
        ((self.op_code & 0x0F00) >> 8) as usize
    }
    fn op_y(&self) -> usize {
        ((self.op_code & 0x00F0) >> 4) as usize
    }
    fn op_n(&self) -> u8 {
        (self.op_code & 0x000F) as u8
    }
    fn op_nn(&self) -> u8 {
        (self.op_code & 0x00FF) as u8
    }
    fn op_nnn(&self) -> u16 {
        self.op_code & 0x0FFF
    }

}
