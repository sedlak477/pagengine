use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum GameType {
    T,
    R,
    S,
    P1,
    P2,
    P3,
    SR,
    PB1,
    PB2,
    PB3,
    B,
    BR,
    PO1,
    PO2,
    PO3,
    D,
    BO,
    PD,
    SD,
    SPD,
}

impl FromStr for GameType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "T" => Ok(GameType::T),
            "R" => Ok(GameType::R),
            "S" => Ok(GameType::S),
            "P1" => Ok(GameType::P1),
            "P2" => Ok(GameType::P2),
            "P3" => Ok(GameType::P3),
            "SR" => Ok(GameType::SR),
            "PB1" => Ok(GameType::PB1),
            "PB2" => Ok(GameType::PB2),
            "PB3" => Ok(GameType::PB3),
            "B" => Ok(GameType::B),
            "BR" => Ok(GameType::BR),
            "PO1" => Ok(GameType::PO1),
            "PO2" => Ok(GameType::PO2),
            "PO3" => Ok(GameType::PO3),
            "D" => Ok(GameType::D),
            "BO" => Ok(GameType::BO),
            "PD" => Ok(GameType::PD),
            "SD" => Ok(GameType::SD),
            "SPD" => Ok(GameType::SPD),
            _ => Err("Invalid game type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_type_from_str() {
        assert_eq!(GameType::from_str("T"), Ok(GameType::T));
        assert_eq!(GameType::from_str("R"), Ok(GameType::R));
        assert_eq!(GameType::from_str("S"), Ok(GameType::S));
        assert_eq!(GameType::from_str("P1"), Ok(GameType::P1));
        assert_eq!(GameType::from_str("P2"), Ok(GameType::P2));
        assert_eq!(GameType::from_str("P3"), Ok(GameType::P3));
        assert_eq!(GameType::from_str("SR"), Ok(GameType::SR));
        assert_eq!(GameType::from_str("PB1"), Ok(GameType::PB1));
        assert_eq!(GameType::from_str("PB2"), Ok(GameType::PB2));
        assert_eq!(GameType::from_str("PB3"), Ok(GameType::PB3));
        assert_eq!(GameType::from_str("B"), Ok(GameType::B));
        assert_eq!(GameType::from_str("BR"), Ok(GameType::BR));
        assert_eq!(GameType::from_str("PO1"), Ok(GameType::PO1));
        assert_eq!(GameType::from_str("PO2"), Ok(GameType::PO2));
        assert_eq!(GameType::from_str("PO3"), Ok(GameType::PO3));
        assert_eq!(GameType::from_str("D"), Ok(GameType::D));
        assert_eq!(GameType::from_str("BO"), Ok(GameType::BO));
        assert_eq!(GameType::from_str("PD"), Ok(GameType::PD));
        assert_eq!(GameType::from_str("SD"), Ok(GameType::SD));
        assert_eq!(GameType::from_str("SPD"), Ok(GameType::SPD));

        assert_eq!(GameType::from_str("t"), Ok(GameType::T));
        assert_eq!(GameType::from_str("r"), Ok(GameType::R));
        assert_eq!(GameType::from_str("s"), Ok(GameType::S));
        assert_eq!(GameType::from_str("p1"), Ok(GameType::P1));
        assert_eq!(GameType::from_str("p2"), Ok(GameType::P2));
        assert_eq!(GameType::from_str("p3"), Ok(GameType::P3));
        assert_eq!(GameType::from_str("sr"), Ok(GameType::SR));
        assert_eq!(GameType::from_str("pb1"), Ok(GameType::PB1));
        assert_eq!(GameType::from_str("pb2"), Ok(GameType::PB2));
        assert_eq!(GameType::from_str("pb3"), Ok(GameType::PB3));
        assert_eq!(GameType::from_str("b"), Ok(GameType::B));
        assert_eq!(GameType::from_str("br"), Ok(GameType::BR));
        assert_eq!(GameType::from_str("po1"), Ok(GameType::PO1));
        assert_eq!(GameType::from_str("po2"), Ok(GameType::PO2));
        assert_eq!(GameType::from_str("po3"), Ok(GameType::PO3));
        assert_eq!(GameType::from_str("d"), Ok(GameType::D));
        assert_eq!(GameType::from_str("bo"), Ok(GameType::BO));
        assert_eq!(GameType::from_str("pd"), Ok(GameType::PD));
        assert_eq!(GameType::from_str("sd"), Ok(GameType::SD));
        assert_eq!(GameType::from_str("spd"), Ok(GameType::SPD));

        assert_eq!(GameType::from_str("sPd"), Ok(GameType::SPD));

        assert_eq!(GameType::from_str("invalid"), Err("Invalid game type"));
        assert_eq!(GameType::from_str("psd"), Err("Invalid game type"));
        assert_eq!(GameType::from_str("pb4"), Err("Invalid game type"));
        assert_eq!(GameType::from_str("pb4"), Err("Invalid game type"));
    }
}
