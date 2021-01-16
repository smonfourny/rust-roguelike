use super::{
    CombatStats, GameLog, Player, BASE_BG_COLOR, CYAN_COLOR, HEALTHBAR_OFFSET, HEALTH_OFFSET,
    LOG_OFFSET, MAP_X, MAP_Y, ORANGE_COLOR, RED_COLOR, WHITE_COLOR,
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
            x if x >= 0.25 && x < 0.75 => ORANGE_COLOR,
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
