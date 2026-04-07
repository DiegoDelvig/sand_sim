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

fn create_cell(mut grid: Vec<Color>, cols: usize, size: f32, sand: Color) -> Vec<Color> {
    if is_mouse_button_down(MouseButton::Left) {
        let mx = (mouse_position().0 / size) as usize;
        let my = (mouse_position().1 / size) as usize;

        let index = my * cols + mx;

        grid[index] = sand;
        grid[index + 1] = sand;
        grid[index - 1] = sand;
        
    }
    grid
}

fn get_new_grid(grid: Vec<Color>, cols: usize, rows: usize, bg: Color, sand: Color) -> Vec<Color> {
    let mut new_grid: Vec<Color> = grid;

    // .rev() Lire de bas en haut
    for row in (0..(rows - 1)).rev() {
        for col in 0..cols {
            let index = row * cols + col;

            if new_grid[index] == sand {
                let index_below = (row + 1) * cols + col;

                if new_grid[index_below] == bg {
                    new_grid[index] = new_grid[index_below];
                    new_grid[index_below] = sand;
                } else {
                    let direction = rand::gen_range(0, 2);
                    let mut a_move = false;

                    if direction == 0 {
                        if col > 0 {
                            let index_bl = index_below - 1;
                            if new_grid[index_bl] == bg {
                                new_grid[index] = bg;
                                new_grid[index_bl] = sand;
                                a_move = true;
                            }
                        }
                        if !a_move && col < cols - 1{
                            let index_br = index_below + 1;
                            if new_grid[index_br] == bg {
                                new_grid[index] = bg;
                                new_grid[index_br] = sand;
                            }
                        }
                    } else {
                        if col < cols - 1 {
                            let index_br = index_below + 1;
                            if new_grid[index_br] == bg {
                                new_grid[index] = bg;
                                new_grid[index_br] = sand;
                                a_move = true;
                            }
                            if !a_move && col > 0 {
                                let index_bl = index_below - 1;
                                if new_grid[index_bl] == bg {
                                    new_grid[index] = bg;
                                    new_grid[index_bl] = sand;
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
    const BACKGROUND: Color = Color::from_rgba(30, 30, 46, 255);
    const SAND_COLOR: Color = Color::from_rgba(203, 166, 247, 255);


    let cols: usize = (screen_width() / CELL_SIZE) as usize;
    let rows: usize = (screen_height() / CELL_SIZE) as usize;

    let mut grid: Vec<Color> = vec![BACKGROUND; cols * rows];
    
    loop {
        clear_background(BACKGROUND);
        grid = create_cell(grid, cols, CELL_SIZE, SAND_COLOR);

        // Pour modifier la fonction, paramètre à passer, index, index_below, bg, sand, col, row
        grid = get_new_grid(grid, cols, rows, BACKGROUND, SAND_COLOR);
        display_grid(&grid, cols, rows, CELL_SIZE); 
        next_frame().await;
    }

}
