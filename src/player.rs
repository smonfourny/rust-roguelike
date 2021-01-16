use bracket_lib::prelude::*;
use specs::prelude::*;
use std::cmp::{max, min};

use super::{CombatStats, GameLog, Item, Map, Player, Position, RunState, State, Viewshed, WantsToDisplayContent, WantsToMelee, WantsToPickupItem };
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
    let items = ecs.read_storage::<Item>();
    let entities = ecs.entities();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();
    let mut wants_to_display = ecs.write_storage::<WantsToDisplayContent>();
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
            if target.is_some() {
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

            let item = items.get(*potential_target);
            if item.is_some() {
                wants_to_display
                    .insert(
                        *potential_target,
                        WantsToDisplayContent {}
                    )
                    .expect("Add target failed");
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

fn get_item(ecs: &mut World) {
    let player_pos = ecs.fetch::<Point>();
    let player_entity = ecs.fetch::<Entity>();
    let entities = ecs.entities();
    let items = ecs.read_storage::<Item>();
    let positions = ecs.read_storage::<Position>();
    let mut gamelog = ecs.fetch_mut::<GameLog>();

    let mut target_item: Option<Entity> = None;
    for (item_entity, _item, position) in (&entities, &items, &positions).join() {
        if position.x == player_pos.x && position.y == player_pos.y {
            target_item = Some(item_entity);
        }
    }

    match target_item {
        None => gamelog.entries.push("There is nothing to pick up here.".to_string()),
        Some(item) => {
            let mut pickup = ecs.write_storage::<WantsToPickupItem>();
            pickup.insert(*player_entity, WantsToPickupItem{ collected_by: *player_entity, item }).expect("Unable to insert want to pickup");
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
            VirtualKeyCode::G => get_item(&mut gs.ecs),
            _ => return RunState::AwaitingInput,
        },
    }
    RunState::PlayerTurn
}
