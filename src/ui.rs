use super::{
    CombatStats, GameLog, InBackpack, Name, Player, State, BASE_BG_COLOR, CYAN_COLOR,
    HEALTHBAR_OFFSET, HEALTH_OFFSET, LOG_OFFSET, MAP_X, MAP_Y, ORANGE_COLOR, RED_COLOR,
    WHITE_COLOR, YELLOW_COLOR,
};
use bracket_lib::prelude::*;
use specs::prelude::*;

pub fn draw_ui(ecs: &World, ctx: &mut BTerm) {
    let log_size = 50 - MAP_Y - 2;
    // Build box starting from bottom of map space (MAP_Y coordinate), spanning
    // full width of viewport (MAP_X)
    ctx.draw_box(
        0,
        MAP_Y,
        MAP_X - 1,
        log_size + 1,
        RGB::named(WHITE_COLOR),
        RGB::named(BASE_BG_COLOR),
    );

    let combat_stats = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<Player>();
    for (_player, stats) in (&players, &combat_stats).join() {
        let health_color = match stats.hp as f32 / stats.max_hp as f32 {
            x if x < 0.25 => RED_COLOR,
            x if (0.25..0.75).contains(&x) => ORANGE_COLOR,
            _ => CYAN_COLOR,
        };

        let hp_message = format!(" HP: {}/{}", stats.hp, stats.max_hp);
        ctx.print_color(
            HEALTH_OFFSET,
            MAP_Y,
            RGB::named(health_color),
            RGB::named(BASE_BG_COLOR),
            &hp_message,
        );

        ctx.draw_bar_horizontal(
            HEALTHBAR_OFFSET,
            MAP_Y,
            HEALTHBAR_OFFSET + 20,
            stats.hp,
            stats.max_hp,
            RGB::named(health_color),
            RGB::named(BASE_BG_COLOR),
        );
    }

    let log = ecs.fetch::<GameLog>();
    for (i, message) in log.entries.iter().rev().take(log_size as usize).enumerate() {
        ctx.print(LOG_OFFSET, MAP_Y + i as i32 + 1, message);
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum ItemMenuResult {
    Cancel,
    NoResponse,
    Selected,
}

pub fn show_inventory(gs: &mut State, ctx: &mut BTerm) -> (ItemMenuResult, Option<Entity>) {
    show_item_menu(gs, ctx, "Inventory")
}

pub fn show_drop_menu(gs: &mut State, ctx: &mut BTerm) -> (ItemMenuResult, Option<Entity>) {
    show_item_menu(gs, ctx, "Choose item to drop:")
}

fn show_item_menu<S: ToString>(
    gs: &mut State,
    ctx: &mut BTerm,
    title: S,
) -> (ItemMenuResult, Option<Entity>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    let names = gs.ecs.read_storage::<Name>();
    let backpack = gs.ecs.read_storage::<InBackpack>();
    let entities = gs.ecs.entities();

    let inventory = (&backpack, &names)
        .join()
        .filter(|item| item.0.owner == *player_entity);
    let count = inventory.count();

    // For now, the list of items should be small.
    // TODO: once the player is able to collect more items, make this pageable
    let y = (25 - (count / 2)) as i32;
    ctx.draw_box(
        15,
        y - 2,
        31,
        (count + 3) as i32,
        RGB::named(WHITE_COLOR),
        RGB::named(BASE_BG_COLOR),
    );
    ctx.print_color(
        17,
        y - 2,
        RGB::named(WHITE_COLOR),
        RGB::named(BASE_BG_COLOR),
        title.to_string(),
    );
    ctx.print_color(
        17,
        y + count as i32 + 1,
        RGB::named(RED_COLOR),
        RGB::named(BASE_BG_COLOR),
        "Esc to close",
    );

    let mut equippable: Vec<Entity> = Vec::new();
    for (j, (entity, _pack, name)) in (&entities, &backpack, &names)
        .join()
        .filter(|item| item.1.owner == *player_entity)
        .enumerate()
    {
        ctx.set(
            17,
            y + j as i32,
            RGB::named(WHITE_COLOR),
            RGB::named(BASE_BG_COLOR),
            to_cp437('('),
        );
        ctx.set(
            18,
            y + j as i32,
            RGB::named(YELLOW_COLOR),
            RGB::named(BASE_BG_COLOR),
            (97 + j as i32) as FontCharType,
        );
        ctx.set(
            19,
            y + j as i32,
            RGB::named(WHITE_COLOR),
            RGB::named(BASE_BG_COLOR),
            to_cp437(')'),
        );

        ctx.print(21, y + j as i32, &name.name.to_string());
        equippable.push(entity);
    }

    match ctx.key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => match key {
            VirtualKeyCode::Escape => (ItemMenuResult::Cancel, None),
            _ => {
                let selection = letter_to_option(key);
                if selection > -1 && selection < count as i32 {
                    return (
                        ItemMenuResult::Selected,
                        Some(equippable[selection as usize]),
                    );
                }

                (ItemMenuResult::NoResponse, None)
            }
        },
    }
}
