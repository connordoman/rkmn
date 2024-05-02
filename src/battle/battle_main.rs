use crate::game_state::*;

fn init_battle() -> () {
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
}

fn init_battle_internal(g: &mut GameState) -> () {
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
    g.set_main_callback(handle_start_battle);

    // -> set in battle flag to TRUE
    g.state_mut().enter_battle();

    // adjust friendship for every party member
}

fn get_multiplayer_id() -> u8 {
    0
}

fn handle_start_battle(g: &mut GameState) {
    let mut player_multiplayer_id: u8 = 0;
    let mut enemy_multiplayer_id: u8 = 0;

    // run tasks

    // animate sprites

    // build dam buffer

    player_multiplayer_id = get_multiplayer_id();
    // global battle scripting multplayer_id = multiplayer_id
    // g.battle_scripting.multiplayer_id = player_multiplayer_id;

    // enemy_multiplayer_id = player_multiplayer_id ^ BIT_SIDE;
}

fn battle_main_callback(g: &mut GameState) {
    for active_battler in 0..g.active_battlers_count() {
        // g.battle_controller_funcs[active_battler]();
    }
}
