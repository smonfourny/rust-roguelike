use crate::constants::*;
use bracket_lib::prelude::*;
use std::cmp::{max, min};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Rect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new(x:i32, y:i32, w:i32, h:i32) -> Rect {
        Rect{x1: x, y1: y, x2: x+w, y2: y+h}
    }

    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2)/2, (self.y1 + self.y2)/2)
    }

    pub fn out_of_bounds(&self, map: &[Vec<TileType>]) -> bool {
        let corners = [(self.x1, self.y1), (self.x1, self.y2), (self.x2, self.y1), (self.x2, self.y2)];
        corners.iter().any(|&corner| corner.0 > map.len() as i32 || corner.1 > map[0].len() as i32)
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

pub fn new_map(max_x: i32, max_y: i32) -> (Vec<Rect>, Vec<Vec<TileType>>) {
    let mut map = vec![vec![TileType::Wall; max_y as usize]; max_x as usize];

    let mut rooms : Vec<Rect> = Vec::new();
    const MAX_ROOMS : i32 = 40;
    const MIN_SIZE : i32 = 6;
    const MAX_SIZE : i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, 80 - w - 1) - 1;
        let y = rng.roll_dice(1, 50 - h - 1) - 1;
        let new_room = Rect::new(x, y, w, h);
        if rooms.iter().all(|other_room| !new_room.intersect(&other_room)) && !new_room.out_of_bounds(&map) {
            apply_room_to_map(&new_room, &mut map);

            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len()-1].center();
                if rng.range(0,2) == 1 {
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                    apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                } else {
                    apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                }
            }

            rooms.push(new_room);
        }
    }

    (rooms, map)
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y * MAP_X) as usize + x as usize
}

fn apply_horizontal_tunnel(map: &mut [Vec<TileType>], x1:i32, x2:i32, y:i32) {
    for x in min(x1, x2) ..= max(x1, x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < (MAP_X*MAP_Y) as usize {
            map[x as usize][y as usize] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [Vec<TileType>], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2) ..= max(y1, y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < (MAP_X*MAP_Y) as usize {
            map[x as usize][y as usize] = TileType::Floor;
        }
    }
}

fn apply_room_to_map(room: &Rect, map: &mut [Vec<TileType>]) {
    // If room is out of bounds, do not render it
    if room.out_of_bounds(map) {
        return;
    }

    for y in room.y1+1 ..= room.y2 {
        for x in room.x1+1 ..= room.x2 {
            map[x as usize][y as usize] = TileType::Floor;
        }
    }
}