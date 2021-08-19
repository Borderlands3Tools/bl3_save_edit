use strum::{Display, EnumIter, EnumMessage, EnumString};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct SaveSduSlotData {
    pub slot: SduSlot,
    pub current: i32,
    pub max: i32,
}

#[derive(
    Debug, Display, EnumString, EnumIter, EnumMessage, Eq, PartialEq, Ord, PartialOrd, Clone,
)]
pub enum SduSlot {
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_Backpack.SDU_Backpack",
        to_string = "Backpack"
    )]
    Backpack,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_SniperRifle.SDU_SniperRifle",
        to_string = "Sniper"
    )]
    Sniper,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_Shotgun.SDU_Shotgun",
        to_string = "Shotgun"
    )]
    Shotgun,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_Pistol.SDU_Pistol",
        to_string = "Pistol"
    )]
    Pistol,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_Grenade.SDU_Grenade",
        to_string = "Grenade"
    )]
    Grenade,
    #[strum(serialize = "/Game/Pickups/SDU/SDU_SMG.SDU_SMG", to_string = "SMG")]
    Smg,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_AssaultRifle.SDU_AssaultRifle",
        to_string = "AR"
    )]
    Ar,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_Heavy.SDU_Heavy",
        to_string = "Heavy"
    )]
    Heavy,
}

impl std::default::Default for SduSlot {
    fn default() -> Self {
        Self::Backpack
    }
}

impl SduSlot {
    pub fn maximum(&self) -> i32 {
        match *self {
            SduSlot::Backpack | SduSlot::Sniper | SduSlot::Heavy => 13,
            SduSlot::Shotgun | SduSlot::Pistol | SduSlot::Grenade | SduSlot::Smg | SduSlot::Ar => {
                10
            }
        }
    }
}
