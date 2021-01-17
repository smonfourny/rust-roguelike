use super::{CombatStats, GameLog, Player, RandomNumberGenerator, RunState, SufferDamage};
use specs::prelude::*;

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
        ReadExpect<'a, Entity>,
        Entities<'a>,
        ReadStorage<'a, Player>,
        WriteExpect<'a, GameLog>,
        WriteExpect<'a, RandomNumberGenerator>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut stats, mut damage, player, entities, players, mut gamelog, mut rng) = data;

        let mut exp_gain = 0;

        for (mut stats, damage, entity) in (&mut stats, &damage, &entities).join() {
            stats.hp -= damage.amount.iter().sum::<i32>();

            if stats.hp < 1 {
                let player = players.get(entity);
                match player {
                    None => { exp_gain += 15 * stats.level; },
                    Some(_) => {}
                }
            }
        }

        if let Some(player_stats) = stats.get_mut(*player) {
            let mut new_exp = player_stats.exp + exp_gain;
            while new_exp >= 100 * player_stats.level {
                new_exp -= 100 * player_stats.level;
                player_stats.level += 1;

                // Increase stats
                player_stats.strength += rng.roll_dice(1, 2) - 1;
                player_stats.agility += rng.roll_dice(1, 2) - 1;
                player_stats.vitality += rng.roll_dice(1, 2) - 1;
                player_stats.magic += rng.roll_dice(1, 2) - 1;

                player_stats.max_hp = player_stats.vitality * 5;

                gamelog.entries.push("You feel stronger.".to_string());
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
