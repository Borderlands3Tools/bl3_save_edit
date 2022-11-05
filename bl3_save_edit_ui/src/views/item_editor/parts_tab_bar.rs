#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AvailablePartType {
    Parts,
    Anointments,
}

impl std::default::Default for AvailablePartType {
    fn default() -> Self {
        Self::Parts
    }
}

impl std::fmt::Display for AvailablePartType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            AvailablePartType::Parts => t!("inventory.available.tab_parts"),
            AvailablePartType::Anointments => t!("inventory.available.tab_anointments"),
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CurrentPartType {
    Parts,
    Anointments,
}

impl std::default::Default for CurrentPartType {
    fn default() -> Self {
        Self::Parts
    }
}

impl std::fmt::Display for CurrentPartType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CurrentPartType::Parts => t!("inventory.current.tab_parts"),
            CurrentPartType::Anointments => t!("inventory.current.tab_anointments"),
        })
    }
}
