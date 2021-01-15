
use bracket_lib::prelude::*;
use specs::prelude::*;
use std::cmp::{max, min};

use super::{BlocksTile, CombatStats, Map, Monster, Name, Player, Position, RunState, State, Viewshed};
use super::{MAP_X, MAP_Y};

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewshed = ecs.write_storage::<Viewshed>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewshed).join() {
        let destination_x = pos.x + delta_x;
        let destination_y = pos.y + delta_y;

        for potential_target in map.tile_content[destination_x as usize][destination_y as usize].iter() {
            let target = combat_stats.get(*potential_target);
            match target {
                None => {},
                Some(t) => {
                    console::log(&format!("HIYAA"));
                    return;
                }
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
        None => return RunState::Paused,
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
            _ => return RunState::Paused,
        },
    }
    RunState::Running
}
