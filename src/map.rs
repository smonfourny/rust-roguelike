use crate::constants::*;
use bracket_lib::prelude::*;
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
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width) as usize + x as usize
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
        let mut map = Map {
            tiles: vec![vec![TileType::Wall; max_y as usize]; max_x as usize],
            rooms: Vec::new(),
            width: MAP_X,
            height: MAP_Y,
        };

        const MAX_ROOMS: i32 = 40;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

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

pub struct Rect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    pub fn out_of_bounds(&self, map: &Map) -> bool {
        let corners = [
            (self.x1, self.y1),
            (self.x1, self.y2),
            (self.x2, self.y1),
            (self.x2, self.y2),
        ];
        corners
            .iter()
            .any(|&corner| corner.0 > map.width as i32 || corner.1 > map.height as i32)
    }
}

pub fn draw_map(map: &[Vec<TileType>], ctx: &mut BTerm) {
    for (x, line) in map.iter().enumerate() {
        for (y, tile) in line.iter().enumerate() {
            match tile {
                TileType::Floor => {
                    ctx.set(
                        x,
                        y,
                        RGB::named(GROUND_COLOR),
                        RGB::named(BASE_BG_COLOR),
                        to_cp437('.'),
                    );
                }
                TileType::Wall => {
                    ctx.set(
                        x,
                        y,
                        RGB::named(WALL_COLOR),
                        RGB::named(BASE_BG_COLOR),
                        to_cp437('#'),
                    );
                }
            }
        }
    }
}
