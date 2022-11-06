use strum::{EnumMessage, EnumString};

#[derive(Clone, Copy, Debug, EnumString, EnumMessage, Eq, PartialEq, Ord, PartialOrd)]
pub enum PlayerClass {
    #[strum(
        serialize = "/Game/PlayerCharacters/Beastmaster/PlayerClassId_Beastmaster.PlayerClassId_Beastmaster",
    )]
    BeastMaster,
    #[strum(
        serialize = "/Game/PlayerCharacters/Gunner/PlayerClassId_Gunner.PlayerClassId_Gunner",
    )]
    Gunner,
    #[strum(
        serialize = "/Game/PlayerCharacters/Operative/PlayerClassId_Operative.PlayerClassId_Operative",
    )]
    Operative,
    #[strum(
        serialize = "/Game/PlayerCharacters/SirenBrawler/PlayerClassId_Siren.PlayerClassId_Siren",
    )]
    Siren,
}

impl Default for PlayerClass {
    fn default() -> Self {
        Self::BeastMaster
    }
}

impl std::fmt::Display for PlayerClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PlayerClass::BeastMaster => t!("PlayerClass.BeastMaster"),
            PlayerClass::Gunner => t!("PlayerClass.Gunner"),
            PlayerClass::Operative => t!("PlayerClass.Operative"),
            PlayerClass::Siren => t!("PlayerClass.Siren"),
        })
    }
}

impl PlayerClass {
    pub const ALL: [PlayerClass; 4] = [
        PlayerClass::BeastMaster,
        PlayerClass::Gunner,
        PlayerClass::Operative,
        PlayerClass::Siren,
    ];
}
