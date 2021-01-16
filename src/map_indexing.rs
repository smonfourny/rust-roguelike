use super::{BlocksTile, Map, Position};
use specs::prelude::*;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blockers, entities) = data;

        map.populate_blocked();
        map.clear_content();
        for (entity, pos) in (&entities, &position).join() {
            let p: Option<&BlocksTile> = blockers.get(entity);
            if p.is_some() {
                map.blocked[pos.x as usize][pos.y as usize] = true;
            }
            map.tile_content[pos.x as usize][pos.y as usize].push(entity);
        }
    }
}
