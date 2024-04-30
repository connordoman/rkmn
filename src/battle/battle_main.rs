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
    let mut i;

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

    // -> set in battle flag to TRUE
    g.enter_battle();
}
