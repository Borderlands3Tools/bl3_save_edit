use strum::{Display, EnumIter, EnumMessage, EnumString};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct SaveSduSlotData {
    pub sdu: SaveSduSlot,
    pub current: i32,
    pub max: i32,
}

#[derive(
    Debug, EnumString, EnumIter, EnumMessage, Eq, PartialEq, Ord, PartialOrd, Clone,
)]
pub enum SaveSduSlot {
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_Backpack.SDU_Backpack",
    )]
    Backpack,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_SniperRifle.SDU_SniperRifle",
    )]
    Sniper,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_Shotgun.SDU_Shotgun",
    )]
    Shotgun,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_Pistol.SDU_Pistol",
    )]
    Pistol,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_Grenade.SDU_Grenade",
    )]
    Grenade,
    #[strum(serialize = "/Game/Pickups/SDU/SDU_SMG.SDU_SMG",)]
    Smg,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_AssaultRifle.SDU_AssaultRifle",
    )]
    Ar,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_Heavy.SDU_Heavy",
    )]
    Heavy,
}

impl std::default::Default for SaveSduSlot {
    fn default() -> Self {
        Self::Backpack
    }
}

impl std::fmt::Display for SaveSduSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            SaveSduSlot::Backpack => t!("SaveSduSlot.Backpack"),
            SaveSduSlot::Sniper => t!("SaveSduSlot.Sniper"),
            SaveSduSlot::Shotgun => t!("SaveSduSlot.Shotgun"),
            SaveSduSlot::Pistol => t!("SaveSduSlot.Pistol"),
            SaveSduSlot::Grenade => t!("SaveSduSlot.Grenade"),
            SaveSduSlot::Smg => t!("SaveSduSlot.Smg"),
            SaveSduSlot::Ar => t!("SaveSduSlot.Ar"),
            SaveSduSlot::Heavy => t!("SaveSduSlot.Heavy"),
        })
    }
}

impl SaveSduSlot {
    pub fn maximum(&self) -> i32 {
        match self {
            SaveSduSlot::Backpack | SaveSduSlot::Sniper | SaveSduSlot::Heavy => 13,
            SaveSduSlot::Shotgun
            | SaveSduSlot::Pistol
            | SaveSduSlot::Grenade
            | SaveSduSlot::Smg
            | SaveSduSlot::Ar => 10,
        }
    }
}
