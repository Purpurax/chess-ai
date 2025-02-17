use super::{board::Board, game::Game, move_validator::is_check};

static mut SNAPSHOT: Snapshot = Snapshot {
    game_boards: vec![],
    debug_index: 0,
};

struct Snapshot {
    game_boards: Vec<[u64; 7]>,
    debug_index: usize,
}

pub fn save_state(board: &Board) {
    let layers: [u64; 7] = board.export();
    unsafe {
        SNAPSHOT.game_boards.push(layers);
        // SNAPSHOT.game_boards.iter().for_each(|layers| {
        //     println!("Board:");
        //     layers.into_iter().for_each(|layer| {
        //         println!("{:#060b}", layer);
        //     });
        // });
    }
}

pub fn enter_debug(game_state: &Game) {
    unsafe {
        SNAPSHOT.game_boards = vec![game_state.board.export()];
        SNAPSHOT.debug_index = 0;
    }
}

pub fn debug_left() -> [u64; 7] {
    unsafe {
        if SNAPSHOT.debug_index != 0 {
            SNAPSHOT.debug_index -= 1;
        } else {
            println!("orignal board reached");
        }
        SNAPSHOT.game_boards[SNAPSHOT.debug_index]
    }
}

pub fn debug_right() -> [u64; 7] {
    unsafe {
        if SNAPSHOT.debug_index + 1 < SNAPSHOT.game_boards.len() {
            SNAPSHOT.debug_index += 1;
        } else {
            println!("right reached (debug_index: {})", SNAPSHOT.debug_index);
        }
        if is_check(
            &Board::import(SNAPSHOT.game_boards[SNAPSHOT.debug_index]),
            false,
        ) {
            println!("1111111111111");
        } else {
            println!("0000000000000");
        }
        SNAPSHOT.game_boards[SNAPSHOT.debug_index]
    }
}

pub fn exit_debug() -> [u64; 7] {
    unsafe {
        let initial_board: [u64; 7] = SNAPSHOT.game_boards[0];
        SNAPSHOT.game_boards = vec![];
        SNAPSHOT.debug_index = 0;
        initial_board
    }
}
