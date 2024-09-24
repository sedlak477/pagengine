use super::card::Card;
use super::game_type::GameType;
use regex::Regex;
use std::str::FromStr;
use std::sync::LazyLock;

const HAND_SIZE: usize = 12;
const STICH_SIZE: usize = 4;
const NUM_PLAYERS: usize = 4;
const TALON_SIZE: usize = 6;
const NUM_CARDS: usize = 54;

#[derive(Debug, Clone, Copy)]
pub struct CardCollection<const N: usize = NUM_CARDS> {
    pub cards: [Option<Card>; N],
    pub excluded: [Option<Card>; NUM_CARDS],
}

impl CardCollection {
    pub fn contains(&self, card: Card) -> bool {
        self.cards.contains(&Some(card))
    }
    pub fn excludes(&self, card: Card) -> bool {
        self.excluded.contains(&Some(card))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub hand: CardCollection<HAND_SIZE>,
    pub stiche: CardCollection,
    pub calls: Calls,
}

#[derive(Debug, Clone, Copy)]
pub struct Calls {
    pub typ: Option<GameType>,
    pub called_king: Option<Card>,
    pub taken_talon: Option<usize>,
    pub pagat: bool,
    pub uhu: bool,
    pub pelikan: bool,
    pub quapil: bool,
    pub trull: bool,
    pub kings: bool,
    pub ultimo: bool,
    pub valat: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct GameState {
    pub players: [Player; NUM_PLAYERS],
    pub stich: CardCollection<STICH_SIZE>,
    pub talon: CardCollection<TALON_SIZE>,
}

const CARD_REGEX_STR: &str = r"(?i)[HK][1-4KDPB]|[PX](?:[7-9KDPB]|10)|T(?:[1-9]|1[0-9]|2[0-2])|\.";
const CARDS_REGEX_STR: &str = r#"(?ix)
    ^(?<talon0>(?:[HK][1-4KDPB]|[PX](?:[7-9KDPB]|10)|T(?:[1-9]|1[0-9]|2[0-2])|\.){3})
    /(?<talon1>(?:[HK][1-4KDPB]|[PX](?:[7-9KDPB]|10)|T(?:[1-9]|1[0-9]|2[0-2])|\.){3})
    \#(?<player0>(?:[HK][1-4KDPB]|[PX](?:[7-9KDPB]|10)|T(?:2[0-2]|1[0-9]|[1-9])|\.){0,12}/(?:[HK][1-4KDPB]|[PX](?:[7-9KDPB]|10)|T(?:[1-9]|1[0-9]|2[0-2])|\.){0,54})
    \#(?<player1>(?:[HK][1-4KDPB]|[PX](?:[7-9KDPB]|10)|T(?:2[0-2]|1[0-9]|[1-9])|\.){0,12}/(?:[HK][1-4KDPB]|[PX](?:[7-9KDPB]|10)|T(?:[1-9]|1[0-9]|2[0-2])|\.){0,54})
    \#(?<player2>(?:[HK][1-4KDPB]|[PX](?:[7-9KDPB]|10)|T(?:2[0-2]|1[0-9]|[1-9])|\.){0,12}/(?:[HK][1-4KDPB]|[PX](?:[7-9KDPB]|10)|T(?:[1-9]|1[0-9]|2[0-2])|\.){0,54})
    \#(?<player3>(?:[HK][1-4KDPB]|[PX](?:[7-9KDPB]|10)|T(?:2[0-2]|1[0-9]|[1-9])|\.){0,12}/(?:[HK][1-4KDPB]|[PX](?:[7-9KDPB]|10)|T(?:[1-9]|1[0-9]|2[0-2])|\.){0,54})
    \#(?<stich>(?:[HK][1-4KDPB]|[PX](?:[7-9KDPB]|10)|T(?:2[0-2]|1[0-9]|[1-9])|\.){0,4})$"#;
static CARDS_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(CARDS_REGEX_STR).unwrap());

impl FromStr for GameState {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split_whitespace().take(5).collect();

        if let [cards_string, game_string, calls_string, kleinen_stechen_großen_string, spritzen_string] =
            &split[..]
        {
            // Parse cards
            let Some(captures) = CARDS_REGEX.captures(&cards_string) else {
                return Err("Invalid cards group");
            };

            let talon0 = captures.name("talon0").unwrap().as_str();
            let talon1 = captures.name("talon1").unwrap().as_str();
            let player0 = captures.name("player0").unwrap().as_str();
            let player1 = captures.name("player1").unwrap().as_str();
            let player2 = captures.name("player2").unwrap().as_str();
            let player3 = captures.name("player3").unwrap().as_str();
            let stich = captures.name("stich").unwrap().as_str();
            println!(
                "talon0: {}\ntalon1: {}\nplayer0: {}\nplayer1: {}\nplayer2: {}\nplayer3: {}\nstich: {}",
                talon0, talon1, player0, player1, player2, player3, stich
            );

            // Parse game

            return Err("Ok");
        } else {
            return Err("Missing TAF groups");
        }
    }
}
