#[derive(Clone, Copy, Debug)]
pub struct SpecialStatus {
    stat_lowered: u8,
    lightning_rod_redirected: u8,
    restored_battler_sprite: u8,
    intimidated_mon: u8,
    traced: u8,
    pp_not_affected_by_pressure: u8,
    fainted_has_replacement: u8,
    focus_banded: u8,
    shell_bell_dmg: i32,
    physical_dmg: i32,
    special_dmg: i32,
    physical_battler_id: u8,
    special_battler_id: u8,
}

impl SpecialStatus {
    pub fn new() -> Self {
        Self {
            stat_lowered: 0,
            lightning_rod_redirected: 0,
            restored_battler_sprite: 0,
            intimidated_mon: 0,
            traced: 0,
            pp_not_affected_by_pressure: 0,
            fainted_has_replacement: 0,
            focus_banded: 0,
            shell_bell_dmg: 0,
            physical_dmg: 0,
            special_dmg: 0,
            physical_battler_id: 0,
            special_battler_id: 0,
        }
    }
}
