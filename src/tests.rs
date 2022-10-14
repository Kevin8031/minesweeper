use crate::{Cell, CellState, Game, GameOpts};

/// A preconfigured map
fn preset_3x3_3mines() -> Game {
    let opts = GameOpts {
        width: 3,
        height: 3,
        mines_count: 2,
        mines_percentage: None,
    };

    let map = vec![
        Cell {
            bomb: true,
            nearby_mines: 1,
            state: CellState::Open,
        },
        Cell {
            bomb: true,
            nearby_mines: 1,
            state: CellState::Open,
        },
        Cell {
            bomb: false,
            nearby_mines: 1,
            state: CellState::Open,
        },
        Cell {
            bomb: true,
            nearby_mines: 2,
            state: CellState::Open,
        },
        Cell {
            bomb: false,
            nearby_mines: 0,
            state: CellState::Open,
        },
        Cell {
            bomb: false,
            nearby_mines: 0,
            state: CellState::Open,
        },
        Cell {
            bomb: false,
            nearby_mines: 0,
            state: CellState::Open,
        },
        Cell {
            bomb: false,
            nearby_mines: 0,
            state: CellState::Open,
        },
        Cell {
            bomb: false,
            nearby_mines: 0,
            state: CellState::Open,
        },
    ];

    Game { opts, map }
}

#[test]
fn map_generation() {
    let game_opts = GameOpts {
        width: 8,
        height: 8,
        mines_count: 16,
        mines_percentage: Some(25),
    };
    let game = Game::new(game_opts);

    assert_eq!(game.map.len(), 64);
    assert_eq!(game.opts.mines_count(), 16);
}

#[test]
fn cell_recursion_check_0_mines() {
    let opts = GameOpts {
        width: 3,
        height: 3,
        mines_count: 2,
        mines_percentage: None,
    };

    let mut map = Vec::new();

    for _ in 0..9 {
        map.push(Cell {
            bomb: false,
            nearby_mines: 0,
            state: CellState::Open,
        })
    }

    let game = Game { opts, map };

    let expected_result = vec![
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8
    ];

    if let Some(mut result) = game.check_empty_cells(8, &mut Vec::new()) {
        result.sort();
        assert_eq!(result, expected_result);
    }
}

#[test]
fn cell_recursion_check_3_mines() {
    let game = preset_3x3_3mines();

    let expected_result = vec![
        2,
        4,
        5,
        6,
        7,
        8
    ];

    if let Some(mut result) = game.check_empty_cells(8, &mut Vec::new()) {
        result.sort();
        assert_eq!(result, expected_result);
    }
}

#[test]
fn move_check_on_mine() {
    let game = preset_3x3_3mines();

    assert_eq!(game.check_move(0), None);
    assert_eq!(game.check_move(1), None);
    assert_eq!(game.check_move(3), None);
}

#[test]
fn move_check_not_on_mine() {
    let game = preset_3x3_3mines();

    assert_eq!(game.check_move(2), Some(
        vec![(2usize,
            &Cell {
                bomb: false,
                nearby_mines: 1,
                state: CellState::Open,
            },
        )]
    ));
}