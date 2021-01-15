use bracket_lib::prelude::*;
use specs::prelude::*;

mod ai;
mod components;
mod constants;
mod map;
mod map_indexing;
mod player;
mod rect;
mod visibility;

use ai::MonsterAI;
use components::{BlocksTile, CombatStats, Monster, Name, Player, Position, Renderable, Viewshed};
use constants::{BASE_BG_COLOR, BROWN_SHIRT_COLOR, MAP_X, MAP_Y, PLAYER_COLOR};
use map::{draw_map, Map};
use map_indexing::MapIndexingSystem;
use player::{player_input};
use visibility::VisibilitySystem;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running,
}

pub struct State {
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
    gs.ecs.register::<CombatStats>();
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
        .with(CombatStats {
            max_hp: 30,
            hp: 30,
            defense: 2,
            attack: 5,
        })
        .build();

    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        let mut rng = RandomNumberGenerator::new();
        let glyph: FontCharType;
        let name: String;
        match rng.roll_dice(1, 2) {
            1 => {
                glyph = to_cp437('o');
                name = "Orc".to_string();
            }
            _ => {
                glyph = to_cp437('g');
                name = "Goblin".to_string();
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
            .with(CombatStats {
                max_hp: 15,
                hp: 15,
                defense: 1,
                attack: 4
            })
            .with(BlocksTile {})
            .build();
    }
    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));

    main_loop(context, gs)
}
