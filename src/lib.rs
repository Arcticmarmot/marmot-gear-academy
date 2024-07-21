#![no_std]

use gstd::*;
use pebbles_game_io::*;

static mut PEBBLES_GAME: Option<GameState> = None;

#[no_mangle]
extern "C" fn init(){
    let init_msg: PebblesInit = msg::load().expect("Unable to load the init message");
    restart(init_msg.difficulty, init_msg.pebbles_count, init_msg.max_pebbles_per_turn);
}

#[no_mangle]
extern "C" fn handle(){
    let action = msg::load().expect("Unable to load the action");
    let pebbles_game = unsafe{ PEBBLES_GAME.get_or_insert(Default::default()) };
    match action {
        PebblesAction::GiveUp => {
            pebbles_game.winner = Some(Player::Program);
            let _ = msg::reply(
                PebblesEvent::Won(pebbles_game.winner.as_ref().expect("Winner").clone()), 
                0);
        }
        PebblesAction::Restart { difficulty, pebbles_count, max_pebbles_per_turn } => {
            restart(difficulty.clone(), pebbles_count, max_pebbles_per_turn);
            // let _ = msg::reply(PebblesInit{
            //     difficulty,
            //     pebbles_count,
            //     max_pebbles_per_turn
            // }, 0);
            let pebbles_game = unsafe{ PEBBLES_GAME.get_or_insert(Default::default()) };
            let _ = msg::reply(PebblesEvent::CounterTurn(pebbles_game.pebbles_remaining), 0);
        }
        PebblesAction::Turn(mut pebbles_remove_num) => {
            // Player::User开始执行
            pebbles_remove_num = regular_num(pebbles_remove_num, pebbles_game);
            remove_pebbles(pebbles_remove_num, pebbles_game);
            if pebbles_game.pebbles_remaining == 0 {
                pebbles_game.winner = Some(Player::User);
                let _ = msg::reply(PebblesEvent::Won(pebbles_game.winner.as_ref().expect("winner").clone()), 0);
                exec::leave();
            } else {
                pebbles_remove_num = get_pebbles_remove_num(&pebbles_game);
                remove_pebbles(pebbles_remove_num, pebbles_game);
                if pebbles_game.pebbles_remaining == 0 {
                    pebbles_game.winner = Some(Player::Program);
                    let _ = msg::reply(PebblesEvent::Won(pebbles_game.winner.as_ref().expect("winner").clone()), 0);
                    exec::leave();
                } else {
                    let _ = msg::reply(PebblesEvent::CounterTurn(pebbles_game.pebbles_remaining), 0);
                }
            }
        }
    };
}

#[no_mangle]
extern "C" fn state(){
    let pebbles_game = unsafe { PEBBLES_GAME.take().expect("State is not existing") };
    msg::reply(pebbles_game, 0).expect("Unable to get the state");
}

/**
 * 生成随机数
 */
fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

/**
 * 重新开始游戏
 */
fn restart(init_msg_difficulty: DifficultyLevel, init_msg_pebbles_count: u32, init_msg_max_pebbles_per_turn: u32) {
    // TODO: 随机
    let first_player: Player = Player::Program; // if get_random_u32() % 2 == 0 { Player::User } else { Player::Program };
    let mut pebbles_game = GameState {
        difficulty: init_msg_difficulty,
        pebbles_count: init_msg_pebbles_count,
        max_pebbles_per_turn: init_msg_max_pebbles_per_turn,
        pebbles_remaining: init_msg_pebbles_count,
        first_player: first_player.clone(),
        winner: None,
    };
    match first_player {
        Player::Program => {
            remove_pebbles(get_pebbles_remove_num(&pebbles_game), &mut pebbles_game);
        }
        Player::User => {}
    };
    unsafe { PEBBLES_GAME = Some(pebbles_game) };
}

/**
 * 控制移除数值大小
 */
pub fn regular_num(pebbles_remove_num: u32, pebbles_game: &GameState) -> u32{
    if pebbles_remove_num <= 0 {
        return 1;
    }
    if pebbles_remove_num > pebbles_game.max_pebbles_per_turn {
        return pebbles_game.max_pebbles_per_turn;
    }
    pebbles_remove_num
}

/**
 * 减少pebbles数量
 */

pub fn remove_pebbles(pebbles_remove_num: u32, pebbles_game: &mut GameState){
    if pebbles_remove_num >= pebbles_game.pebbles_remaining {
        pebbles_game.pebbles_remaining = 0;
    } else {
        pebbles_game.pebbles_remaining -= pebbles_remove_num;
    }
}

/**
 * 判定赢家是否诞生
 */
pub fn check_and_reply(pebbles_game: &mut GameState, player: Player){
    if pebbles_game.pebbles_remaining == 0 {
        pebbles_game.winner = Some(player);
        let _ = msg::reply(PebblesEvent::Won(pebbles_game.winner.as_ref().expect("winner").clone()), 0);
        exec::leave();
    } else {
        let _ = msg::reply(PebblesEvent::CounterTurn(pebbles_game.pebbles_remaining), 0);
    }
}

/**
 * 根据难度生成程序所需拿走的鹅卵石数量
 */
pub fn get_pebbles_remove_num(pebbles_game: &GameState) -> u32{
    match pebbles_game.difficulty {
        DifficultyLevel::Easy => (get_random_u32() % (pebbles_game.max_pebbles_per_turn)) + 1,
        DifficultyLevel::Hard => (get_random_u32() % (pebbles_game.max_pebbles_per_turn)) + 1,
    }
}