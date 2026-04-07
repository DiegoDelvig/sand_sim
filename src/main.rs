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

fn create_cell(mut grid: Vec<Color>, cols: usize, rows: usize, size: f32, sand: Color, water: Color) -> Vec<Color> {
    let mut color_to_spawn = None;

    // Clic Gauche = Sable, Clic Droit = Eau
    if is_mouse_button_down(MouseButton::Left) {
        color_to_spawn = Some(sand);
    } else if is_mouse_button_down(MouseButton::Right) {
        color_to_spawn = Some(water);
    }

    if let Some(color) = color_to_spawn {
        let mx = (mouse_position().0 / size) as usize;
        let my = (mouse_position().1 / size) as usize;

        if mx > 0 && mx < cols - 1 && my < rows {
            let index = my * cols + mx;
            grid[index] = color;
            grid[index + 1] = color;
            grid[index - 1] = color;
        }
    }
    grid
}

fn get_new_grid(grid: Vec<Color>, cols: usize, rows: usize, bg: Color, sand: Color, water: Color) -> Vec<Color> {
    let mut new_grid: Vec<Color> = grid;

    // .rev() Lire de bas en haut
    for row in (0..(rows - 1)).rev() {
        for col in 0..cols {
            let index = row * cols + col;
            let current_color = new_grid[index];

            if current_color == sand || current_color == water {
                let index_below = (row + 1) * cols + col;

                // 1. Règle commune : Tomber tout droit
                if new_grid[index_below] == bg {
                    new_grid[index] = bg;
                    new_grid[index_below] = current_color;
                } else {
                    // 2. Règle commune : Glisser sur les diagonales
                    let direction = rand::gen_range(0, 2);
                    let mut a_move = false;

                    if direction == 0 {
                        if col > 0 && new_grid[index_below - 1] == bg {
                            new_grid[index] = bg;
                            new_grid[index_below - 1] = current_color;
                            a_move = true;
                        } else if col < cols - 1 && new_grid[index_below + 1] == bg {
                            new_grid[index] = bg;
                            new_grid[index_below + 1] = current_color;
                            a_move = true;
                        }
                    } else {
                        if col < cols - 1 && new_grid[index_below + 1] == bg {
                            new_grid[index] = bg;
                            new_grid[index_below + 1] = current_color;
                            a_move = true;
                        } else if col > 0 && new_grid[index_below - 1] == bg {
                            new_grid[index] = bg;
                            new_grid[index_below - 1] = current_color;
                            a_move = true;
                        }
                    }

                    // 3. RÈGLE SPÉCIFIQUE À L'EAU : S'étaler sur les côtés
                    if !a_move && current_color == water {
                        let dir_horiz = rand::gen_range(0, 2);
                        
                        if dir_horiz == 0 {
                            if col > 0 && new_grid[index - 1] == bg {
                                new_grid[index] = bg;
                                new_grid[index - 1] = current_color;
                            } else if col < cols - 1 && new_grid[index + 1] == bg {
                                new_grid[index] = bg;
                                new_grid[index + 1] = current_color;
                            }
                        } else {
                            if col < cols - 1 && new_grid[index + 1] == bg {
                                new_grid[index] = bg;
                                new_grid[index + 1] = current_color;
                            } else if col > 0 && new_grid[index - 1] == bg {
                                new_grid[index] = bg;
                                new_grid[index - 1] = current_color;
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
    const WATER_COLOR: Color = Color::from_rgba(137, 180, 250, 255);

    let cols: usize = (screen_width() / CELL_SIZE) as usize;
    let rows: usize = (screen_height() / CELL_SIZE) as usize;

    let mut grid: Vec<Color> = vec![BACKGROUND; cols * rows];
    
    loop {
        clear_background(BACKGROUND);
        
        grid = create_cell(grid, cols, rows, CELL_SIZE, SAND_COLOR, WATER_COLOR);

        grid = get_new_grid(grid, cols, rows, BACKGROUND, SAND_COLOR, WATER_COLOR);
        
        display_grid(&grid, cols, rows, CELL_SIZE); 
        next_frame().await;
    }
}
