use crate::{Cell, CellState, Game, GameOpts};

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

    let map = vec![
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

    let game = Game { opts, map };

    let expected_result = Some(vec![
        0,
        1,
        3,
        2,
        4,
        5,
        6,
        7,
        8
    ]);

    assert_eq!(game.check_empty_cells(0, &mut Vec::new()), expected_result);
}
