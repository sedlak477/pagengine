use std::str::FromStr;

use model::game::GameState;

mod model;

fn main() {
    match GameState::from_str(
        ".../...#hdt1t3t5t6k1k2k3k4kbkp/#.........../#.........../#.........../#hkx8t22t21 R1XK-1 1K/T// - R12",
    ) {
        Ok(game_state) => {
            println!("OK: {:?}", game_state);
        }
        Err(e) => {
            println!("Err: {:?}", e);
        }
    }
}
