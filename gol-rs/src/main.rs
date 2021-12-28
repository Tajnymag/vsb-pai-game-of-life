extern crate sdl2;

use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, AtomicU8};
use std::sync::atomic::Ordering::Relaxed;
use std::{env, thread};
use std::thread::JoinHandle;
use sdl2::event::{Event, WindowEvent};
use sdl2::pixels;
use sdl2::pixels::Color;

use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::patterns::{get_searchable_patterns, PATTERN_COLORS_RGB};

mod patterns;
mod utils;

const DEFAULT_BOARD_WIDTH: u32 = 128;
const DEFAULT_BOARD_HEIGHT: u32 = 128;

const CELL_SIZE: u32 = 8;

const DEFAULT_CANVAS_WIDTH: u32 = DEFAULT_BOARD_WIDTH * CELL_SIZE;
const DEFAULT_CANVAS_HEIGHT: u32 = DEFAULT_BOARD_HEIGHT * CELL_SIZE;

fn num_of_neighbors(cell_index: i32, board: &Arc<RwLock<Vec<AtomicBool>>>, board_width: u32, board_height: u32) -> Result<i32, String> {
    let mut neighbors_count = 0;

    let neighbor_offsets: [(i32, i32); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

    for (offset_y, offset_x) in neighbor_offsets {
        let (cell_x, cell_y) = utils::to_coordinate_2d(cell_index, board_width);

        let neighbor_x: i32 = cell_x + offset_x;
        let neighbor_y: i32 = cell_y + offset_y;

        if neighbor_x < 0 || neighbor_x >= board_width as i32 { continue }
        if neighbor_y < 0 || neighbor_y >= board_height as i32 { continue }

        let neighbor_index = utils::to_coordinate_1d(neighbor_x, neighbor_y, board_width);

        if board.read().unwrap()[neighbor_index as usize].load(Relaxed) {
            neighbors_count += 1;
        }
    }

    return Ok(neighbors_count);
}

fn play_round(from: usize, to: usize, source_board: &Arc<RwLock<Vec<AtomicBool>>>, target_board: &mut Arc<RwLock<Vec<AtomicBool>>>, board_width: u32, board_height: u32) -> Result<(), String> {
    for cell_index in from..to {
        let mut cell_alive = source_board.read().unwrap()[cell_index].load(Relaxed);
        let neighbors_count = num_of_neighbors(cell_index as i32, source_board, board_width, board_height).unwrap();

        if cell_alive && neighbors_count < 2 {
            cell_alive = false;
        } else if cell_alive && neighbors_count <= 3 {
            cell_alive = true;
        } else if cell_alive && neighbors_count > 3 {
            cell_alive = false;
        } else if !cell_alive && neighbors_count == 3 {
            cell_alive = true;
        }

        target_board.read().unwrap()[cell_index].store(cell_alive, Relaxed);
    }

    return Ok(());
}

fn play_round_parallel(source_board: &Arc<RwLock<Vec<AtomicBool>>>, target_board: &mut Arc<RwLock<Vec<AtomicBool>>>, board_width: u32, board_height: u32) -> Result<(), String> {
    let mut threads: Vec<JoinHandle<Result<(), String>>> = Vec::new();

    let cpus = num_cpus::get();

    let chunk_size = (board_width * board_height) / cpus as u32;
    let chunks_count = (board_width * board_height) / chunk_size;

    for chunk_index in 0..chunks_count {
        let from = chunk_index * chunk_size;
        let to = if chunk_index + 1 != chunks_count { from + chunk_size } else { board_width * board_height };

        let source_board_ref = source_board.clone();
        let mut target_board_ref = target_board.clone();

        let thread_handle = thread::spawn(move || {
            play_round(from as usize, to as usize, &source_board_ref, &mut target_board_ref, board_width, board_height)
        });

        threads.push(thread_handle);
    }

    for handle in threads {
        handle.join().map_err(|err| println!("{:?}", err)).ok();
    }

    return Ok(());
}

fn render_board(board: &Arc<RwLock<Vec<AtomicBool>>>, board_width: u32, _board_height: u32, found_patterns: &Arc<RwLock<Vec<AtomicU8>>>, canvas: &mut WindowCanvas) -> Result<(), String> {
    let board_unlocked = board.read().unwrap();
    let found_patterns_unlocked = found_patterns.read().unwrap();

    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();

    for cell_index in 0..board_unlocked.len() {
        let cell_alive = board_unlocked[cell_index].load(Relaxed);
        let cell_color = if cell_alive {
            let pattern_id = &found_patterns_unlocked[cell_index].load(Relaxed);

            if *pattern_id > 0 {
                let color: &Color = &PATTERN_COLORS_RGB[*pattern_id as usize];
                pixels::Color::RGB(color.r, color.g, color.b)
            } else {
                pixels::Color::RGB(255, 255, 255)
            }
        } else {
            pixels::Color::RGB(0, 0, 0)
        };

        let (x, y) = utils::to_coordinate_2d(cell_index as i32, board_width);

        canvas.set_draw_color(cell_color);
        canvas.fill_rect(Rect::new((x * CELL_SIZE as i32) as i32, (y * CELL_SIZE as i32) as i32, CELL_SIZE, CELL_SIZE))?;
    }

    canvas.present();

    return Ok(());
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys
        .window(
            "VŠB PAI - Game of Life",
            DEFAULT_CANVAS_WIDTH,
            DEFAULT_CANVAS_HEIGHT
        )
        .position_centered()
        .resizable()
        .allow_highdpi()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut board_width = DEFAULT_CANVAS_WIDTH / CELL_SIZE;
    let mut board_height = DEFAULT_CANVAS_HEIGHT / CELL_SIZE;

    let mut board_a: Arc<RwLock<Vec<AtomicBool>>> = Arc::new(RwLock::new(Vec::new()));
    let mut board_b: Arc<RwLock<Vec<AtomicBool>>> = Arc::new(RwLock::new(Vec::new()));
    let mut pattern_board: Arc<RwLock<Vec<AtomicU8>>> = Arc::new(RwLock::new(Vec::new()));

    let searched_patterns = Arc::new(get_searchable_patterns());

    board_a.write().unwrap().resize_with((board_width * board_height) as usize, AtomicBool::default);
    board_b.write().unwrap().resize_with((board_width * board_height) as usize, AtomicBool::default);
    pattern_board.write().unwrap().resize_with((board_width * board_height) as usize, AtomicU8::default);

    let seed_file_path = env::args().skip(1).next().unwrap_or(String::from("patterns/n_glider_loop.rle"));

    let seed = patterns::load_pattern_from_file(seed_file_path);

    let seed_start_x = utils::clamp((board_width as i32 - seed.width as i32) / 2, 0, board_width as i32 - 1);
    let seed_start_y = utils::clamp((board_height as i32 - seed.height as i32) / 2, 0, board_height as i32 - 1);
    for pattern_cell in &seed.data {
        let (x, y) = pattern_cell.position;
        let board_cell_index = utils::to_coordinate_1d(seed_start_x as i32, seed_start_y as i32, board_width) + utils::to_coordinate_1d(x as i32, y as i32, board_width);
        if board_cell_index >= (board_width * board_height) as i32 {
            continue;
        }
        board_a.write().unwrap()[board_cell_index as usize].store(pattern_cell.state == 1, Relaxed);
    }

    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut events = sdl_context.event_pump()?;
    let timer = sdl_context.timer()?;
    let mut last_tick = 0;

    let mut iteration = 0;

    'main: loop {
        let delta = timer.ticks() - last_tick;

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,

                Event::Window { win_event, .. } => {
                    if let WindowEvent::Resized(w, h) = win_event {
                        let old_board_width = board_width;
                        let old_board_height = board_height;

                        board_width = (w as u32) / CELL_SIZE;
                        board_height = (h as u32) / CELL_SIZE;

                        let mut board_a_unlocked = board_a.write().unwrap();
                        let mut board_b_unlocked = board_b.write().unwrap();
                        let mut patterns_unlocked = pattern_board.write().unwrap();

                        let mut old_board_a = Vec::new();
                        let mut old_board_b = Vec::new();

                        for cell_a in board_a_unlocked.iter() {
                            old_board_a.push(cell_a.load(Relaxed));
                        }
                        for cell_b in board_b_unlocked.iter() {
                            old_board_b.push(cell_b.load(Relaxed));
                        }

                        if old_board_width != board_width || old_board_height != board_height {
                            patterns_unlocked.resize_with((board_width * board_height) as usize, AtomicU8::default);

                            board_a_unlocked.clear();
                            board_b_unlocked.clear();

                            for i in 0..(board_width * board_height) {
                                let (x, y) = utils::to_coordinate_2d(i as i32, board_width);
                                let old_board_index = utils::to_coordinate_1d(x as i32, y as i32, old_board_width) as usize;

                                if old_board_index < old_board_a.len() && old_board_index < old_board_b.len() && x < old_board_width as i32 && y < old_board_height as i32 {
                                    board_a_unlocked.push(AtomicBool::new(old_board_a[old_board_index]));
                                    board_b_unlocked.push(AtomicBool::new(old_board_b[old_board_index]));
                                } else {
                                    board_a_unlocked.push(AtomicBool::new(false));
                                    board_b_unlocked.push(AtomicBool::new(false));
                                }
                            }
                        }

                        debug_assert_eq!(board_a_unlocked.len(), (board_width * board_height) as usize);
                        debug_assert_eq!(board_b_unlocked.len(), (board_width * board_height) as usize);

                        canvas.clear();
                        canvas.present();
                    }
                }

                _ => {}
            }
        }

        if iteration == 0 {
            render_board(&board_a, board_width, board_height, &pattern_board, &mut canvas)?;
        }

        if delta <= 1000 / 60 {
            continue 'main;
        }

        if iteration % 2 == 0 {
            play_round_parallel(&board_a, &mut board_b, board_width, board_height)?;
            patterns::detect_patterns_parallel(&board_b, board_width, board_height, &mut pattern_board, &searched_patterns)?;
            render_board(&board_b, board_width, board_height, &pattern_board, &mut canvas)?;
        } else {
            play_round_parallel(&board_b, &mut board_a, board_width, board_height)?;
            patterns::detect_patterns_parallel(&board_a, board_width, board_height, &mut pattern_board, &searched_patterns)?;
            render_board(&board_a, board_width, board_height, &pattern_board, &mut canvas)?;
        }

        pattern_board.write().unwrap().clear();
        pattern_board.write().unwrap().resize_with((board_width * board_height) as usize, Default::default);

        iteration += 1;
        last_tick = timer.ticks();
    }

    Ok(())
}
