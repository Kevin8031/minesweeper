use std::ops::RangeInclusive;

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
    pub fn print_map(&self) {
        for x in 0..self.opts.width {
            for y in 0..self.opts.height {
                let mut char = 'X';
                if self.map.get(x * self.opts.width + y).unwrap().mine() {
                    char = 'B';
                }
                print!("{} ", char);
            }

            print!("  ");
            
            for y in 0..self.opts.height {
                print!("{} ", self.map.get(x * self.opts.width + y).unwrap().nearby_mines());
            }
            println!();
        }
    }
    
    fn calculate_mines_count(map: &mut Vec<Cell>, opts: &GameOpts) {
        for x in 0..opts.width() {
            for y in 0..opts.height() {
                let index = x * opts.width() + y;  // calculate index

                if !map[index].mine() {
                    let mut x_range = -1..=1;
                    if x == 0 { x_range = 0..=1; }
                    else if x == opts.width() - 1 { x_range = -1..=0; }
                    for x_offset in x_range {
                        let mut y_range = -1..=1;
                        if y == 0 { y_range = 0..=1; }
                        else if y == opts.height() - 1 { y_range = -1..=0; }
                        
                        for y_offset in y_range {

                            let x_cell = (x as i32 + x_offset) as usize;
                            let y_cell = (y as i32 + y_offset) as usize;
    
                            let check_index = x_cell * opts.width() + y_cell;
                        
                            if check_index < opts.width() * opts.height() && check_index != index {
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
        while mines_total > 0 {
            let x = rand::thread_rng().gen_range(0..game_opts.width());
            let y = rand::thread_rng().gen_range(0..game_opts.height());       

            let index = x * game_opts.width() + y;
            map[index].set_bomb();
        
            mines_total -= 1;
        }

        Self::calculate_mines_count(&mut map, &game_opts);

        map
    }

    pub fn new(game_opts: &GameOpts) -> Game {
        let map = Self::generate_map(&game_opts);
        Game { opts: game_opts.clone(), map }
    }

    /// Returns the cell if target
    /// cell isn't a mine. Otherwise
    /// return none
    pub fn check_move(&self, target_index: usize, past_index: &mut Option<&Vec<usize>>) -> Option<Vec<(usize, &Cell)>> {
        let mut a = match past_index {
            Some(a) => a.to_vec(),
            None => Vec::new(),
        };

        let cell = self.get_cell(target_index);
        if cell.mine() {
            // Game Over
            None
        } else {
            let x = target_index / self.opts.width;
            let y = target_index / self.opts.height;
            let mut vec = Vec::new();
            
            if cell.nearby_mines == 0 {
                for x_offset in self.nearby_range_x(x) {
                    'here: for y_offset in self.nearby_range_y(y) {
                        let x_cell = (x as i32 + x_offset) as usize;
                        let y_cell = (y as i32 + y_offset) as usize;
                        let index = x_cell * self.opts.width() + y_cell;
 
                        if index == target_index { a.push(index); continue; }
                        
                        for i in &a {
                            if *i == target_index { continue 'here; }
                        }
                        
                        self.check_move(index, &mut Some(&a));
                    }
                }
            } else {
                vec.push((target_index, cell));
            }
            
            Some(vec)
        }
    }

    pub fn get_cell(&self, index: usize) -> &Cell {
        &self.map[index]
    }

    pub fn opts(&self) -> &GameOpts {
        &self.opts
    }

    fn nearby_range_x(&self, x: usize) -> RangeInclusive<i32> {
        let mut x_range = -1..=1;
        if x == 0 { x_range = 0..=1; }
        else if x == self.opts.width() - 1 { x_range = -1..=0; }

        x_range
    }

    fn nearby_range_y(&self, y: usize) -> RangeInclusive<i32> {
        let mut y_range = -1..=1;
        if y == 0 { y_range = 0..=1; }
        else if y == self.opts.height() - 1 { y_range = -1..=0; }

        y_range
    }
}