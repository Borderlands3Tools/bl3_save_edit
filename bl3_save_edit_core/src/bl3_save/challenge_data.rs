use strum::{Display, EnumIter, EnumMessage, EnumString};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct ChallengeData {
    pub challenge: Challenge,
    pub unlocked: bool,
}

#[derive(
    Debug, EnumString, EnumIter, EnumMessage, Eq, PartialEq, Ord, PartialOrd, Clone,
)]
pub enum Challenge {
    #[strum(
        serialize = "/Game/GameData/Challenges/Account/Challenge_VaultReward_ArtifactSlot.Challenge_VaultReward_ArtifactSlot_C",
    )]
    ArtifactSlot,
    #[strum(
        serialize = "/Game/GameData/Challenges/Account/Challenge_VaultReward_Analyzer.Challenge_VaultReward_Analyzer_C",
    )]
    EridianAnalyzer,
    #[strum(
        serialize = "/Game/GameData/Challenges/Account/Challenge_VaultReward_Resonator.Challenge_VaultReward_Resonator_C",
    )]
    EridianResonator,
    #[strum(
        serialize = "/Game/GameData/Challenges/Account/Challenge_VaultReward_Mayhem.Challenge_VaultReward_Mayhem_C",
    )]
    MayhemMode,
    #[strum(
        serialize = "/Game/GameData/Challenges/Character/Beastmaster/BP_Challenge_Beastmaster_ClassMod.BP_Challenge_Beastmaster_ClassMod_C",
    )]
    BeastMasterClassModSlot,
    #[strum(
        serialize = "/Game/GameData/Challenges/Character/Gunner/BP_Challenge_Gunner_ClassMod.BP_Challenge_Gunner_ClassMod_C",
    )]
    GunnerClassModSlot,
    #[strum(
        serialize = "/Game/GameData/Challenges/Character/Operative/BP_Challenge_Operative_ClassMod.BP_Challenge_Operative_ClassMod_C",
    )]
    OperativeClassModSlot,
    #[strum(
        serialize = "/Game/GameData/Challenges/Character/Siren/BP_Challenge_Siren_ClassMod.BP_Challenge_Siren_ClassMod_C",
    )]
    SirenClassModSlot,
}

impl std::fmt::Display for Challenge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Challenge::ArtifactSlot => t!("Challenge.ArtifactSlot"),
            Challenge::EridianAnalyzer => t!("Challenge.EridianAnalyzer"),
            Challenge::EridianResonator => t!("Challenge.EridianResonator"),
            Challenge::MayhemMode => t!("Challenge.MayhemMode"),
            Challenge::BeastMasterClassModSlot => t!("Challenge.BeastMasterClassModSlot"),
            Challenge::GunnerClassModSlot => t!("Challenge.GunnerClassModSlot"),
            Challenge::OperativeClassModSlot => t!("Challenge.OperativeClassModSlot"),
            Challenge::SirenClassModSlot => t!("Challenge.SirenClassModSlot"),
        })
    }
}
