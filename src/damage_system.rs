use super::{CombatStats, GameLog, Player, RunState, SufferDamage};
use specs::prelude::*;

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut stats, mut damage, player, mut gamelog) = data;

        let mut exp_gain = 0;

        for (mut stats, damage) in (&mut stats, &damage).join() {
            stats.hp -= damage.amount.iter().sum::<i32>();

            if stats.hp < 1 {
                // TODO separate this, as this is likely not the right place to do this
                exp_gain += 120 * stats.level;
            }
        }

        // TODO separate this, as this is likely not the right place to do this
        if let Some(player_stats) = stats.get_mut(*player) {
            let mut new_exp = player_stats.exp + exp_gain;
            while new_exp >= 100 * player_stats.level {
                new_exp -= 100 * player_stats.level;
                player_stats.level += 1;
                gamelog
                    .entries
                    .push(format!("You reached level {}!", player_stats.level));
            }
            player_stats.exp = new_exp;
        }

        damage.clear();
    }
}

pub fn delete_dead(ecs: &mut World) {
    let mut dead: Vec<Entity> = Vec::new();
    {
        let combat_stats = ecs.read_storage::<CombatStats>();
        let players = ecs.read_storage::<Player>();
        let entities = ecs.entities();
        for (entity, stats) in (&entities, &combat_stats).join() {
            if stats.hp < 1 {
                let player = players.get(entity);
                match player {
                    None => dead.push(entity),
                    Some(_) => {
                        let mut gamelog = ecs.write_resource::<GameLog>();
                        let mut runwriter = ecs.write_resource::<RunState>();
                        if *runwriter != RunState::Dead {
                            *runwriter = RunState::Dead;
                            gamelog.entries.push("You are dead!".to_string());
                        }
                    }
                }
            }
        }
    }

    for victim in dead {
        ecs.delete_entity(victim).expect("Unable to delete");
    }
}
