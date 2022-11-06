use strum::{Display, EnumIter, EnumMessage, EnumString};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct ProfileSduSlotData {
    pub sdu: ProfileSduSlot,
    pub current: i32,
    pub max: i32,
}

#[derive(
    Debug, Display, EnumString, EnumIter, EnumMessage, Eq, PartialEq, Ord, PartialOrd, Clone,
)]
pub enum ProfileSduSlot {
    #[strum(serialize = "/Game/Pickups/SDU/SDU_Bank.SDU_Bank")]
    Bank,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_LostLoot.SDU_LostLoot",
    )]
    LostLoot,
}

impl ProfileSduSlot {
    pub fn maximum(&self) -> i32 {
        match self {
            ProfileSduSlot::Bank => 28,
            ProfileSduSlot::LostLoot => 10,
        }
    }
}

impl std::default::Default for ProfileSduSlot {
    fn default() -> Self {
        Self::Bank
    }
}

impl std::fmt::Display for ProfileSduSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ProfileSduSlot::Bank => t!("ProfileSduSlot.Bank"),
            ProfileSduSlot::LostLoot => t!("ProfileSduSlot.LostLoot"),
        })
    }
}
