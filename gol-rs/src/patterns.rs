use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, AtomicU8};
use ca_formats::{CellData, Input};
use std::sync::atomic::Ordering::Relaxed;
use ca_formats::rle::{Rle};
use std::fs::File;
use std::thread::JoinHandle;
use std::thread;
use sdl2::pixels;
use sdl2::pixels::Color;

pub struct RuleLengthEncoded {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub data: Vec<CellData>
}

impl RuleLengthEncoded {
    fn new_from_rle<I: Input>(rle: Rle<I>) -> Self {
        let mut cell_data: Vec<CellData> = Vec::new();
        let header = rle.header_data().unwrap();
        let width = header.x as u32;
        let height = header.y as u32;

        let tmp = rle.map(|cell| cell.unwrap()).collect::<Vec<CellData>>();

        for cell in tmp {
            cell_data.push(cell);
        }

        return Self {
            name: "NOT_IMPLEMENTED".parse().unwrap(),
            width,
            height,
            data: cell_data
        };
    }
}

pub fn get_beehive_pattern() -> RuleLengthEncoded {
    RuleLengthEncoded::new_from_rle(Rle::new(r"#N Beehive
#O John Conway
#C An extremely common 6-cell still life.
#C www.conwaylife.com/wiki/index.php?title=Beehive
x = 4, y = 3, rule = B3/S23
b2ob$o2bo$b2o!").unwrap())
}

pub fn get_glider_pattern() -> RuleLengthEncoded {
    RuleLengthEncoded::new_from_rle(Rle::new(r"#N Glider
#O Richard K. Guy
#C The smallest, most common, and first discovered spaceship. Diagonal, has period 4 and speed c/4.
#C www.conwaylife.com/wiki/index.php?title=Glider
x = 3, y = 3, rule = B3/S23
bob$2bo$3o!").unwrap())
}

pub fn get_block_pattern() -> RuleLengthEncoded {
    RuleLengthEncoded::new_from_rle(Rle::new(r"#N Block
#C An extremely common 4-cell still life.
#C www.conwaylife.com/wiki/index.php?title=Block
x = 2, y = 2, rule = B3/S23
2o$2o!").unwrap())
}

pub fn get_blinker_pattern() -> RuleLengthEncoded {
    RuleLengthEncoded::new_from_rle(Rle::new(r"#N Blinker
#O John Conway
#C A period 2 oscillator that is the smallest and most common oscillator.
#C www.conwaylife.com/wiki/index.php?title=Blinker
x = 3, y = 1, rule = B3/S23
3o!").unwrap())
}

pub fn get_r_pentomino() -> RuleLengthEncoded {
    RuleLengthEncoded::new_from_rle(
        Rle::new(r"#N R-pentomino
#C A methuselah with lifespan 1103.
#C www.conwaylife.com/wiki/index.php?title=R-pentomino
x = 3, y = 3, rule = B3/S23
b2o$2ob$bo!").unwrap())
}

pub fn get_searchable_patterns() -> Vec<RuleLengthEncoded> {
    vec![get_beehive_pattern(), get_glider_pattern(), get_block_pattern(), get_blinker_pattern(), get_r_pentomino()]
}

pub const PATTERN_COLORS_RGB: [Color; 8] = [
    pixels::Color::RGB(0, 0, 0),
    pixels::Color::RGB(255, 0, 0),
    pixels::Color::RGB(0, 255, 0),
    pixels::Color::RGB(0, 0, 255),
    pixels::Color::RGB(255, 255, 0),
    pixels::Color::RGB(255, 0, 255),
    pixels::Color::RGB(0, 255, 255),
    pixels::Color::RGB(255, 255, 255),
];

pub fn load_pattern_from_file(file_path: String) -> RuleLengthEncoded {
    let file = File::open(file_path).unwrap();
    return RuleLengthEncoded::new_from_rle(Rle::new_from_file(file).unwrap());
}

fn detect_patterns(from: usize, to: usize, board: &Arc<RwLock<Vec<AtomicBool>>>, board_width: u32, _board_height: u32, pattern_board: &mut Arc<RwLock<Vec<AtomicU8>>>, searched_patterns: &Arc<Vec<RuleLengthEncoded>>) -> Result<(), String> {
    let board_unlocked = board.read().unwrap();
    let pattern_board_unlocked = pattern_board.read().unwrap();
    let searched_pattern_unlocked = searched_patterns;


    'board_loop: for i in from..to {
        'pattern_loop: for (pattern_id, pattern) in searched_pattern_unlocked.iter().enumerate() {
            let pattern_start = i;

            for pattern_cell in &pattern.data {
                let (pattern_x, pattern_y) = pattern_cell.position;
                let cell_index = pattern_start + crate::utils::to_coordinate_1d(pattern_x as i32, pattern_y as i32, board_width) as usize;

                if cell_index >= board_unlocked.len() {
                    continue 'pattern_loop;
                }

                let pattern_value = pattern_cell.state == 1;
                let cell_value: bool = board_unlocked[cell_index].load(Relaxed);

                if cell_value != pattern_value {
                    continue 'pattern_loop;
                }

                if pattern_board_unlocked[cell_index].load(Relaxed) > 0 {
                    continue 'board_loop;
                }
            }

            for pattern_cell in &pattern.data {
                let (pattern_x, pattern_y) = pattern_cell.position;
                let cell_index = pattern_start + crate::utils::to_coordinate_1d(pattern_x as i32, pattern_y as i32, board_width) as usize;
                pattern_board_unlocked[cell_index].store((pattern_id + 1) as u8, Relaxed);
            }
        }
    }

    return Ok(());
}

pub fn detect_patterns_parallel(board: &Arc<RwLock<Vec<AtomicBool>>>, board_width: u32, board_height: u32, pattern_board: &mut Arc<RwLock<Vec<AtomicU8>>>, searched_patterns: &Arc<Vec<RuleLengthEncoded>>) -> Result<(), String> {
    let mut threads: Vec<JoinHandle<()>> = Vec::new();

    let cpus = num_cpus::get();

    let chunk_size = (board_width * board_height) / cpus as u32;
    let chunks_count = (board_width * board_height) / chunk_size;

    for chunk_index in 0..chunks_count {
        let from = chunk_index * chunk_size;
        let to = if chunk_index + 1 != chunks_count { from + chunk_size } else { board_width * board_height };

        let board_ref = board.clone();
        let mut pattern_board_ref = pattern_board.clone();
        let searched_patterns_ref = searched_patterns.clone();

        let thread_handle = thread::spawn(move || {
            detect_patterns(from as usize, to as usize, &board_ref, board_width, board_height, &mut pattern_board_ref, &searched_patterns_ref).unwrap();
        });

        threads.push(thread_handle);
    }

    for handle in threads {
        handle.join().map_err(|err| println!("{:?}", err)).ok();
    }

    return Ok(());
}