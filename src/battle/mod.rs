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

#[derive(Clone, Copy, Debug)]
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

impl DisableStruct {
    pub fn new() -> Self {
        Self {
            transformed_mon_personality: 0,
            disabled_move: 0,
            encored_move: 0,
            protect_uses: 0,
            stockpile_counter: 0,
            substitute_hp: 0,
            disable_timer: 0,
            encored_move_pos: 0,
            encore_timer: 0,
            perish_song_timer: 0,
            fury_cutter_counter: 0,
            rollout_timer: 0,
            charge_timer: 0,
            taunt_timer: 0,
            battler_preventing_escape: 0,
            battler_with_sure_hit: 0,
            is_first_turn: 0,
            truant_counter: 0,
            mimicked_moves: 0,
            recharge_timer: 0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
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

impl ProtectStruct {
    pub fn new() -> Self {
        Self {
            protected: 0,
            endured: 0,
            no_valid_moves: 0,
            helping_hand: 0,
            bounce_move: 0,
            steal_move: 0,
            flag0_unknown: 0,
            prlz_immobility: 0,
            confusion_self_dmg: 0,
            target_not_affected: 0,
            charging_turn: 0,
            flee_type: 0,
            used_imprisoned_move: 0,
            love_immobility: 0,
            used_disabled_move: 0,
            used_taunted_move: 0,
            flag2_unknown: 0,
            flinch_immobility: 0,
            not_first_strike: 0,
            palace_unable_to_use_move: 0,
            physical_dmg: 0,
            special_dmg: 0,
            physical_battler_id: 0,
            special_battler_id: 0,
        }
    }
}

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

#[derive(Clone, Copy, Debug)]
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

impl WishFutureKnock {
    pub fn new() -> Self {
        Self {
            future_sight_counter: [0; MAX_BATTLERS_COUNT],
            future_sight_attacker: [0; MAX_BATTLERS_COUNT],
            future_sight_dmg: [0; MAX_BATTLERS_COUNT],
            future_sight_move: [0; MAX_BATTLERS_COUNT],
            wish_counter: [0; MAX_BATTLERS_COUNT],
            wish_mon_id: [0; MAX_BATTLERS_COUNT],
            weather_duration: 0,
            knocked_off_mons: [0; NUM_BATTLE_SIDES],
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ArtificialPlayerThinking {
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

impl ArtificialPlayerThinking {
    pub fn new() -> Self {
        Self {
            ai_state: 0,
            moveset_index: 0,
            move_considered: 0,
            score: [0; MAX_MON_MOVES],
            func_result: 0,
            ai_flags: 0,
            ai_action: 0,
            ai_logic_id: 0,
            filler12: [0; 6],
            simulated_rng: [0; MAX_MON_MOVES],
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UsedMoves {
    moves: [u16; MAX_MON_MOVES],
    uknown: [u8; MAX_MON_MOVES],
}

impl UsedMoves {
    pub fn new() -> Self {
        Self {
            moves: [0; MAX_MON_MOVES],
            uknown: [0; MAX_MON_MOVES],
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BattleHistory {
    used_moves: [UsedMoves; MAX_BATTLERS_COUNT],
    abilities: [u8; MAX_BATTLERS_COUNT],
    item_effects: [u8; MAX_BATTLERS_COUNT],
    trainer_items: [u16; NUM_BATTLE_SIDES],
    items_no: u8,
}

impl BattleHistory {
    pub fn new() -> Self {
        Self {
            used_moves: [UsedMoves::new(); MAX_BATTLERS_COUNT],
            abilities: [0; MAX_BATTLERS_COUNT],
            item_effects: [0; MAX_BATTLERS_COUNT],
            trainer_items: [0; NUM_BATTLE_SIDES],
            items_no: 0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BattleScriptStack {
    ptr: *mut [u8; 8],
    size: u8,
}

impl BattleScriptStack {
    pub fn new() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            size: 0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BattleCallbacksStack {
    func_stack: [fn(); 8],
    size: u8,
}

impl BattleCallbacksStack {
    pub fn new() -> Self {
        Self {
            func_stack: [|| {}; 8],
            size: 0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct StatsArray {
    state: [u16; NUM_STATS],
}

impl StatsArray {
    pub fn new() -> Self {
        Self {
            state: [0; NUM_STATS],
        }
    }
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

impl Battle {
    pub fn new() -> Self {
        Self {
            turn_effects_tracker: 0,
            turn_effects_battler_id: 0,
            unused_0: 0,
            turn_counters_tracker: 0,
            wrapped_move: [0; MAX_BATTLERS_COUNT * 2],
            move_target: [0; MAX_BATTLERS_COUNT],
            exp_getter_mon_id: 0,
            unused_1: 0,
            wild_victory_song: 0,
            dynamic_move_type: 0,
            wrapped_by: [0; MAX_BATTLERS_COUNT],
            assist_possible_moves: [0; PARTY_SIZE * MAX_MON_MOVES],
            focus_punch_battler_id: 0,
            battler_preventing_switchout: 0,
            money_multiplier: 0,
            saved_turn_action_number: 0,
            switch_in_abilities_counter: 0,
            fainted_actions_state: 0,
            fainted_actions_battler_id: 0,
            exp_value: 0,
            script_party_idx: 0,
            sent_in_rkmn: 0,
            selection_script_finished: [false; MAX_BATTLERS_COUNT],
            battler_party_indexes: [0; MAX_BATTLERS_COUNT],
            mon_to_switch_into_id: [0; MAX_BATTLERS_COUNT],
            battler_party_orders: [[0; PARTY_SIZE / 2]; MAX_BATTLERS_COUNT],
            run_tries: 0,
            caught_mon_nick: [0; RKMN_NAME_LENGTH + 1],
            unused_2: 0,
            safari_go_near_counter: 0,
            safari_rkbl_throw_counter: 0,
            safari_escape_factor: 0,
            safari_catch_factor: 0,
            link_battle_vs_sprite_id_v: 0,
            link_battle_vs_sprite_id_s: 0,
            form_to_change_into: 0,
            chosen_move_positions: [0; MAX_BATTLERS_COUNT],
            state_id_after_sel_script: [0; MAX_BATTLERS_COUNT],
            unused_3: [0; 3],
            prev_selected_party_slot: 0,
            unused_4: [0; 2],
            string_move_type: 0,
            exp_getter_battler_id: 0,
            unused_5: 0,
            absent_battler_flags: 0,
            palace_flags: 0,
            field_93: 0,
            wally_battle_state: 0,
            wally_moves_state: 0,
            wally_wait_frames: 0,
            wally_move_frames: 0,
            last_taken_move: [0; MAX_BATTLERS_COUNT * 2 * 2],
            hp_on_switchout: [0; NUM_BATTLE_SIDES],
            saved_battle_type_flags: 0,
            ability_preventing_switchout: 0,
            hp_scale: 0,
            synchronize_move_effect: 0,
            any_mon_has_transformed: false,
            saved_callback: None,
            used_held_items: [0; MAX_BATTLERS_COUNT],
            chosen_item: [0; MAX_BATTLERS_COUNT],
            ai_item_type: [0; 2],
            ai_item_flags: [0; 2],
            choiced_move: [0; MAX_BATTLERS_COUNT],
            changed_items: [0; MAX_BATTLERS_COUNT],
            intimidate_battler: 0,
            switch_in_items_counter: 0,
            arena_turn_counter: 0,
            turn_side_tracker: 0,
            unused_6: [0; 3],
            given_exp_mons: 0,
            last_taken_move_from: [[[0; 2]; MAX_BATTLERS_COUNT]; MAX_BATTLERS_COUNT],
            castform_palette: [[0; 16]; NUM_CASTFORM_FORMS],
            // multi_buffer: MultiBuffer::new(),
            wish_perish_song_state: 0,
            wish_perish_song_battler_id: 0,
            overworld_weather_done: false,
            atk_canceller_tracker: 0,
            ai_mon_to_switch_into_id: [0; MAX_BATTLERS_COUNT],
            arena_mind_points: [0; 2],
            arena_skill_points: [0; 2],
            arena_start_hp: [0; 2],
            arena_lost_player_mons: 0,
            arena_lost_opponent_mons: 0,
            already_statused_move_attempt: 0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BattleResources {
    // SecretBase
}

impl BattleResources {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Copy, Debug)]
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

impl BattleResults {
    pub fn new() -> Self {
        Self {
            player_faint_counter: 0,
            opponent_faint_counter: 0,
            player_switches_counter: 0,
            num_healing_items_used: 0,
            num_revives_used: 0,
            player_mon_was_damaged: 0,
            used_master_ball: 0,
            caught_mon_ball: 0,
            shiny_wild_mon: 0,
            player_mon1_species: 0,
            player_mon1_name: [0; 11],
            battle_turn_counter: 0,
            player_mon2_name: [0; 11],
            rkblock_throws: 0,
            last_opponent_species: 0,
            last_used_move_player: 0,
            last_used_move_opponent: 0,
            player_mon2_species: 0,
            caught_mon_species: 0,
            caught_mon_nick: [0; 11],
            filler35: 0,
            catch_attempts: [0; 7],
        }
    }
}
