use crate::{
    game::{PARTY_SIZE, RKMN_NAME_LENGTH},
    rkmn::NUM_STATS,
};

pub mod battle_ai;
pub mod battle_history;
// pub mod battle_main;
pub mod battle_start;
pub mod battle_state;
pub mod data;
pub mod move_data;
pub mod scripting;
pub mod side_timer;
pub mod special_status;
pub mod state;
pub mod struct_disable;
pub mod struct_protect;
pub mod wish_future_knock;

pub const MAX_BATTLERS_COUNT: usize = 4;
pub const DEFAULT_BATTLERS_COUNT: usize = 2;
pub const NUM_BATTLE_SIDES: usize = 2;

pub const MAX_MON_MOVES: usize = 4;

pub const NUM_CASTFORM_FORMS: usize = 4;

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

    pub fn increment_turn(&mut self) {
        self.turn_counters_tracker += 1
    }

    pub fn num_turns(&self) -> u8 {
        self.turn_counters_tracker
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
