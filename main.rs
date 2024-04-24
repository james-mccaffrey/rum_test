use std::env;
use std::time::Instant;
use std::io::stdin;
use std::io;
use std::io::Read;
use std::io::Write;

//got this from the rumdump lab
pub fn load(input: Option<&str>) -> Vec<u32> {
    let mut raw_reader: Box<dyn std::io::BufRead> = match input {
        None => Box::new(std::io::BufReader::new(std::io::stdin())),
        
        Some(filename) => {
            match std::fs::File::open(filename) {
                Err(_) => std::process::exit(1),
                Ok(val) =>Box::new(std::io::BufReader::new(val,)),
            }
        }
    };
    let mut buf = Vec::<u8>::new();
    raw_reader.read_to_end(&mut buf).unwrap();
    let instructions: Vec<u32> = buf.chunks_exact(4).map(|x| u32::from_be_bytes(x.try_into().unwrap())).collect();
    instructions
}

fn main() {
    let input = env::args().nth(1);
    let mut regs:[u32; 8] = [0,0,0,0,0,0,0,0];
    let mut memory:Vec<Vec<u32>> = vec![load(input.as_deref())];
    let mut unmapped_memory: Vec<u32> = vec![];
    let mut program_counter: usize = 0;
    let now = Instant::now();
    loop{
        let instruction = memory[0][program_counter]; 
        program_counter+=1;

        let op = (instruction >> 28) as u8;
        let ra = ((instruction >> 6) & 0b111) as u8;
        let rb = ((instruction >> 3) & 0b111) as u8;
        let rc = (instruction & 0b111) as u8;

        match op {
            0 => {
                if regs[rc as usize] != 0 {
                    regs[ra as usize] = regs[rb as usize];
                }
            },
            1 => regs[ra as usize] = memory[regs[rb as usize] as usize][regs[rc as usize] as usize],
            2 => memory[regs[ra as usize] as usize][regs[rb as usize] as usize] = regs[rc as usize],//mem.update_word(seg[ra as usize], seg[rb as usize], seg[rc as usize]),
            3 => regs[ra as usize] = regs[rb as usize].wrapping_add(regs[rc as usize]),//Register::add(&mut regs, ra, rb,  rc), 
            4 => regs[ra as usize] = regs[rb as usize].wrapping_mul(regs[rc as usize]),
            5 => regs[ra as usize] = regs[rb as usize] / regs[rc as usize],
            6 => regs[ra as usize] = !(regs[rb as usize] & regs[rc as usize]),
            7 => {
                let elapsed = now.elapsed();
                eprintln!("{:.2?}", elapsed);
                std::process::exit(0);
            },
            8 => {
                match unmapped_memory.pop() {
                    Some(value) => {
                        memory[value as usize] = vec![0; regs[rc as usize] as usize];
                        regs[rb as usize] = value as u32;
                    }
                    None => {
                        //unmapped_memory.push(memory.len() as u32 );
                        memory.push(vec![0; regs[rc as usize] as usize]);
                        regs[rb as usize] = memory.len() as u32 -1;
                    }
                }
            },
                    
            9 => unmapped_memory.push(regs[rc as usize]),
            10 => {
                io::stdout().write(&[regs[rc as usize] as u8]).unwrap();
                io::stdout().flush().unwrap();
            },
            11 => regs[rc as usize] = stdin().bytes().next().unwrap().unwrap() as u32,
            12 => {
                if regs[rb as usize] != 0 {
                    memory[0] = memory[regs[rb as usize] as usize].clone();
                }
                program_counter = regs[rc as usize] as usize;
            }, 
            13 => {
                let rl = ((instruction << 4) >> 29) as u8;
                let vl = (instruction << 7) >> 7; 
                regs[rl as usize] = vl;
            },
            _ => println!("INVALID INSTRUCTION")
        }
    }
}