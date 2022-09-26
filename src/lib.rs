use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameOpts {
    width: usize,
    height: usize,
    mines_count: usize,
    mines_percentage: Option<usize>
}

impl GameOpts {
    pub fn new(width: usize, height: usize, mines_count: usize, mines_percentage: Option<usize>) -> GameOpts {
        GameOpts { width, height, mines_count, mines_percentage }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn mines_percentage(&self) -> Option<usize> {
        self.mines_percentage
    }

    pub fn mines_count(&self) -> usize {
        self.mines_count
    }
}

impl Default for GameOpts {
    fn default() -> Self {
        Self { width: 8, height: 8, mines_count: 10, mines_percentage: None }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cell {
    bomb: bool,
    nearby_mines: usize
}

impl Cell {
    pub fn new() -> Cell {
        Cell { bomb: false, nearby_mines: 0 }
    }

    pub fn set_bomb(&mut self) {
        self.bomb = true;
    }

    pub fn add_nearby_mine(&mut self) {
        self.nearby_mines += 1;
    }

    pub fn mine(&self) -> bool {
        self.bomb
    }

    pub fn nearby_mines(&self) -> usize {
        self.nearby_mines
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    opts: GameOpts,
    map: Vec<Cell>
}

impl Game {
    fn calculate_mines_count(map: &mut Vec<Cell>, width: usize, height: usize) {
        for x in 0..width {
            for y in 0..height {
                let index = x * width + y;  // calculate index

                if !map[index].mine() {
                    let mut x_range = -1..=1;
                    if x == 0 { x_range = 0..=1; }
                    else if x == width - 1 { x_range = -1..=0; }
                    for x_offset in x_range {
                        let mut y_range = -1..=1;
                        if y == 0 { y_range = 0..=1; }
                        else if y == height - 1 { y_range = -1..=0; }
                        
                        for y_offset in y_range {

                            let x_cell = (x as i32 + x_offset) as usize;
                            let y_cell = (y as i32 + y_offset) as usize;
    
                            let check_index = x_cell * width + y_cell;
                        
                            if check_index < width * height && check_index != index {
                                if map[check_index].mine() { map[index].add_nearby_mine(); }
                            }
                        }
                    }
                }
            }
        }
    }
    fn generate_map(game_opts: &GameOpts) -> Vec<Cell> {
        let mut mines_total = game_opts.mines_percentage().unwrap_or_else(|| game_opts.mines_count());

        let mut map = vec![Cell::new(); game_opts.width() * game_opts.height()];
        println!("{mines_total}");
        while mines_total > 0 {
            let x = rand::thread_rng().gen_range(0..game_opts.width());
            let y = rand::thread_rng().gen_range(0..game_opts.height());       

            let index = x * game_opts.width() + y;
            map[index].set_bomb();
        
            mines_total -= 1;
        }

        Self::calculate_mines_count(&mut map, game_opts.width(), game_opts.height());

        map
    }

    pub fn new(game_opts: &GameOpts) -> Game {
        let map = Self::generate_map(&game_opts);
        Game { opts: game_opts.clone(), map }
    }

    /// Returns true if the
    /// target cell isn't a mine 
    pub fn check_move(&self, target_index: usize) -> Option<usize> {
        let cell = self.get_cell(target_index);
        if cell.mine() {
            None
        } else {
            Some(cell.nearby_mines())
        }
    }

    pub fn get_cell(&self, index: usize) -> &Cell {
        &self.map[index]
    }

    pub fn opts(&self) -> &GameOpts {
        &self.opts
    }
}