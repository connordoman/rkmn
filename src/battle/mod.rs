use crate::{
    game::{PARTY_SIZE, RKMN_NAME_LENGTH},
    rkmn::NUM_STATS,
};

pub mod battle_main;
mod move_data;
pub mod setup;
mod side_timer;
pub mod state;

pub const MAX_BATTLERS_COUNT: usize = 4;
pub const NUM_BATTLE_SIDES: usize = 2;

pub const MAX_MON_MOVES: usize = 4;

pub const NUM_CASTFORM_FORMS: usize = 4;

pub struct DisableStruct {
    transformed_mon_personality: u32,
    disabled_move: u16,
    encored_move: u16,
    protect_uses: u8,
    stockpile_counter: u8,
    substitute_hp: u8,
    disable_timer: u8, // Consider using bitflags crate for bitfields
    encored_move_pos: u8,
    encore_timer: u8,      // Consider using bitflags crate for bitfields
    perish_song_timer: u8, // Consider using bitflags crate for bitfields
    fury_cutter_counter: u8,
    rollout_timer: u8, // Consider using bitflags crate for bitfields
    charge_timer: u8,  // Consider using bitflags crate for bitfields
    taunt_timer: u8,   // Consider using bitflags crate for bitfields
    battler_preventing_escape: u8,
    battler_with_sure_hit: u8,
    is_first_turn: u8,
    truant_counter: u8, // Consider using bitflags crate for bitfields
    mimicked_moves: u8,
    recharge_timer: u8,
}

pub struct ProtectStruct {
    protected: u8,
    endured: u8,
    no_valid_moves: u8,
    helping_hand: u8,
    bounce_move: u8,
    steal_move: u8,
    flag0_unknown: u8,
    prlz_immobility: u8,
    confusion_self_dmg: u8,
    target_not_affected: u8,
    charging_turn: u8,
    flee_type: u8,
    used_imprisoned_move: u8,
    love_immobility: u8,
    used_disabled_move: u8,
    used_taunted_move: u8,
    flag2_unknown: u8,
    flinch_immobility: u8,
    not_first_strike: u8,
    palace_unable_to_use_move: u8,
    physical_dmg: u8,
    special_dmg: u8,
    physical_battler_id: u8,
    special_battler_id: u8,
}

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

pub struct WishFutureKnock {
    future_sight_counter: [u8; MAX_BATTLERS_COUNT],
    future_sight_attacker: [u8; MAX_BATTLERS_COUNT],
    future_sight_dmg: [i32; MAX_BATTLERS_COUNT],
    future_sight_move: [u16; MAX_BATTLERS_COUNT],
    wish_counter: [u8; MAX_BATTLERS_COUNT],
    wish_mon_id: [u8; MAX_BATTLERS_COUNT],
    weather_duration: u8,
    knocked_off_mons: [u8; NUM_BATTLE_SIDES],
}

pub struct ArticialPlayerThinking {
    ai_state: u8,
    moveset_index: u8,
    move_considered: u16,
    score: [i8; MAX_MON_MOVES],
    func_result: u32,
    ai_flags: u32,
    ai_action: u8,
    ai_logic_id: u8,
    filler12: [u8; 6],
    simulated_rng: [u8; MAX_MON_MOVES],
}

pub struct UsedMoves {
    moves: [u16; MAX_MON_MOVES],
    uknown: [u8; MAX_MON_MOVES],
}

pub struct BattleHistory {
    used_moves: [UsedMoves; MAX_BATTLERS_COUNT],
    abilities: [u8; MAX_BATTLERS_COUNT],
    item_effects: [u8; MAX_BATTLERS_COUNT],
    trainer_items: [u16; NUM_BATTLE_SIDES],
    items_no: u8,
}

pub struct BattleScriptStack {
    ptr: *mut [u8; 8],
    size: u8,
}

pub struct BattleCallbacksStack {
    func_stack: [fn(); 8],
    size: u8,
}

pub struct StatsArray {
    state: [u16; NUM_STATS],
}

pub type BattlerMoves = [u16; MAX_BATTLERS_COUNT];

#[derive(Default, Debug)]
pub struct Battle {
    turn_effects_tracker: u8,
    turn_effects_battler_id: u8,
    unused_0: u8,
    turn_counters_tracker: u8,
    wrapped_move: [u8; MAX_BATTLERS_COUNT * 2],
    move_target: [u8; MAX_BATTLERS_COUNT],
    exp_getter_mon_id: u8,
    unused_1: u8,
    wild_victory_song: u8,
    dynamic_move_type: u8,
    wrapped_by: [u8; MAX_BATTLERS_COUNT],
    assist_possible_moves: [u16; PARTY_SIZE * MAX_MON_MOVES],
    focus_punch_battler_id: u8,
    battler_preventing_switchout: u8,
    money_multiplier: u8,
    saved_turn_action_number: u8,
    switch_in_abilities_counter: u8,
    fainted_actions_state: u8,
    fainted_actions_battler_id: u8,
    exp_value: u16,
    script_party_idx: u8,
    sent_in_rkmn: u8,
    selection_script_finished: [bool; MAX_BATTLERS_COUNT],
    battler_party_indexes: [u8; MAX_BATTLERS_COUNT],
    mon_to_switch_into_id: [u8; MAX_BATTLERS_COUNT],
    battler_party_orders: [[u8; PARTY_SIZE / 2]; MAX_BATTLERS_COUNT],
    run_tries: u8,
    caught_mon_nick: [u8; RKMN_NAME_LENGTH + 1],
    unused_2: u8,
    safari_go_near_counter: u8,
    safari_rkbl_throw_counter: u8,
    safari_escape_factor: u8,
    safari_catch_factor: u8,
    link_battle_vs_sprite_id_v: u8,
    link_battle_vs_sprite_id_s: u8,
    form_to_change_into: u8,
    chosen_move_positions: [u8; MAX_BATTLERS_COUNT],
    state_id_after_sel_script: [u8; MAX_BATTLERS_COUNT],
    unused_3: [u8; 3],
    prev_selected_party_slot: u8,
    unused_4: [u8; 2],
    string_move_type: u8,
    exp_getter_battler_id: u8,
    unused_5: u8,
    absent_battler_flags: u8,
    palace_flags: u8,
    field_93: u8,
    wally_battle_state: u8,
    wally_moves_state: u8,
    wally_wait_frames: u8,
    wally_move_frames: u8,
    last_taken_move: [u8; MAX_BATTLERS_COUNT * 2 * 2],
    hp_on_switchout: [u16; NUM_BATTLE_SIDES],
    saved_battle_type_flags: u32,
    ability_preventing_switchout: u8,
    hp_scale: u8,
    synchronize_move_effect: u8,
    any_mon_has_transformed: bool,
    saved_callback: Option<fn()>,
    used_held_items: [u16; MAX_BATTLERS_COUNT],
    chosen_item: [u8; MAX_BATTLERS_COUNT],
    ai_item_type: [u8; 2],
    ai_item_flags: [u8; 2],
    choiced_move: [u16; MAX_BATTLERS_COUNT],
    changed_items: [u16; MAX_BATTLERS_COUNT],
    intimidate_battler: u8,
    switch_in_items_counter: u8,
    arena_turn_counter: u8,
    turn_side_tracker: u8,
    unused_6: [u8; 3],
    given_exp_mons: u8,
    last_taken_move_from: [[[u8; 2]; MAX_BATTLERS_COUNT]; MAX_BATTLERS_COUNT],
    castform_palette: [[u16; 16]; NUM_CASTFORM_FORMS],
    // multi_buffer: MultiBuffer,
    wish_perish_song_state: u8,
    wish_perish_song_battler_id: u8,
    overworld_weather_done: bool,
    atk_canceller_tracker: u8,
    ai_mon_to_switch_into_id: [u8; MAX_BATTLERS_COUNT],
    arena_mind_points: [i8; 2],
    arena_skill_points: [i8; 2],
    arena_start_hp: [u16; 2],
    arena_lost_player_mons: u8,
    arena_lost_opponent_mons: u8,
    already_statused_move_attempt: u8,
}

pub struct BattleResources {
    // SecretBase
}

pub struct BattleResults {
    player_faint_counter: u8,
    opponent_faint_counter: u8,
    player_switches_counter: u8,
    num_healing_items_used: u8,
    num_revives_used: u8,
    player_mon_was_damaged: u8,
    used_master_ball: u8,
    caught_mon_ball: u8,
    shiny_wild_mon: u8,
    player_mon1_species: u16,
    player_mon1_name: [u8; 11],
    battle_turn_counter: u8,
    player_mon2_name: [u8; 11],
    rkblock_throws: u8,
    last_opponent_species: u16,
    last_used_move_player: u16,
    last_used_move_opponent: u16,
    player_mon2_species: u16,
    caught_mon_species: u16,
    caught_mon_nick: [u8; 11],
    filler35: u8,
    catch_attempts: [u8; 7],
}
