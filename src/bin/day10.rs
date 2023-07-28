use std::{fs::File, io::{BufReader, BufRead}};

#[derive(sscanf::FromScanf, Debug)]
enum CpuCommand {
    #[sscanf(format = "addx {0}")]
    AddX(i32),
    #[sscanf("noop")]
    Noop
}

struct CpuState {
    x_reg: i32,
    cycle_count: usize,
}

impl CpuState {
    fn process_command(&mut self, command: &CpuCommand) {
        match command {
            CpuCommand::AddX(val) => {
                self.cycle_count += 1;
                self.print_pixel();
                self.cycle_count += 1;
                self.print_pixel();
                self.x_reg += val;

            },
            CpuCommand::Noop => {
                self.cycle_count += 1;
                self.print_pixel();
            }
        }
    }

    fn print_pixel(&self) {
        if self.sprite_is_visible() {
            print!("#");
        } else {
            print!(".");
        }

        if self.cycle_count % 40 == 0 {
            println!();
        }
    }

    fn sprite_is_visible(&self) -> bool {
        let pos = (self.cycle_count % 40) as i32 - 1;
        (pos - self.x_reg).abs() < 2
    }
}

impl Default for CpuState {
    fn default() -> CpuState {
        CpuState { x_reg: 1, cycle_count: 0 }
    }
}

fn main() {
    let file = File::open("data/day10-full.txt").expect("Could not find date file");
    let reader = BufReader::new(file);

    let mut cpu = CpuState::default();
    for line in reader.lines() {
        let line = line.unwrap();

        let command = sscanf::sscanf!(line, "{CpuCommand}");
        let command = command.unwrap();
        cpu.process_command(&command);
    }
}