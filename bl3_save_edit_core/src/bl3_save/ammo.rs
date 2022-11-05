use strum::{EnumMessage, EnumString};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct AmmoPoolData {
    pub pool: AmmoPool,
    pub current: i32,
    pub max: i32,
}

#[derive(Debug, EnumString, EnumMessage, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum AmmoPool {
    #[strum(
        serialize = "/Game/GameData/Weapons/Ammo/Resource_Ammo_Grenade.Resource_Ammo_Grenade",
    )]
    Grenade,
    #[strum(
        serialize = "/Game/GameData/Weapons/Ammo/Resource_Ammo_Pistol.Resource_Ammo_Pistol",
    )]
    Pistol,
    #[strum(
        serialize = "/Game/GameData/Weapons/Ammo/Resource_Ammo_Shotgun.Resource_Ammo_Shotgun",
    )]
    Shotgun,
    #[strum(
        serialize = "/Game/GameData/Weapons/Ammo/Resource_Ammo_SMG.Resource_Ammo_SMG",
    )]
    Smg,
    #[strum(
        serialize = "/Game/GameData/Weapons/Ammo/Resource_Ammo_AssaultRifle.Resource_Ammo_AssaultRifle",
    )]
    Ar,
    #[strum(
        serialize = "/Game/GameData/Weapons/Ammo/Resource_Ammo_Sniper.Resource_Ammo_Sniper",
    )]
    Sniper,
    #[strum(
        serialize = "/Game/GameData/Weapons/Ammo/Resource_Ammo_Heavy.Resource_Ammo_Heavy",
    )]
    Heavy,
}

impl std::default::Default for AmmoPool {
    fn default() -> Self {
        Self::Grenade
    }
}

impl std::fmt::Display for AmmoPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            AmmoPool::Grenade => t!("AmmoPool.Grenade"),
            AmmoPool::Pistol => t!("AmmoPool.Pistol"),
            AmmoPool::Shotgun => t!("AmmoPool.Shotgun"),
            AmmoPool::Smg => t!("AmmoPool.Smg"),
            AmmoPool::Ar => t!("AmmoPool.Ar"),
            AmmoPool::Sniper => t!("AmmoPool.Sniper"),
            AmmoPool::Heavy => t!("AmmoPool.Heavy"),
        })
    }
}

impl AmmoPool {
    pub fn maximum(&self) -> i32 {
        match self {
            AmmoPool::Grenade => 13,
            AmmoPool::Pistol => 1200,
            AmmoPool::Shotgun => 280,
            AmmoPool::Smg => 2160,
            AmmoPool::Ar => 1680,
            AmmoPool::Sniper => 204,
            AmmoPool::Heavy => 51,
        }
    }
}
