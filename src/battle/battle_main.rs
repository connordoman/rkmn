use crate::{game::*, state::GlobalState};

use super::{data::BattleData, state::BattleState, Battle};

pub fn init_battle(
    g: &mut GlobalState,
    battle: &mut Battle,
    battle_data: &mut BattleData,
    battle_state: &mut BattleState,
) -> () {
    // reset heap
    // allocate battle resources
    // allocate battle sprites data
    // allocate mon spritex graphics
    // recorded battle clear frontier pass flag

    // if multi battle
    //      if recorded battle
    //          init battle internal
    //      if partner battle
    //      else pre init multi battle
    // else
    //      init battle internal
    // g.set_main_callback(init_battle_internal);
    battle_state.set_battle_state(BattleState::InitBattleInternal);
}

pub fn init_battle_internal(
    g: &mut GlobalState,
    battle: &mut Battle,
    battle_data: &mut BattleData,
    battle_state: &mut BattleState,
) -> () {
    // set hblank to null
    // set vblank to null

    // clear vram

    // reset gpu registers

    // scanline effects:
    // if partner battle not with steven:
    // else:
    //      ...

    // set battle terrain...

    // init battle bgs video
    // load battle textbox and bg
    // reset sprite data
    // reset tasks
    // draw battle entry background
    // free all sprite palettes
    // reset reserved sprite count to max_battlers
    // set vblank callback to battle vblank
    // -> set battle vars

    // -> set main callback to handle start battle
    // g.set_main_callback(handle_start_battle);
    battle_state.set_battle_state(BattleState::StartBattle);

    // -> set in battle flag to TRUE
    // battle_data.in_battle = true;

    // adjust friendship for every party member
}

pub fn get_multiplayer_id() -> u8 {
    0
}

pub fn handle_start_battle(
    g: &mut GlobalState,
    battle: &mut Battle,
    battle_data: &mut BattleData,
    battle_state: &mut BattleState,
) {
    let mut player_multiplayer_id: u8 = 0;
    let mut enemy_multiplayer_id: u8 = 0;

    // run tasks

    // animate sprites

    // build dam buffer

    player_multiplayer_id = get_multiplayer_id();
    // global battle scripting multplayer_id = multiplayer_id
    // g.battle_scripting.multiplayer_id = player_multiplayer_id;
    battle_state.set_battle_state(BattleState::Main);

    // enemy_multiplayer_id = player_multiplayer_id ^ BIT_SIDE;
}

pub fn battle_main_callback(
    g: &mut GlobalState,
    battle: &mut Battle,
    battle_data: &mut BattleData,
    battle_state: &mut BattleState,
) {
    for battler in 0..battle_data.num_battlers() {
        println!("Battler: {}", battler);
    }

    battle_state.set_battle_state(BattleState::RunScripts);
}

pub fn battle_turn_passed(
    g: &mut GlobalState,
    battle: &mut Battle,
    battle_data: &mut BattleData,
    battle_state: &mut BattleState,
) -> () {
    battle.increment_turn();

    let turn = battle.num_turns();
    println!("Finished turn {}", turn);

    if turn == 5 {
        println!("Turns exceeded limit, ending battle...");
        battle_state.set_battle_state(BattleState::End);
    } else {
        battle_state.set_battle_state(BattleState::Main);
    }
}
