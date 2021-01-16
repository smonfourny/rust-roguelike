use bracket_lib::prelude::*;
use specs::prelude::*;
use std::cmp::{max, min};

use super::{CombatStats, Map, Player, Position, RunState, State, Viewshed, WantsToMelee};
use super::{MAP_X, MAP_Y};

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let runstate = ecs.fetch::<RunState>();
    if RunState::Dead == *runstate {
        return;
    }

    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewshed = ecs.write_storage::<Viewshed>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let entities = ecs.entities();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();
    let map = ecs.fetch::<Map>();

    for (entity, _player, pos, viewshed) in
        (&entities, &mut players, &mut positions, &mut viewshed).join()
    {
        let destination_x = pos.x + delta_x;
        let destination_y = pos.y + delta_y;

        for potential_target in
            map.tile_content[destination_x as usize][destination_y as usize].iter()
        {
            let target = combat_stats.get(*potential_target);
            if let Some(_target) = target {
                wants_to_melee
                    .insert(
                        entity,
                        WantsToMelee {
                            target: *potential_target,
                        },
                    )
                    .expect("Add target failed");
                return;
            }
        }

        if !map.blocked[destination_x as usize][destination_y as usize] {
            pos.x = min(MAP_X - 1, max(0, destination_x));
            pos.y = min(MAP_Y - 1, max(0, destination_y));

            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;

            viewshed.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut BTerm) -> RunState {
    match ctx.key {
        None => return RunState::AwaitingInput,
        Some(key) => match key {
            VirtualKeyCode::Numpad4 | VirtualKeyCode::H | VirtualKeyCode::Left => {
                try_move_player(-1, 0, &mut gs.ecs)
            }
            VirtualKeyCode::Numpad6 | VirtualKeyCode::L | VirtualKeyCode::Right => {
                try_move_player(1, 0, &mut gs.ecs)
            }
            VirtualKeyCode::Numpad8 | VirtualKeyCode::K | VirtualKeyCode::Up => {
                try_move_player(0, -1, &mut gs.ecs)
            }
            VirtualKeyCode::Numpad2 | VirtualKeyCode::J | VirtualKeyCode::Down => {
                try_move_player(0, 1, &mut gs.ecs)
            }
            _ => return RunState::AwaitingInput,
        },
    }
    RunState::PlayerTurn
}
