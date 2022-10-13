use std::{io, cmp};
use console::{Term, Key, Style};
use minesweeper::{Game, GameOpts};

fn draw_menu(term: &Term) -> io::Result<()> {
    let highlighted = Style::new().black().on_white();
    let mut selection = 0;
    
    let items = vec![
        "8x8 - 10 mines",
        "16x16 - 40 mines",
        "30x16 - 99 miines"
    ];
    
    loop {
        term.clear_screen()?;

        for i in 0..items.len() {
            let item = items.get(i).unwrap();
            if i == selection {
                term.write_line(&format!("{}", highlighted.apply_to(item)))?;
            } else {
                term.write_line(item)?;
            }
        }

        match term.read_key().unwrap() {
            Key::Char('q') => break,
            Key::ArrowDown => selection = cmp::min(selection + 1, items.len() - 1),
            Key::ArrowUp => if selection > 0 { selection -= 1; },
            Key::Enter => {
                let opts= match selection {
                    0 => {
                        GameOpts::new(8, 8, 10, None)
                    },
                    1 => {
                        GameOpts::new(16, 16, 40, None)
                    },
                    2 => {
                        GameOpts::new(36, 16, 99, None)
                    },
                    _ => GameOpts::default()
                };
                let game = Game::new(opts);
                draw_game(term, game)?;
            },
            _ => {}
        }
        
    }

    Ok(())
}

fn draw_game(term: &Term, game: Game) -> io::Result<()> {
    const EMPTY_CELL: &str = "[ ]";
    const HIGHLIGHTED_CELL: &str = "[*]";
    const MARKED_CELL: &str = "[?]";
    
    let closed = Style::new().color256(64);
    
    let map_height = game.opts().height();
    let map_width = game.opts().width();

    let mut selection = 0;

    let mut x = 0;
    let mut y = 0;

    loop {
        term.clear_screen()?;

        for i in 0..map_width {
            for j in 0..map_height {
                let index = i * map_width + j;
                let cell = game.get_cell(index);
                
                if index == selection {
                    print!("{}", HIGHLIGHTED_CELL);
                } else {
                    match cell.state() {
                        minesweeper::CellState::Closed => {
                            print!("{}", format!("{}", closed.apply_to(EMPTY_CELL)));
                        },
                        minesweeper::CellState::Open => {
                            print!("{}", EMPTY_CELL);
                        },
                        minesweeper::CellState::Marked => {
                            print!("{}", MARKED_CELL);
                        },
                    }
                }
                print!(" ");
            }
            term.write_line("")?;
        }

        match term.read_key().unwrap() {
            Key::Char('q') => break,
            Key::ArrowUp => if y > 0 { y -= 1 },
            Key::ArrowLeft => if x > 0 { x -= 1},
            Key::ArrowDown => y = cmp::min(y + 1, map_height - 1),
            Key::ArrowRight => x = cmp::min(x + 1, map_width - 1),
            _ => {}
        }

        selection = y * map_width + x;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let term = Term::stdout();
    term.hide_cursor()?;

    draw_menu(&term)?;

    term.clear_screen()?;
    term.show_cursor()?;
    Ok(())
}