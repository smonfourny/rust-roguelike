use crate::constants::{BASE_BG_COLOR, FLOOR_COLOR, FLOOR_COLOR_OOS, MAP_X, MAP_Y, WALL_COLOR, WALL_COLOR_OOS};
use crate::rect::Rect;
use bracket_lib::prelude::*;
use specs::prelude::*;
use std::cmp::{max, min};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<Vec<TileType>>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<Vec<bool>>,
    pub visible_tiles: Vec<Vec<bool>>
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width) as usize + x as usize
    }

    pub fn idx_xy(&self, idx: usize) -> (usize, usize) {
        (idx % self.width as usize, idx / self.width as usize)
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < (MAP_X * MAP_Y) as usize {
                self.tiles[x as usize][y as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < (MAP_X * MAP_Y) as usize {
                self.tiles[x as usize][y as usize] = TileType::Floor;
            }
        }
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        // If room is out of bounds, do not render it
        if room.out_of_bounds(self) {
            return;
        }

        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                self.tiles[x as usize][y as usize] = TileType::Floor;
            }
        }
    }

    pub fn new_map(max_x: i32, max_y: i32) -> Map {
        const MAX_ROOMS: i32 = 40;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut map = Map {
            tiles: vec![vec![TileType::Wall; max_y as usize]; max_x as usize],
            rooms: Vec::new(),
            width: MAP_X,
            height: MAP_Y,
            revealed_tiles: vec![vec![false; max_y as usize]; max_x as usize],
            visible_tiles: vec![vec![false; max_y as usize]; max_x as usize],
        };

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width - w - 1) - 1;
            let y = rng.roll_dice(1, map.height - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);
            if map
                .rooms
                .iter()
                .all(|other_room| !new_room.intersect(&other_room))
                && !new_room.out_of_bounds(&map)
            {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        map
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        let (x, y) = self.idx_xy(idx);
        self.tiles[x][y] == TileType::Wall
    }
}

pub fn draw_map(ecs: &World, ctx: &mut BTerm) {
    let map = ecs.fetch::<Map>();

    for (x, line) in map.tiles.iter().enumerate() {
        for (y, tile) in line.iter().enumerate() {
            if map.revealed_tiles[x][y] {
                let glyph;
                let fg;
                match tile {
                    TileType::Floor => {
                        glyph = to_cp437('.');
                        fg = if map.visible_tiles[x][y] { FLOOR_COLOR } else { FLOOR_COLOR_OOS };
                    }
                    TileType::Wall => {
                        glyph = to_cp437('#');
                        fg = if map.visible_tiles[x][y] { WALL_COLOR } else { WALL_COLOR_OOS };
                    }
                }
                ctx.set(x, y, RGB::named(fg), RGB::named(BASE_BG_COLOR), glyph);
            }
        }
    }
}
