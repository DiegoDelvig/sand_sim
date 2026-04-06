use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title : "sand_sim".to_owned(),
        window_width : 800,
        window_height : 800,
        window_resizable: false,
        ..Default::default()
    }
}

fn display_grid(grid: &Vec<Color>, cols: usize, rows: usize, size: f32) {
    for col in 0..cols {
        for row in 0..rows {
            let index = row * cols + col;
            let x = col as f32 * size;
            let y = row as f32 * size;

            draw_rectangle(x, y, size, size, grid[index]);
        }
    }
}

fn create_cell(mut grid: Vec<Color>, cols: usize, size: f32, mm: Color) -> Vec<Color> {
    if is_mouse_button_down(MouseButton::Left) {
        let mx = (mouse_position().0 / size) as usize;
        let my = (mouse_position().1 / size) as usize;

        let index = my * cols + mx;
        grid[index] = mm;
        
    }
    grid
}

fn get_new_grid(grid: Vec<Color>, cols: usize, rows: usize, mb: Color, mm: Color) -> Vec<Color> {
    let mut new_grid: Vec<Color> = grid;

    // .rev() Lire de bas en haut
    for row in (0..(rows - 1)).rev() {
        for col in 0..cols {
            let index = row * cols + col;

            if new_grid[index] == mm {
                let index_below = (row + 1) * cols + col;

                if new_grid[index_below] == mb {
                    new_grid[index] = new_grid[index_below];
                    new_grid[index_below] = mm;
                } else {
                    let direction = rand::gen_range(0, 2);
                    let mut a_move = false;

                    if direction == 0 {
                        if col > 0 {
                            let index_bl = index_below - 1;
                            if new_grid[index_bl] == mb {
                                new_grid[index] = mb;
                                new_grid[index_bl] = mm;
                                a_move = true;
                            }
                        }
                        if !a_move && col < cols - 1{
                            let index_br = index_below + 1;
                            if new_grid[index_br] == mb {
                                new_grid[index] = mb;
                                new_grid[index_br] = mm;
                            }
                        }
                    } else {
                        if col < cols - 1 {
                            let index_br = index_below + 1;
                            if new_grid[index_br] == mb {
                                new_grid[index] = mb;
                                new_grid[index_br] = mm;
                                a_move = true;
                            }
                            if !a_move && col > 0 {
                                let index_bl = index_below - 1;
                                if new_grid[index_bl] == mb {
                                    new_grid[index] = mb;
                                    new_grid[index_bl] = mm;
                                }
                            }
                        }

                    }
                }
                
            }
        }
    }
    new_grid
}

#[macroquad::main(window_conf)]
async fn main() {
    const CELL_SIZE: f32 = 5.0;
    const MOCHABASE: Color = Color::from_rgba(30, 30, 46, 255);
    const MOCHAMAUVE: Color = Color::from_rgba(203, 166, 247, 255);


    let cols: usize = (screen_width() / CELL_SIZE) as usize;
    let rows: usize = (screen_height() / CELL_SIZE) as usize;

    let mut grid: Vec<Color> = vec![MOCHABASE; cols * rows];
    
    loop {
        clear_background(MOCHABASE);
        grid = create_cell(grid, cols, CELL_SIZE, MOCHAMAUVE);
        grid = get_new_grid(grid, cols, rows, MOCHABASE, MOCHAMAUVE);
        display_grid(&grid, cols, rows, CELL_SIZE); 
        next_frame().await;
    }

}
