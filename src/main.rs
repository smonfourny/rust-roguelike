use bracket_lib::prelude::*;
use specs::prelude::*;
use std::cmp::{max, min};

mod components;
mod constants;
mod map;

use components::{Player, Position, Renderable};
use constants::{BASE_BG_COLOR, MAP_X, MAP_Y, PLAYER_COLOR};
use map::{draw_map, Map, TileType};

struct State {
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        player_input(self, ctx);

        let map = self.ecs.fetch::<Vec<Vec<TileType>>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Explore").build()?;
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    let map = Map::new_map(MAP_X, MAP_Y);
    gs.ecs.insert(map.tiles);
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(PLAYER_COLOR),
            bg: RGB::named(BASE_BG_COLOR),
        })
        .with(Player {})
        .build();

    main_loop(context, gs)
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<Vec<TileType>>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        match map[(pos.x + delta_x) as usize][(pos.y + delta_y) as usize] {
            TileType::Wall => {}
            TileType::Floor => {
                pos.x = min(79, max(0, pos.x + delta_x));
                pos.y = min(49, max(0, pos.y + delta_y));
            }
        }
    }
}

fn player_input(gs: &mut State, ctx: &mut BTerm) {
    match ctx.key {
        None => {}
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
            _ => {}
        },
    }
}
