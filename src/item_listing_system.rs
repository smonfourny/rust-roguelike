use specs::prelude::*;
use super::{GameLog, Name, WantsToDisplayContent};

pub struct ItemListingSystem {}

impl<'a> System<'a> for ItemListingSystem {
    type SystemData = (
        ReadStorage<'a, Name>,
        WriteStorage<'a, WantsToDisplayContent>,
        WriteExpect<'a, GameLog>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (name, mut wants_to_display, mut gamelog) = data;

        for (name, _wants_to_display) in (&name, &wants_to_display).join() {
            gamelog.entries.push(format!("There is a {} here.", name.name));
        }

        wants_to_display.clear();
    }
}