pub enum BattleState {
    Initializing,
    Main,
    RunScripts,
    TurnPassed,
    End,
}

impl BattleState {
    pub fn new() -> Self {
        BattleState::Initializing
    }
}
