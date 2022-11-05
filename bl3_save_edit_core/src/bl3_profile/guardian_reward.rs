use strum::{EnumIter, EnumMessage, EnumString};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct GuardianRewardData {
    pub reward: GuardianReward,
    pub current: i32,
    pub max: i32,
}

#[derive(
    Debug, EnumString, EnumIter, EnumMessage, Eq, PartialEq, Ord, PartialOrd, Clone,
)]
pub enum GuardianReward {
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_Accuracy.GuardianReward_Accuracy",
    )]
    Accuracy,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_ActionSkillCooldown.GuardianReward_ActionSkillCooldown",
    )]
    ActionSkillCooldown,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_CriticalDamage.GuardianReward_CriticalDamage",
    )]
    CriticalDamage,
    #[strum(
        serialize = "/Game/PatchDLC/Hibiscus/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_ElementalDamage.GuardianReward_ElementalDamage",
    )]
    ElementalDamage,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_FFYLDuration.GuardianReward_FFYLDuration",
    )]
    FFYLDuration,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_FFYLMovementSpeed.GuardianReward_FFYLMovementSpeed",
    )]
    FFYLMovementSpeed,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_GrenadeDamage.GuardianReward_GrenadeDamage",
    )]
    GrenadeDamage,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_GunDamage.GuardianReward_GunDamage",
    )]
    GunDamage,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_GunFireRate.GuardianReward_GunFireRate",
    )]
    GunFireRate,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_MaxHealth.GuardianReward_MaxHealth",
    )]
    MaxHealth,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_MeleeDamage.GuardianReward_MeleeDamage",
    )]
    MeleeDamage,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_RarityRate.GuardianReward_RarityRate",
    )]
    RarityRate,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_RecoilReduction.GuardianReward_RecoilReduction",
    )]
    RecoilReduction,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_ReloadSpeed.GuardianReward_ReloadSpeed",
    )]
    ReloadSpeed,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_ShieldCapacity.GuardianReward_ShieldCapacity",
    )]
    ShieldCapacity,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_ShieldRechargeDelay.GuardianReward_ShieldRechargeDelay",
    )]
    ShieldRechargeDelay,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_ShieldRechargeRate.GuardianReward_ShieldRechargeRate",
    )]
    ShieldRechargeRate,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_VehicleDamage.GuardianReward_VehicleDamage",
    )]
    VehicleDamage,
}

impl std::default::Default for GuardianReward {
    fn default() -> Self {
        Self::Accuracy
    }
}

impl std::fmt::Display for GuardianReward {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            GuardianReward::Accuracy => t!("GuardianReward.Accuracy"),
            GuardianReward::ActionSkillCooldown => t!("GuardianReward.ActionSkillCooldown"),
            GuardianReward::CriticalDamage => t!("GuardianReward.CriticalDamage"),
            GuardianReward::ElementalDamage => t!("GuardianReward.ElementalDamage"),
            GuardianReward::FFYLDuration => t!("GuardianReward.FFYLDuration"),
            GuardianReward::FFYLMovementSpeed => t!("GuardianReward.FFYLMovementSpeed"),
            GuardianReward::GrenadeDamage => t!("GuardianReward.GrenadeDamage"),
            GuardianReward::GunDamage => t!("GuardianReward.GunDamage"),
            GuardianReward::GunFireRate => t!("GuardianReward.GunFireRate"),
            GuardianReward::MaxHealth => t!("GuardianReward.MaxHealth"),
            GuardianReward::MeleeDamage => t!("GuardianReward.MeleeDamage"),
            GuardianReward::RarityRate => t!("GuardianReward.RarityRate"),
            GuardianReward::RecoilReduction => t!("GuardianReward.RecoilReduction"),
            GuardianReward::ReloadSpeed => t!("GuardianReward.ReloadSpeed"),
            GuardianReward::ShieldCapacity => t!("GuardianReward.ShieldCapacity"),
            GuardianReward::ShieldRechargeDelay => t!("GuardianReward.ShieldRechargeDelay"),
            GuardianReward::ShieldRechargeRate => t!("GuardianReward.ShieldRechargeRate"),
            GuardianReward::VehicleDamage => t!("GuardianReward.VehicleDamage"),
        })
    }
}
