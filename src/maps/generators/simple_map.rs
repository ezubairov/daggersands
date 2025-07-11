use bracket_geometry::prelude::Rect;

use crate::maps::generators::utils::*;
use crate::maps::generators::MapGenerator;
use crate::maps::map::TileType;
use crate::prelude::*;

pub struct SimpleMapGenerator {
    map: Map,
    starting_position: Position,
    depth: u32,
    rooms: Vec<Rect>,
}

impl SimpleMapGenerator {
    pub fn new(depth: u32) -> SimpleMapGenerator {
        SimpleMapGenerator {
            map: Map::new(depth),
            depth,
            starting_position: Position(IVec2::new(0, 0)),
            rooms: Vec::new(),
        }
    }
    fn rooms_and_corridors(&mut self, mut rng: ResMut<Rng>) {
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        for _i in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, self.map.width - w - 1) - 1;
            let y = rng.roll_dice(1, self.map.height - h - 1) - 1;
            let new_room = Rect::with_size(x, y, w, h);
            let mut ok = true;
            for other_room in self.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }
            if ok {
                apply_room_to_map(&mut self.map, &new_room);

                if !self.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center().to_tuple();
                    let (prev_x, prev_y) = self.rooms[self.rooms.len() - 1].center().to_tuple();
                    if rng.range(0, 2) == 1 {
                        apply_horizontal_tunnel(&mut self.map, prev_x, new_x, prev_y);
                        apply_vertical_tunnel(&mut self.map, prev_y, new_y, new_x);
                    } else {
                        apply_vertical_tunnel(&mut self.map, prev_y, new_y, prev_x);
                        apply_horizontal_tunnel(&mut self.map, prev_x, new_x, new_y);
                    }
                }

                self.rooms.push(new_room);
            }
        }

        let stairs_position = self.rooms[self.rooms.len() - 1].center().to_tuple();
        let stairs_idx = self.map.xy_idx(stairs_position.0, stairs_position.1);
        self.map.tiles[stairs_idx] = TileType::DownStairs;

        let start_pos = self.rooms[0].center().to_tuple();
        self.starting_position = Position(IVec2::new(start_pos.0, start_pos.1))
    }
}

impl MapGenerator for SimpleMapGenerator {
    fn build_map(&mut self, rng: ResMut<Rng>) {
        self.rooms_and_corridors(rng);
        self.map.populate_blocked()
    }

    fn spawn_entities(&mut self, mut commands: Commands) {
        commands.spawn(PlayerBundle::default());

    }

    fn get_map(&self) -> Map {
        self.map.clone()
    }
}
