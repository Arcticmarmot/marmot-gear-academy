use gtest::{Program, System};
use pebbles_game_io::*;

const USER_ID: u64 = 100;

/**
 * 初始化成功测试
 */
#[test]
fn init_success(){
    let system = System::new();
    let program = Program::current(&system);

    let init_data = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 100,
        max_pebbles_per_turn: 10,
    };

    let init_result = program.send(USER_ID, init_data);
    assert!(!init_result.main_failed());

    let game_state: GameState = program.read_state(b"").unwrap();
    assert_eq!(game_state.pebbles_count, 100);
    assert_eq!(game_state.max_pebbles_per_turn, 10);
    assert_eq!(game_state.pebbles_remaining, 100);
    // assert!(game_state.first_player == Player::User || game_state.first_player == Player::Program);
}

/**
 * 初始化失败测试
 * 当最大数量大于总数时，初始化失败
 */
#[test]
fn init_fail(){
    let system = System::new();
    let program = Program::current(&system);

    let init_data = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 100,
        max_pebbles_per_turn: 101,
    };

    let init_result = program.send(USER_ID, init_data);
    assert!(!init_result.main_failed());

    let game_state: GameState = program.read_state(b"").unwrap();
    assert_eq!(game_state.pebbles_count, 0);
    assert_eq!(game_state.max_pebbles_per_turn, 0);
    assert_eq!(game_state.pebbles_remaining, 0);
}

/**
 * Giveup选项测试
 */
#[test]
fn giveup(){
    let system = System::new();
    let program = Program::current(&system);

    let init_data = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 100,
        max_pebbles_per_turn: 10,
    };

    let init_result = program.send(USER_ID, init_data);
    assert!(!init_result.main_failed());

    let _ = program.send(USER_ID, PebblesAction::GiveUp);
    let game_state: GameState = program.read_state(b"").unwrap();
    let winner = game_state.winner.as_ref().expect("Unable to convert").clone();
    match winner {
        Player::Program => {
        }
        Player::User => {
            // 若为User胜利，判定失败
            assert_eq!(1, 0)
        }
    };
}
