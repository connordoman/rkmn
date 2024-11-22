use crate::v2::{battle::BattleType, Game};

use super::SpecialTrainerIds;

pub fn calculate_trainer_reward_money(g: Game, trainer_id: usize) -> i32 {
    let money_reward: i32;
    let mut last_mon_level: i32 = 0;
    let mut i = 0;

    if trainer_id == SpecialTrainerIds::SecretBase.value() {
        // here would be from the party loaded into the shared secret base memory, and the money from the global battle struct
        money_reward = 20 * 25 * g.battle.money_multiplier;
    } else {
        // here we pattern match and find which party archetype the trainer has, and then extract relevant rkmn data

        let mut reward_factor = 1;

        while let Some(t) = g.trainers.get(i) {
            if t.trainer_id == trainer_id {
                if let Some(last_mon) = t.party.get_last() {
                    last_mon_level = last_mon.level as i32
                }
                reward_factor = t.class.reward_factor();
                break;
            }
            i += 1;
        }

        // if g.battle.battle_type.flags() & BattleType::TwoOpponents.flags() != 0 {
        //     money_reward = 4 * last_mon_level * g.battle.money_multiplier * reward_factor;
        // } else
        if g.battle.battle_type.flags() & BattleType::Double.flags() != 0 {
            money_reward = 4 * last_mon_level * g.battle.money_multiplier * 2 * reward_factor;
        } else {
            money_reward = 4 * last_mon_level * g.battle.money_multiplier * reward_factor;
        }
    }

    money_reward
}
