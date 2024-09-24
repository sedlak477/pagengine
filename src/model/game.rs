use super::card::{Card, MAX_SIZE};
use super::game_type::GameType;

const HAND_SIZE: usize = 12;
const STICH_SIZE: usize = 4;
const NUM_PLAYERS: usize = 4;
const TALON_SIZE: usize = 6;

pub struct CardCollection<const N: usize = MAX_SIZE> {
    pub cards: [Option<Card>; N],
    pub excluded: [Option<Card>; MAX_SIZE],
}

impl CardCollection {
    pub fn contains(&self, card: Card) -> bool {
        self.cards.contains(&Some(card))
    }
    pub fn excludes(&self, card: Card) -> bool {
        self.excluded.contains(&Some(card))
    }
}

struct Player {
    hand: CardCollection<HAND_SIZE>,
    stiche: CardCollection,
    calls: Option<Calls>,
}

struct Calls {
    typ: GameType,
    called_king: Option<Card>,
    taken_talon: Option<usize>,
    pagat: bool,
    uhu: bool,
    pelikan: bool,
    quapil: bool,
    trull: bool,
    kings: bool,
    ultimo: bool,
    valat: bool,
}

struct GameState {
    players: [Player; NUM_PLAYERS],
    stich: CardCollection<STICH_SIZE>,
    talon: CardCollection<TALON_SIZE>,
}
