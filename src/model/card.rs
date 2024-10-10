use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Card {
    UNKNOWN,
    HK,
    HD,
    HP,
    HB,
    H1,
    H2,
    H3,
    H4,
    PK,
    PD,
    PP,
    PB,
    P10,
    P9,
    P8,
    P7,
    KK,
    KD,
    KP,
    KB,
    K1,
    K2,
    K3,
    K4,
    XK,
    XD,
    XP,
    XB,
    X10,
    X9,
    X8,
    X7,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11,
    T12,
    T13,
    T14,
    T15,
    T16,
    T17,
    T18,
    T19,
    T20,
    T21,
    T22,
}

impl FromStr for Card {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "." => Ok(Card::UNKNOWN),
            "HK" => Ok(Card::HK),
            "HD" => Ok(Card::HD),
            "HP" => Ok(Card::HP),
            "HB" => Ok(Card::HB),
            "H1" => Ok(Card::H1),
            "H2" => Ok(Card::H2),
            "H3" => Ok(Card::H3),
            "H4" => Ok(Card::H4),
            "PK" => Ok(Card::PK),
            "PD" => Ok(Card::PD),
            "PP" => Ok(Card::PP),
            "PB" => Ok(Card::PB),
            "P10" => Ok(Card::P10),
            "P9" => Ok(Card::P9),
            "P8" => Ok(Card::P8),
            "P7" => Ok(Card::P7),
            "KK" => Ok(Card::KK),
            "KD" => Ok(Card::KD),
            "KP" => Ok(Card::KP),
            "KB" => Ok(Card::KB),
            "K1" => Ok(Card::K1),
            "K2" => Ok(Card::K2),
            "K3" => Ok(Card::K3),
            "K4" => Ok(Card::K4),
            "XK" => Ok(Card::XK),
            "XD" => Ok(Card::XD),
            "XP" => Ok(Card::XP),
            "XB" => Ok(Card::XB),
            "X10" => Ok(Card::X10),
            "X9" => Ok(Card::X9),
            "X8" => Ok(Card::X8),
            "X7" => Ok(Card::X7),
            "T1" => Ok(Card::T1),
            "T2" => Ok(Card::T2),
            "T3" => Ok(Card::T3),
            "T4" => Ok(Card::T4),
            "T5" => Ok(Card::T5),
            "T6" => Ok(Card::T6),
            "T7" => Ok(Card::T7),
            "T8" => Ok(Card::T8),
            "T9" => Ok(Card::T9),
            "T10" => Ok(Card::T10),
            "T11" => Ok(Card::T11),
            "T12" => Ok(Card::T12),
            "T13" => Ok(Card::T13),
            "T14" => Ok(Card::T14),
            "T15" => Ok(Card::T15),
            "T16" => Ok(Card::T16),
            "T17" => Ok(Card::T17),
            "T18" => Ok(Card::T18),
            "T19" => Ok(Card::T19),
            "T20" => Ok(Card::T20),
            "T21" => Ok(Card::T21),
            "T22" => Ok(Card::T22),
            _ => Err("Unknown card"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_from_str() {
        assert_eq!(Card::from_str("."), Ok(Card::UNKNOWN));
        assert_eq!(Card::from_str("HK"), Ok(Card::HK));
        assert_eq!(Card::from_str("HD"), Ok(Card::HD));
        assert_eq!(Card::from_str("HP"), Ok(Card::HP));
        assert_eq!(Card::from_str("HB"), Ok(Card::HB));
        assert_eq!(Card::from_str("H1"), Ok(Card::H1));
        assert_eq!(Card::from_str("H2"), Ok(Card::H2));
        assert_eq!(Card::from_str("H3"), Ok(Card::H3));
        assert_eq!(Card::from_str("H4"), Ok(Card::H4));
        assert_eq!(Card::from_str("PK"), Ok(Card::PK));
        assert_eq!(Card::from_str("PD"), Ok(Card::PD));
        assert_eq!(Card::from_str("PP"), Ok(Card::PP));
        assert_eq!(Card::from_str("PB"), Ok(Card::PB));
        assert_eq!(Card::from_str("P10"), Ok(Card::P10));
        assert_eq!(Card::from_str("P9"), Ok(Card::P9));
        assert_eq!(Card::from_str("P8"), Ok(Card::P8));
        assert_eq!(Card::from_str("P7"), Ok(Card::P7));
        assert_eq!(Card::from_str("KK"), Ok(Card::KK));
        assert_eq!(Card::from_str("KD"), Ok(Card::KD));
        assert_eq!(Card::from_str("KP"), Ok(Card::KP));
        assert_eq!(Card::from_str("KB"), Ok(Card::KB));
        assert_eq!(Card::from_str("K1"), Ok(Card::K1));
        assert_eq!(Card::from_str("K2"), Ok(Card::K2));
        assert_eq!(Card::from_str("K3"), Ok(Card::K3));
        assert_eq!(Card::from_str("K4"), Ok(Card::K4));
        assert_eq!(Card::from_str("XK"), Ok(Card::XK));
        assert_eq!(Card::from_str("XD"), Ok(Card::XD));
        assert_eq!(Card::from_str("XP"), Ok(Card::XP));
        assert_eq!(Card::from_str("XB"), Ok(Card::XB));
        assert_eq!(Card::from_str("X10"), Ok(Card::X10));
        assert_eq!(Card::from_str("X9"), Ok(Card::X9));
        assert_eq!(Card::from_str("X8"), Ok(Card::X8));
        assert_eq!(Card::from_str("X7"), Ok(Card::X7));
        assert_eq!(Card::from_str("T1"), Ok(Card::T1));
        assert_eq!(Card::from_str("T2"), Ok(Card::T2));
        assert_eq!(Card::from_str("T3"), Ok(Card::T3));
        assert_eq!(Card::from_str("T4"), Ok(Card::T4));
        assert_eq!(Card::from_str("T5"), Ok(Card::T5));
        assert_eq!(Card::from_str("T6"), Ok(Card::T6));
        assert_eq!(Card::from_str("T7"), Ok(Card::T7));
        assert_eq!(Card::from_str("T8"), Ok(Card::T8));
        assert_eq!(Card::from_str("T9"), Ok(Card::T9));
        assert_eq!(Card::from_str("T10"), Ok(Card::T10));
        assert_eq!(Card::from_str("T11"), Ok(Card::T11));
        assert_eq!(Card::from_str("T12"), Ok(Card::T12));
        assert_eq!(Card::from_str("T13"), Ok(Card::T13));
        assert_eq!(Card::from_str("T14"), Ok(Card::T14));
        assert_eq!(Card::from_str("T15"), Ok(Card::T15));
        assert_eq!(Card::from_str("T16"), Ok(Card::T16));
        assert_eq!(Card::from_str("T17"), Ok(Card::T17));
        assert_eq!(Card::from_str("T18"), Ok(Card::T18));
        assert_eq!(Card::from_str("T19"), Ok(Card::T19));
        assert_eq!(Card::from_str("T20"), Ok(Card::T20));
        assert_eq!(Card::from_str("T21"), Ok(Card::T21));
        assert_eq!(Card::from_str("T22"), Ok(Card::T22));

        assert_eq!(Card::from_str("T23"), Err("Unknown card"));
        assert_eq!(Card::from_str("asd"), Err("Unknown card"));
        assert_eq!(Card::from_str("4564"), Err("Unknown card"));
        assert_eq!(Card::from_str("X1"), Err("Unknown card"));
        assert_eq!(Card::from_str("X4"), Err("Unknown card"));
        assert_eq!(Card::from_str("H10"), Err("Unknown card"));
    }
}
