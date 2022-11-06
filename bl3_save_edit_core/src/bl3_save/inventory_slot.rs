use strum::{EnumMessage, EnumString};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct InventorySlotData {
    pub slot: InventorySlot,
    pub unlocked: bool,
}

#[derive(Debug, EnumString, EnumMessage, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum InventorySlot {
    #[strum(
        serialize = "/Game/Gear/Weapons/_Shared/_Design/InventorySlots/BPInvSlot_Weapon1.BPInvSlot_Weapon1",
    )]
    Weapon1,
    #[strum(
        serialize = "/Game/Gear/Weapons/_Shared/_Design/InventorySlots/BPInvSlot_Weapon2.BPInvSlot_Weapon2",
    )]
    Weapon2,
    #[strum(
        serialize = "/Game/Gear/Weapons/_Shared/_Design/InventorySlots/BPInvSlot_Weapon3.BPInvSlot_Weapon3",
    )]
    Weapon3,
    #[strum(
        serialize = "/Game/Gear/Weapons/_Shared/_Design/InventorySlots/BPInvSlot_Weapon4.BPInvSlot_Weapon4",
    )]
    Weapon4,
    #[strum(
        serialize = "/Game/Gear/Shields/_Design/A_Data/BPInvSlot_Shield.BPInvSlot_Shield",
    )]
    Shield,
    #[strum(
        serialize = "/Game/Gear/GrenadeMods/_Design/A_Data/BPInvSlot_GrenadeMod.BPInvSlot_GrenadeMod",
    )]
    Grenade,
    #[strum(
        serialize = "/Game/Gear/ClassMods/_Design/_Data/BPInvSlot_ClassMod.BPInvSlot_ClassMod",
    )]
    ClassMod,
    #[strum(
        serialize = "/Game/Gear/Artifacts/_Design/_Data/BPInvSlot_Artifact.BPInvSlot_Artifact",
    )]
    Artifact,
}

impl std::default::Default for InventorySlot {
    fn default() -> Self {
        Self::Grenade
    }
}

impl std::fmt::Display for InventorySlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            InventorySlot::Weapon1 => t!("InventorySlot.Weapon1"),
            InventorySlot::Weapon2 => t!("InventorySlot.Weapon2"),
            InventorySlot::Weapon3 => t!("InventorySlot.Weapon3"),
            InventorySlot::Weapon4 => t!("InventorySlot.Weapon4"),
            InventorySlot::Shield => t!("InventorySlot.Shield"),
            InventorySlot::Grenade => t!("InventorySlot.Grenade"),
            InventorySlot::ClassMod => t!("InventorySlot.ClassMod"),
            InventorySlot::Artifact => t!("InventorySlot.Artifact"),
        })
    }
}
