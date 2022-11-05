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
        match self {
            AvailablePartType::Parts => write!(f, "{}", t!("inventory.available.tab_parts")),
            AvailablePartType::Anointments => write!(f, "{}", t!("inventory.available.tab_anointments")),
        }
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
        match self {
            CurrentPartType::Parts => write!(f, "{}", t!("inventory.current.tab_parts")),
            CurrentPartType::Anointments => write!(f, "{}", t!("inventory.current.tab_anointments")),
        }
    }
}
