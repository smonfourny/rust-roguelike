use bracket_lib::prelude::*;
use specs::prelude::*;

mod ai;
mod components;
#[allow(dead_code)]
mod constants;
mod damage_system;
mod gamelog;
mod map;
mod map_indexing;
mod melee_system;
mod player;
mod rect;
mod ui;
mod visibility;

use ai::MonsterAI;
use components::{
    BlocksTile, CombatStats, Monster, Name, Player, Position, Renderable, SufferDamage, Viewshed,
    WantsToMelee,
};
use constants::*;
use damage_system::DamageSystem;
use gamelog::GameLog;
use map::{draw_map, Map};
use map_indexing::MapIndexingSystem;
use melee_system::MeleeCombatSystem;
use player::player_input;
use visibility::VisibilitySystem;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
    Dead
}

pub struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);
        let mut map_index = MapIndexingSystem {};
        map_index.run_now(&self.ecs);
        let mut melee = MeleeCombatSystem {};
        melee.run_now(&self.ecs);
        let mut damage = DamageSystem {};
        damage.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        let mut new_runstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            new_runstate = *runstate;
        }

        match new_runstate {
            RunState::PreRun => {
                self.run_systems();
                new_runstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                new_runstate = player_input(self, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                new_runstate = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                new_runstate = RunState::AwaitingInput;
            }
            RunState::Dead => { }
        }

        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = new_runstate;
        }

        damage_system::delete_dead(&mut self.ecs);

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            if map.visible_tiles[pos.x as usize][pos.y as usize] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }

        ui::draw_ui(&self.ecs, ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Explore").build()?;
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<SufferDamage>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<WantsToMelee>();

    let map = Map::new_map(MAP_X, MAP_Y);
    let (player_x, player_y) = map.rooms[0].center();
    let player_entity = gs
        .ecs
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

    gs.ecs.insert(player_entity);

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
                attack: 4,
            })
            .with(BlocksTile {})
            .build();
    }
    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));
    gs.ecs.insert(RunState::PreRun);
    gs.ecs.insert(gamelog::GameLog{ entries: vec!["Welcome, traveller.".to_string()] });

    main_loop(context, gs)
}
