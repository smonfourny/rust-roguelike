use bracket_lib::prelude::*;
use specs::prelude::*;
use std::cmp::{max, min};

mod ai;
mod components;
mod constants;
mod map;
mod map_indexing;
mod rect;
mod visibility;

use ai::MonsterAI;
use components::{BlocksTile, Monster, Name, Player, Position, Renderable, Viewshed};
use constants::{BASE_BG_COLOR, BROWN_SHIRT_COLOR, MAP_X, MAP_Y, PLAYER_COLOR};
use map::{draw_map, Map};
use map_indexing::MapIndexingSystem;
use visibility::VisibilitySystem;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running,
}

struct State {
    ecs: World,
    runstate: RunState,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);
        let mut map_index = MapIndexingSystem {};
        map_index.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused;
        } else {
            self.runstate = player_input(self, ctx);
        }

        // let map = self.ecs.fetch::<Map>();
        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            if map.visible_tiles[pos.x as usize][pos.y as usize] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Explore").build()?;
    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Running,
    };
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<Viewshed>();

    let map = Map::new_map(MAP_X, MAP_Y);
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
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name {
            name: "Player".to_string(),
        })
        .build();

    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        let mut rng = RandomNumberGenerator::new();
        let glyph: FontCharType;
        let name: String;
        match rng.roll_dice(1, 2) {
            1 => {
                glyph = to_cp437('f');
                name = "Fascist".to_string();
            }
            _ => {
                glyph = to_cp437('n');
                name = "Nazi".to_string();
            }
        };

        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph,
                fg: RGB::named(BROWN_SHIRT_COLOR),
                bg: RGB::named(BASE_BG_COLOR),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 6,
                dirty: true,
            })
            .with(Monster {})
            .with(Name {
                name: format!("{} {}", &name, i),
            })
            .with(BlocksTile {})
            .build();
    }
    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));

    main_loop(context, gs)
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewshed = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewshed).join() {
        if !map.blocked[(pos.x + delta_x) as usize][(pos.y + delta_y) as usize] {
            pos.x = min(MAP_X - 1, max(0, pos.x + delta_x));
            pos.y = min(MAP_Y - 1, max(0, pos.y + delta_y));

            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;

            viewshed.dirty = true;
        }
    }
}

fn player_input(gs: &mut State, ctx: &mut BTerm) -> RunState {
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
