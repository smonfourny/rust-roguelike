use super::{Map, Player};
use super::{Position, Viewshed};
use bracket_lib::prelude::*;
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player) = data;

        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles =
                    field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
                viewshed
                    .visible_tiles
                    .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);

                // If this is the player, reveal visible tiles
                let p: Option<&Player> = player.get(ent);
                if let Some(_) = p {
                    for line in map.visible_tiles.iter_mut() {
                        for t in line.iter_mut() {
                            *t = false;
                        }
                    }
                    for vis in viewshed.visible_tiles.iter() {
                        map.revealed_tiles[vis.x as usize][vis.y as usize] = true;
                        map.visible_tiles[vis.x as usize][vis.y as usize] = true;
                    }
                }
            }
        }
    }
}
