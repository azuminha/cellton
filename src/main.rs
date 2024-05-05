extern crate sdl2;

use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;

static WIDTH: u32 = 800;
static HEIGHT: u32 = 600;
static CELL_SIZE: u32 = 10;

#[derive(Clone)]
#[derive(Copy)]
struct Cell{
    value: u8,
    die: u8,
}

fn count_neighbors(grid: &mut [[Cell; 80]; 80], x: &i32, y: &i32) -> i32{
    let mut count = 0;
    for i in (x-1)..(x+2) {
        for j in (y-1)..(y+2){
            if i != *x || j != *y {
                if i >= 0 && i < 80 && j >= 0 && j < 80 {
                    count += grid[i as usize][j as usize].value as i32;
                }
            }
        }
    }
    count
}

fn update_cell(grid: &mut [[Cell; 80]; 80]){
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let ii: i32 = i as i32;
            let jj: i32 = j as i32;
            let neighbors = count_neighbors(grid, &ii, &jj);

            if grid[i][j].value == 1{
              //  println!("{} {} {}", i, j, neighbors);
                if neighbors < 2 || neighbors > 3 {
                    grid[i][j].die = 1;
                }
            }else{
                if neighbors == 3 {
                    grid[i][j].die = 2;
                }
            }
        }
    }

    
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j].die == 1{
                grid[i][j].value = 0;
            }else if grid[i][j].die == 2 {
               grid[i][j].value = 1; 
            }

            grid[i][j].die = 0;
        }
    }
}

pub fn main() {
    //let mut GRID: [[Cell; 80]; 60] = [[Cell: 0}; 80]; 60];
    let mut GRID: [[Cell; 80]; 80] = [[Cell { value: 0, die: 0 }; 80]; 80];

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("cellton", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    // glider generator
    // col lin
    GRID[2][7].value = 1;
    GRID[2][8].value = 1;
    GRID[3][7].value = 1;
    GRID[3][8].value = 1;

    
    GRID[13][6].value = 1;
    GRID[13][7].value = 1;
    GRID[13][8].value = 1;

    GRID[14][9].value = 1;
    GRID[14][5].value = 1;

    GRID[15][10].value = 1;
    GRID[15][4].value = 1;

    GRID[16][9].value = 1;
    GRID[16][5].value = 1;

    GRID[17][6].value = 1;
    GRID[17][7].value = 1;
    GRID[17][8].value = 1;
    GRID[18][6].value = 1;
    GRID[18][7].value = 1;
    GRID[18][8].value = 1;

    GRID[23][4].value = 1;
    GRID[23][5].value = 1;
    GRID[23][6].value = 1;

    GRID[24][4].value = 1;
    GRID[24][3].value = 1;
    GRID[24][6].value = 1;
    GRID[24][7].value = 1;
    GRID[25][4].value = 1;
    GRID[25][3].value = 1;
    GRID[25][6].value = 1;
    GRID[25][7].value = 1;
    GRID[26][4].value = 1;
    GRID[26][3].value = 1;
    GRID[26][5].value = 1;
    GRID[26][6].value = 1;
    GRID[26][7].value = 1;

    GRID[27][3].value = 1;
    GRID[27][2].value = 1;
    GRID[27][8].value = 1;
    GRID[27][7].value = 1;

    GRID[32][3].value = 1;
    GRID[32][4].value = 1;

    
    GRID[36][5].value = 1;
    GRID[36][6].value = 1;
    GRID[37][5].value = 1;
    GRID[37][6].value = 1;
    //
    let mut flag = 0;
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

   
        // draw cells;
        for (i, row) in GRID.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                if (*col).value == 1 {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                    canvas.fill_rect(Rect::new((CELL_SIZE * i as u32) as i32, (CELL_SIZE * j as u32) as i32, CELL_SIZE, CELL_SIZE));
                }
            }
        }

        if flag == -1 {
            update_cell(&mut GRID);
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    update_cell(&mut GRID);
                },
                Event::KeyDown { keycode: Some(Keycode::B), .. } => {
                    flag = !flag;
                    println!("B: {}", flag);
                },
                Event::MouseButtonDown { x, y, mouse_btn: MouseButton::Left, .. } =>{
                    let xx = (x as u32)/CELL_SIZE;
                    let yy = (y as u32)/CELL_SIZE;
                    GRID[xx as usize][yy as usize].value = 1;
                    println!("add {} {}", xx, yy);
                },
                Event::MouseButtonDown { x, y, mouse_btn: MouseButton::Right, .. } =>{
                    let xx = (x as u32)/CELL_SIZE;
                    let yy = (y as u32)/CELL_SIZE;
                    GRID[xx as usize][yy as usize].value = 0;
                    println!("deleted {} {}", xx, yy);
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
