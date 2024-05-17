use std::io::{self, Write};

#[derive(Debug, Clone)]
pub enum BattleAction {
    None,
    Fight,
    Item,
    Switch,
    Run,
}
pub fn get_battle_action() -> BattleAction {
    let actions = vec![
        BattleAction::Fight,
        BattleAction::Item,
        BattleAction::Switch,
        BattleAction::Run,
    ];

    loop {
        println!("Choose an action:");

        for (i, action) in actions.iter().enumerate() {
            match action {
                BattleAction::Fight => println!("{}: Fight", i),
                BattleAction::Item => println!("{}: Item", i),
                BattleAction::Switch => println!("{}: Switch", i),
                BattleAction::Run => println!("{}: Run", i),
                _ => (),
            }
        }

        let mut input = String::new();
        io::stdout().flush().expect("Failed to flush stdout.");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input.");

        match input.trim().parse::<usize>() {
            Ok(i) if i < actions.len() => return actions[i].clone(),
            _ => println!("Invalid selection, please try again."),
        }
    }
}
