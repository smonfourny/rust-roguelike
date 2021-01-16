use super::{
    BlocksTile, CombatStats, HealEffect, Item, Monster, Name, Player, Position, Rect, Renderable,
    Viewshed, BASE_BG_COLOR, BROWN_SHIRT_COLOR, MAX_ITEMS_PER_ROOM, MAX_MONSTERS_PER_ROOM,
    PLAYER_COLOR, PURPLE_COLOR,
};
use bracket_lib::prelude::*;
use specs::prelude::*;

// Spawns player in the specified location and returns entity
pub fn spawn_player(ecs: &mut World, player_x: i32, player_y: i32) -> Entity {
    ecs.create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(PLAYER_COLOR),
            bg: RGB::named(BASE_BG_COLOR),
            render_order: 0,
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
        .build()
}

pub fn spawn_room_contents(ecs: &mut World, room: &Rect) {
    let mut monster_spawn_points: Vec<(usize, usize)> = Vec::new();
    let mut item_spawn_points: Vec<(usize, usize)> = Vec::new();

    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        let num_monsters = rng.roll_dice(1, MAX_MONSTERS_PER_ROOM + 2) - 3;
        let num_items = rng.roll_dice(1, MAX_ITEMS_PER_ROOM + 2) - 3;

        for _i in 0..num_monsters {
            loop {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                if !monster_spawn_points.contains(&(x, y)) {
                    monster_spawn_points.push((x, y));
                    break;
                }
            }
        }

        for _i in 0..num_items {
            loop {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                if !item_spawn_points.contains(&(x, y)) {
                    item_spawn_points.push((x, y));
                    break;
                }
            }
        }
    }

    for (x, y) in monster_spawn_points.iter() {
        random_monster(ecs, *x as i32, *y as i32);
    }

    for (x, y) in item_spawn_points.iter() {
        health_potion(ecs, *x as i32, *y as i32);
    }
}

fn random_monster(ecs: &mut World, x: i32, y: i32) {
    let result;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        result = rng.roll_dice(1, 2);
    }

    match result {
        1 => skeleton(ecs, x, y),
        _ => goblin(ecs, x, y),
    };
}

fn skeleton(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, to_cp437('s'), "Skeleton");
}
fn goblin(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, to_cp437('g'), "Goblin");
}

fn monster<S: ToString>(ecs: &mut World, x: i32, y: i32, glyph: FontCharType, name: S) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph,
            fg: RGB::named(BROWN_SHIRT_COLOR),
            bg: RGB::named(BASE_BG_COLOR),
            render_order: 1
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 6,
            dirty: true,
        })
        .with(Monster {})
        .with(Name {
            name: name.to_string(),
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

fn health_potion(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: to_cp437('i'),
            fg: RGB::named(PURPLE_COLOR),
            bg: RGB::named(BASE_BG_COLOR),
            render_order: 2
        })
        .with(Name {
            name: "Health Potion".to_string(),
        })
        .with(Item {})
        .with(HealEffect { amount: 8 })
        .build();
}
