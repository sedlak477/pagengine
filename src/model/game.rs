use super::card::Card;
use super::game_type::GameType;
use regex::Regex;
use std::collections::HashSet;
use std::fmt::Debug;
use std::iter;
use std::str::FromStr;
use std::sync::LazyLock;

const HAND_SIZE: usize = 12;
const STICH_SIZE: usize = 4;
const NUM_PLAYERS: usize = 4;
const NUM_CARDS: usize = 54;

const CARD_REGEX_STR: &str = r"(?i)[HK][1-4KDPB]|[PX](?:[7-9KDPB]|10)|T(?:2[0-2]|1[0-9]|[1-9])|\.";
static CARD_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(CARD_REGEX_STR).unwrap());

const CARDS_REGEX_STR: &str = r#"(?ix)
    ^(?<talon0>(?:[HK][1-4KDPB]|[PX](?:[7-9KDPB]|10)|T(?:2[0-2]|1[0-9]|[1-9])|\.){3})
        /(?<talon1>(?:[HK][1-4KDPB]|[PX](?:[7-9KDPB]|10)|T(?:2[0-2]|1[0-9]|[1-9])|\.){3})
    \#(?<player0hand>(?:[HK][1-4KDPB]|[PX](?:[7-9KDPB]|10)|T(?:2[0-2]|1[0-9]|[1-9])|\.){0,12})
        /(?<player0stiche>(?:[HK][1-4KDPB]|[PX](?:[7-9KDPB]|10)|T(?:2[0-2])|1[0-9]|[1-9]|\.){0,54})
    \#(?<player1hand>(?:[HK][1-4KDPB]|[PX](?:[7-9KDPB]|10)|T(?:2[0-2]|1[0-9]|[1-9])|\.){0,12})
        /(?<player1stiche>(?:[HK][1-4KDPB]|[PX](?:[7-9KDPB]|10)|T(?:2[0-2])|1[0-9]|[1-9]|\.){0,54})
    \#(?<player2hand>(?:[HK][1-4KDPB]|[PX](?:[7-9KDPB]|10)|T(?:2[0-2]|1[0-9]|[1-9])|\.){0,12})
        /(?<player2stiche>(?:[HK][1-4KDPB]|[PX](?:[7-9KDPB]|10)|T(?:2[0-2])|1[0-9]|[1-9]|\.){0,54})
    \#(?<player3hand>(?:[HK][1-4KDPB]|[PX](?:[7-9KDPB]|10)|T(?:2[0-2]|1[0-9]|[1-9])|\.){0,12})
        /(?<player3stiche>(?:[HK][1-4KDPB]|[PX](?:[7-9KDPB]|10)|T(?:2[0-2])|1[0-9]|[1-9]|\.){0,54})
    \#(?<stich>(?:[HK][1-4KDPB]|[PX](?:[7-9KDPB]|10)|T(?:2[0-2]|1[0-9]|[1-9])|\.){0,4})$"#;
static CARDS_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(CARDS_REGEX_STR).unwrap());

const GAME_TYPE_REGEX_STR: &str = r#"(?ix)
    (?<game>SPD|SR|BR|BO|PD|SD|PO[1-3]|PB[1-3]|P[1-3]|[TRSBD])(?<player>[1-4])"#;
static GAME_TYPE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(GAME_TYPE_REGEX_STR).unwrap());

const GAME_REGEX_STR: &str = r#"(?ix)
    ^(?<gameAndPlayer>(?:(?:SPD|SR|BR|BO|PD|SD|PO[1-3]|PB[1-3]|P[1-3]|[TRSBD])[1-4]){1,4})(?<king>[HKPX]K|\-)(?<teammate>[1-4-])(?<talon>12|[12-])$"#;
static GAME_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(GAME_REGEX_STR).unwrap());

const CALL_REGEX_STR: &str = r#"(?ix)
    (?<player0>1?2?3?4?T?U?K?V?)/(?<player1>1?2?3?4?T?U?K?V?)/(?<player2>1?2?3?4?T?U?K?V?)/(?<player3>1?2?3?4?T?U?K?V?)"#;
static CALL_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(CALL_REGEX_STR).unwrap());

#[derive(Clone, Copy, Eq)]
pub struct CardCollection<const N: usize = NUM_CARDS> {
    pub cards: [Option<Card>; N],
    pub excluded: [Option<Card>; NUM_CARDS],
}

impl<const N: usize> CardCollection<N> {
    pub fn new() -> Self {
        Self {
            cards: [None; N],
            excluded: [None; NUM_CARDS],
        }
    }
    pub fn contains(&self, card: Card) -> bool {
        self.cards.contains(&Some(card))
    }
    pub fn excludes(&self, card: Card) -> bool {
        self.excluded.contains(&Some(card))
    }
}

impl<const N: usize> TryFrom<&Vec<Card>> for CardCollection<N> {
    type Error = &'static str;

    fn try_from(value: &Vec<Card>) -> Result<Self, Self::Error> {
        Ok(CardCollection {
            cards: value
                .iter()
                .map(|card| Some(*card))
                .collect::<Vec<_>>()
                .try_into()
                .map_err(|_| "Invalid vector length")?,
            excluded: [None; NUM_CARDS],
        })
    }
}

impl<const N: usize> FromStr for CardCollection<N> {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CardCollection::<N> {
            cards: CARD_REGEX
                .find_iter(s)
                .map(|mtch| {
                    Card::from_str(mtch.as_str())
                        .map(|card| Some(card))
                        .map_err(|_| "Invalid card")
                })
                .chain(iter::repeat(Ok(None)))
                .take(N)
                .collect::<Result<Vec<_>, _>>()?
                .try_into()
                .unwrap(),
            excluded: [None; NUM_CARDS],
        })
    }
}

impl<const N: usize> PartialEq for CardCollection<N> {
    fn eq(&self, other: &Self) -> bool {
        let self_cards: HashSet<Option<Card>> = self.cards.try_into().unwrap();
        let other_cards: HashSet<Option<Card>> = other.cards.try_into().unwrap();
        let self_excluded: HashSet<Option<Card>> = self.excluded.try_into().unwrap();
        let other_excluded: HashSet<Option<Card>> = other.excluded.try_into().unwrap();
        let self_num_none = self.cards.iter().filter(|card| card.is_none()).count();
        let other_num_none = other.cards.iter().filter(|card| card.is_none()).count();
        self_cards == other_cards
            && self_excluded == other_excluded
            && self_num_none == other_num_none
    }
}

impl<const N: usize> Debug for CardCollection<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards: Vec<String> = self
            .cards
            .iter()
            .filter(|card| card.is_some())
            .map(|card| match card {
                Some(Card::UNKNOWN) => ".".to_string(),
                Some(card) => format!("{card:?}"),
                None => "".to_string(),
            })
            .collect();
        let excluded: Vec<String> = self
            .excluded
            .iter()
            .filter(|card| card.is_some())
            .map(|card| match card {
                Some(Card::UNKNOWN) => ".".to_string(),
                Some(card) => format!("{card:?}"),
                None => "".to_string(),
            })
            .collect();
        write!(
            f,
            "CardCollection {{ cards: {} + {} None, excluded: {} + {} None }}",
            cards.join(" "),
            N - cards.len(),
            excluded.join(" "),
            NUM_CARDS - excluded.len(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player {
    pub hand: CardCollection<HAND_SIZE>,
    pub stiche: CardCollection,
    pub calls: Calls,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameState {
    pub players: [Player; NUM_PLAYERS],
    pub stich: CardCollection<STICH_SIZE>,
    pub talon: [CardCollection<3>; 2],
    pub kleinen_stechen_großen: bool,
}

impl FromStr for GameState {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split_whitespace().take(5).collect();

        if let [cards_string, game_string, calls_string, kleinen_stechen_großen_string, spritzen_string] =
            &split[..]
        {
            // Parse cards
            let Some(captures) = CARDS_REGEX.captures(&cards_string) else {
                return Err("Invalid TAF cards group");
            };

            let talon0: CardCollection<3> = captures.name("talon0").unwrap().as_str().parse()?;
            let talon1: CardCollection<3> = captures.name("talon1").unwrap().as_str().parse()?;

            let mut players: [Player; 4] = [
                ("player0hand", "player0stiche"),
                ("player1hand", "player1stiche"),
                ("player2hand", "player2stiche"),
                ("player3hand", "player3stiche"),
            ]
            .iter()
            .map(
                |(hand_group, stiche_group)| -> Result<Player, &'static str> {
                    Ok(Player {
                        hand: captures.name(hand_group).unwrap().as_str().parse()?,
                        stiche: captures.name(stiche_group).unwrap().as_str().parse()?,
                        calls: Calls::default(),
                    })
                },
            )
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .unwrap();

            let stich = captures.name("stich").unwrap().as_str().parse()?;

            // Parse game
            let Some(captures) = GAME_REGEX.captures(&game_string) else {
                return Err("Invalid TAF game group");
            };

            let king: Option<Card> = match captures.name("king").unwrap().as_str() {
                "-" => None,
                s => Some(Card::from_str(s)?),
            };
            let teammate: Option<usize> = match captures.name("teammate").unwrap().as_str() {
                "-" => None,
                s => Some(s.parse().map_err(|_| "Invalid teammate")?),
            };
            let talon: Option<usize> = match captures.name("talon").unwrap().as_str() {
                "-" => None,
                s => Some(s.parse().map_err(|_| "Invalid talon")?),
            };

            let game_and_player: &str = captures.name("gameAndPlayer").unwrap().as_str();
            for game_and_player in GAME_TYPE_REGEX.captures_iter(game_and_player) {
                let game_type: GameType = game_and_player.name("game").unwrap().as_str().parse()?;
                let player: usize = game_and_player
                    .name("player")
                    .unwrap()
                    .as_str()
                    .parse()
                    .map_err(|_| "Invalid game player index")?;

                if player < 1 || player > NUM_PLAYERS {
                    return Err("Invalid game player index");
                }
                players[player - 1].calls.typ = Some(game_type);
                players[player - 1].calls.called_king = king;
                players[player - 1].calls.taken_talon = talon;
            }

            // Parse calls
            let Some(captures) = CALL_REGEX.captures(&calls_string) else {
                return Err("Invalid TAF calls group");
            };

            [
                ("player0", 0),
                ("player1", 1),
                ("player2", 2),
                ("player3", 3),
            ]
            .iter()
            .map(|(player_group, player_index)| {
                captures
                    .name(player_group)
                    .unwrap()
                    .as_str()
                    .chars()
                    .map(|c| {
                        match c {
                            '1' => players[*player_index].calls.pagat = true,
                            '2' => players[*player_index].calls.uhu = true,
                            '3' => players[*player_index].calls.pelikan = true,
                            '4' => players[*player_index].calls.quapil = true,
                            'T' => players[*player_index].calls.trull = true,
                            'K' => players[*player_index].calls.kings = true,
                            'U' => players[*player_index].calls.ultimo = true,
                            'V' => players[*player_index].calls.valat = true,
                            _ => return Err("Invalid call"),
                        }
                        Ok(())
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

            // Parse kleinen_stechen_großen
            let kleinen_stechen_großen: bool = match kleinen_stechen_großen_string {
                &"J" | &"j" => true,
                &"-" => false,
                _ => return Err("Invalid kleinen_stechen_großen"),
            };

            // TODO: Parse spritzen

            Ok(GameState {
                players,
                stich,
                talon: [talon0, talon1],
                kleinen_stechen_großen,
            })
        } else {
            Err("Missing TAF groups")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_collection_from_str() {
        assert_eq!(
            CardCollection::<3>::from_str("H1H2H3").unwrap().cards,
            [Some(Card::H1), Some(Card::H2), Some(Card::H3)],
        );
        assert_eq!(
            CardCollection::<3>::from_str("H1H2").unwrap().cards,
            [Some(Card::H1), Some(Card::H2), None],
        );
        assert_eq!(
            CardCollection::<3>::from_str("H1H2H3H4").unwrap().cards,
            [Some(Card::H1), Some(Card::H2), Some(Card::H3)],
        );
        assert_eq!(
            CardCollection::<3>::from_str("XKXBXD").unwrap().cards,
            [Some(Card::XK), Some(Card::XB), Some(Card::XD)],
        );
        assert_ne!(
            CardCollection::<3>::from_str("XKXBXD").unwrap().cards,
            [Some(Card::HK), Some(Card::HK), Some(Card::UNKNOWN)],
        );
        assert_eq!(
            CardCollection::<12>::from_str("").unwrap().cards,
            [None; 12]
        );
        assert_eq!(
            CardCollection::<12>::from_str("asdaP07sd").unwrap().cards,
            [None; 12]
        );
        assert_eq!(
            CardCollection::<6>::from_str("......").unwrap().cards,
            [Some(Card::UNKNOWN); 6]
        );
        assert_eq!(
            CardCollection::<6>::from_str("P9P8P7").unwrap().excluded,
            [None; 54]
        );
    }

    #[test]
    fn card_collection_eq() {
        assert_eq!(
            CardCollection::<3>::from_str("H1H2H3").unwrap(),
            CardCollection::<3>::from_str("H1H2H3").unwrap(),
        );
        assert_eq!(
            CardCollection::<3>::from_str("H1H2H3").unwrap(),
            CardCollection::<3>::from_str("H3H2H1").unwrap(),
        );
        assert_ne!(
            CardCollection::<3>::from_str("H1H2H3").unwrap(),
            CardCollection::<3>::from_str("H1H2H4").unwrap(),
        );
        assert_eq!(
            CardCollection::<3>::from_str("H1H2H3.").unwrap(),
            CardCollection::<3>::from_str("H1H2H3.").unwrap(),
        );
        assert_eq!(
            CardCollection::<4>::from_str("H1H2H3.").unwrap(),
            CardCollection::<4>::from_str("H1H2H3.").unwrap(),
        );
        assert_eq!(
            CardCollection::<5>::from_str("H1H2H3..").unwrap(),
            CardCollection::<5>::from_str("H1H2H3..").unwrap(),
        );
        assert_eq!(
            CardCollection::<5>::from_str("H1H2H3..").unwrap(),
            CardCollection::<5>::from_str("H2H1H3..").unwrap(),
        );
        assert_ne!(
            CardCollection::<5>::from_str("H1H2H3..").unwrap(),
            CardCollection::<5>::from_str("H1H2H3.").unwrap(),
        );
        assert_eq!(
            CardCollection::<12>::from_str("hdt1t3t5t6k1k2k3k4kbkp").unwrap(),
            CardCollection::<12>::from_str("hdt1t3t5t6k1k2k3k4kbkp").unwrap(),
        );
        assert_eq!(
            CardCollection::<54>::new(),
            CardCollection::<54>::from_str("").unwrap(),
        );
        assert_eq!(
            CardCollection::<54>::from_str("xkxbxd...").unwrap(),
            CardCollection::<54>::from_str("xdxbxk...").unwrap(),
        );
        assert_ne!(
            CardCollection::<54>::from_str("xkxbxd..").unwrap(),
            CardCollection::<54>::from_str("xdxbxk...").unwrap(),
        );
    }

    #[test]
    fn game_state_from_str() {
        // Start of Rufer
        assert_eq!(
            GameState::from_str(
                ".../...#hdt1t3t5t6k1k2k3k4kbkp/#.........../#.........../#.........../#hkx8t22t21 R1XK-1 1K/T// j -"
            )
            .unwrap(),
            GameState {
                players: [
                    Player {
                        hand: CardCollection::from_str("hdt1t3t5t6k1k2k3k4kbkp").unwrap(),
                        stiche: CardCollection::new(),
                        calls: Calls {
                            typ: Some(GameType::R),
                            taken_talon: Some(1),
                            pagat: true,
                            kings: true,
                            called_king: Some(Card::XK),
                            ..Default::default()
                        },
                    },
                    Player {
                        hand: CardCollection::from_str("...........").unwrap(),
                        stiche: CardCollection::new(),
                        calls: Calls {
                            trull: true,
                            ..Default::default()
                        }
                    },
                    Player {
                        hand: CardCollection::from_str("...........").unwrap(),
                        stiche: CardCollection::new(),
                        calls: Calls::default(),
                    },
                    Player {
                        hand: CardCollection::from_str("...........").unwrap(),
                        stiche: CardCollection::new(),
                        calls: Calls::default(),
                    },
                ],
                stich: CardCollection::<4>::from_str("HKX8T22T21").unwrap(),
                talon: [
                    CardCollection::<3>::from_str("...").unwrap(),
                    CardCollection::<3>::from_str("...").unwrap(),
                ],
                kleinen_stechen_großen: true,
            }
        );
    }

    #[test]
    fn game_state_eq() {
        assert_eq!(
            GameState {
                players: [
                    Player {
                        hand: CardCollection::<12>::from_str("H1H3T5T6K1K2K3K4KBKP....").unwrap(),
                        stiche: CardCollection::<54>::from_str("..............").unwrap(),
                        calls: Calls::default(),
                    },
                    Player {
                        hand: CardCollection::<12>::from_str("H8T22T21").unwrap(),
                        stiche: CardCollection::<54>::from_str("..............").unwrap(),
                        calls: Calls::default(),
                    },
                    Player {
                        hand: CardCollection::<12>::from_str("HK").unwrap(),
                        stiche: CardCollection::<54>::from_str("..............").unwrap(),
                        calls: Calls::default(),
                    },
                    Player {
                        hand: CardCollection::<12>::from_str("HX").unwrap(),
                        stiche: CardCollection::<54>::from_str("..............").unwrap(),
                        calls: Calls::default(),
                    },
                ],
                stich: CardCollection::<4>::from_str("T12....").unwrap(),
                talon: [
                    CardCollection::<3>::from_str("HDT").unwrap(),
                    CardCollection::<3>::from_str("HDT").unwrap(),
                ],
                kleinen_stechen_großen: true,
            },
            GameState {
                players: [
                    Player {
                        hand: CardCollection::<12>::from_str("H1H3T5T6K1K2K3K4KBKP....").unwrap(),
                        stiche: CardCollection::<54>::from_str("..............").unwrap(),
                        calls: Calls::default(),
                    },
                    Player {
                        hand: CardCollection::<12>::from_str("H8T22T21").unwrap(),
                        stiche: CardCollection::<54>::from_str("..............").unwrap(),
                        calls: Calls::default(),
                    },
                    Player {
                        hand: CardCollection::<12>::from_str("HK").unwrap(),
                        stiche: CardCollection::<54>::from_str("..............").unwrap(),
                        calls: Calls::default(),
                    },
                    Player {
                        hand: CardCollection::<12>::from_str("HX").unwrap(),
                        stiche: CardCollection::<54>::from_str("..............").unwrap(),
                        calls: Calls::default(),
                    },
                ],
                stich: CardCollection::<4>::from_str("T12....").unwrap(),
                talon: [
                    CardCollection::<3>::from_str("HDT").unwrap(),
                    CardCollection::<3>::from_str("HDT").unwrap(),
                ],
                kleinen_stechen_großen: true,
            }
        );
    }
}
