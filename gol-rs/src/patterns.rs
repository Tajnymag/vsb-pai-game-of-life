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

pub fn get_n_glider_loop() -> RuleLengthEncoded {
    RuleLengthEncoded::new_from_rle(
        Rle::new(r"#N Glider loop
#C A pattern in which two gliders are bounced back and forth along an
#C ever-lengthening track.
#C www.conwaylife.com/wiki/index.php?title=Glider_loop
x = 73, y = 150, rule = b3/s23
45bo27b$37b3o5bo27b$36bo2b2o3bobo26b$35bo5bo3bo27b$41bo3bo27b$36b2o2bo
32b$25b2o11b2o33b$25b2o46b6$61bo11b$17b2o40b2ob2o9b$17b2o42bo11b3$59b
2o12b$58bo2bo11b$51bo5bo3b2o10b$51bo5bo4bo10b$9b2o40bo6bo3bo10b$9b2o
37bob3obo3bo2bo11b$48b5obo5bo12b$48bobo22b$52b2o17bob$52b2o9b3o5bob$
52bo9bo2b2o3bobo$51b2o8bo5bo3bob$46b2o3bo15bo3bob$17b3o31bo10b2o2bo6b$
20bo29b2obo10b2o7b$15b2o4bo30bo20b$18bo3bo50b$14bo4bo2bo50b$14bo3bo3bo
50b$14b3obob2o51b$18bo54b$14b4o7b3o45b$14b2o8bo3bo44b$24bo4bo28bo14b$
26bo3bo25b2o9b2o4b$23b3obo2bo24b2o8b3obo3b$22bo7bo23bo10bo3bo3b$22bobo
3bo26b5o5bob2o4b$21b2obo3bo27bo10bo5b$21b2ob3o46b5$63b2o8b$63b2o8b4$
43b3o27b$46bo26b$41b2o4bo25b$44bo3bo6b2o16b$40bo4bo2bo6b2o16b$40bo3bo
3bo24b$40b3obob2o25b$44bo28b$40b4o29b$40b2o31b2$47b2o24b$47b2o24b2$33b
o39b$33bobo37b$33b2o38b3$38b2o33b$37bobo33b$39bo33b2$24b2o47b$24b2o47b
2$31b2o40b$29b4o40b$28bo44b$25b2obob3o40b$24bo3bo3bo40b$16b2o6bo2bo4bo
40b$16b2o6bo3bo44b$25bo4b2o41b$26bo46b$27b3o43b4$8b2o63b$8b2o63b5$46b
3ob2o21b$5bo10bo27bo3bob2o21b$4b2obo5b5o26bo3bobo22b$3bo3bo10bo23bo7bo
22b$3bob3o8b2o24bo2bob3o23b$4b2o9b2o25bo3bo26b$14bo28bo4bo24b$44bo3bo
8b2o14b$45b3o7b4o14b$54bo18b$51b2obob3o14b$50bo3bo3bo14b$50bo2bo4bo14b
$50bo3bo18b$20bo30bo4b2o15b$7b2o10bob2o29bo20b$6bo2b2o10bo31b3o17b$bo
3bo15bo3b2o46b$bo3bo5bo8b2o51b$obo3b2o2bo9bo52b$bo5b3o9b2o52b$bo17b2o
52b$22bobo48b$12bo5bob5o48b$11bo2bo3bob3obo37b2o9b$10bo3bo6bo40b2o9b$
10bo4bo5bo51b$10b2o3bo5bo51b$11bo2bo58b$12b2o59b3$11bo42b2o17b$9b2ob2o
40b2o17b$11bo61b6$46b2o25b$33b2o11b2o25b$32bo2b2o36b$27bo3bo41b$27bo3b
o5bo35b$26bobo3b2o2bo36b$27bo5b3o37b$27bo!").unwrap()
    )
}

pub fn get_searchable_patterns() -> Vec<RuleLengthEncoded> {
    vec![get_beehive_pattern(), get_glider_pattern(), get_block_pattern(), get_blinker_pattern(), get_r_pentomino(), get_n_glider_loop()]
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