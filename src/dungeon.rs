//Author: Everett Sullivan
//Date Created: 12/12/2019
//Purpose To create for general dungeon layouts
//Notes:

use crate::maze;
use maze::Direction;
use maze::Point;
use maze::Wall;

use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use std::collections::HashSet;
use rand::Rng;

////////////////////
//Custom Error handling code
////////////////////

#[derive(Debug)]
pub enum DungeonError {
    Syntax(String),
}

use std::fmt;
use std::error::Error;

impl fmt::Display for DungeonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DungeonError::Syntax(ref err_string) => write!(f,"{}",err_string),
        }
    }
}

impl Error for DungeonError {
    fn cause(&self) -> Option<&Error> {
        match *self {
            DungeonError::Syntax(ref _err_string) => None,
        }
    }
}

////////////////////
//Main Dungeon code
////////////////////

////////////////////
//Tile
////////////////////

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct Tile {
    pub name: String,
	pub glyph: char,
}

impl Tile {

    pub fn init(name: String, glyph: char) -> Tile {
        Tile{name: name, glyph: glyph}
    }

}

////////////////////
//Room
////////////////////

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct Room {
    pub base_point: Point,
	pub dimensions: Point,
}

impl Room {

    pub fn init(base_point: Point, dimensions: Point) -> Room {
        Room{base_point: base_point, dimensions: dimensions}
    }

    /* pub fn contains_cell(my_room: Room, cell: Point) {
        let in_x_dim = (my_room.base_point.row <= cell.row) && (cell.row <= (my_room.base_point.row + my_room.dimensions.row));
        let in_y_dim = (my_room.base_point.col <= cell.col) && (cell.col <= (my_room.base_point.col + my_room.dimensions.col));
        return in_x_dim && in_y_dim;
    } */

    /* pub fn check_intersection(first_room: Room, second_room: Room) -> bool {
        if first_room.contains_cell(second_room.base_point) {
            return true;
        }
        let mut other_point = Point::init(second_room.base_point.row,second_room.base_point.col + second_room.dimensions.col);
        if first_room.contains_cell(other_point) {
            return true;
        }
        if second_room.contains_cell(first_room.base_point) {
            return true;
        }
        other_point = Point::init(first_room.base_point.row,first_room.base_point.col + first_room.dimensions.col);
        if second_room.contains_cell(other_point) {
            return true;
        }
        return false
    }

    pub fn get_walls(&self) -> Vec<Wall>{
        let mut all_walls = Vec::new();
        for i in 0..self.dimensions.row {
            let north_wall_cell = Point::init(self.base_point.row+i,self.base_point.col);
            let south_wall_cell = Point::init(self.base_point.row+i,self.base_point.col);
            all_walls.push(Wall::init(north_wall_cell,Direction::North));
            all_walls.push(Wall::init(south_wall_cell,Direction::South));
        }
        for j in 0..self.dimensions.col {
            let west_wall_cell = Point::init(self.base_point.row,self.base_point.col+j);
            let east_wall_cell = Point::init(self.base_point.row,self.base_point.col+j);
            all_walls.push(Wall::init(west_wall_cell,Direction::West));
            all_walls.push(Wall::init(east_wall_cell,Direction::East));
        }
        return all_walls;
    } */

}

////////////////////
//Dungeon
////////////////////

#[derive(Debug)]
pub struct Dungeon {
    pub rows: usize,
	pub columns: usize,
	pub map_matrix: Vec<Vec<Tile>>
}

pub fn maze_to_map(my_maze: &maze::Maze) -> Result<Dungeon,DungeonError>{
    let maze_columns = my_maze.columns;
    let maze_rows = my_maze.rows;
    let wall_tile = Tile::init("Wall".to_string(),'#');
    let floor_tile = Tile::init("Floor".to_string(),' ');
    let mut map_matrix = vec![vec![wall_tile; 2*maze_columns + 1]; 2*maze_rows + 1];
    for i in 0..maze_rows {
        for j in 0..maze_columns {
            if my_maze.maze_matrix[i][j].get_number_of_exits() != 0 {
                map_matrix[2*i + 1][2*j + 1] = floor_tile.clone();
            }
            if my_maze.maze_matrix[i][j].has_dir(Direction::North) == true {
                map_matrix[2*i + 2][2*j + 1] = floor_tile.clone();
            }
            if my_maze.maze_matrix[i][j].has_dir(Direction::East) == true {
                map_matrix[2*i + 1][2*j + 2] = floor_tile.clone();
            }
        }
    }
    for i in 0..maze_rows {
        if my_maze.maze_matrix[i][0].has_dir(Direction::West) == true {
            map_matrix[2*i + 1][0] = floor_tile.clone();
        }
    }
    for j in 0..maze_columns {
        if my_maze.maze_matrix[0][j].has_dir(Direction::South) == true {
            map_matrix[0][2*j + 1] = floor_tile.clone();
        }
    }

    Ok(Dungeon{rows: (2*maze_rows + 1), columns: (2*maze_columns + 1), map_matrix: map_matrix})
}

pub fn create_dungeon() -> Result<Dungeon,DungeonError>{

    let num_of_small_rooms = 2;
    let num_of_medium_rooms = 4;
    let num_of_large_rooms = 2;

    let size_of_small_room_min = 2;//second odd number
    let size_of_small_room_min = 3;//thrid odd number
    let num_of_small_room_doors_min = 1;
    let num_of_small_room_doors_max = 2;

    let size_of_medium_room_min = 3;//thrid odd number
    let size_of_medium_room_min = 5;//fifth odd number
    let num_of_medium_room_doors_min = 1;
    let num_of_medium_room_doors_max = 4;

    let size_of_large_room_min = 3;//thrid odd number
    let size_of_large_room_min = 5;//fifth odd number
    let num_of_large_room_doors_min = 1;
    let num_of_large_room_doors_max = 6;

    //let mut rooms = Vec::new();
    for i in 0..num_of_large_rooms {

    }

    Err(DungeonError::Syntax("Not yet implemented.".to_string()))

}

pub fn print_dungeon_as_image(my_dungeon: &Dungeon, output_file_name: String, block_size: usize){
    let block_size_u32 = block_size as u32;
    let mut imgbuf = image::ImageBuffer::new((block_size*my_dungeon.rows) as u32, (block_size*my_dungeon.columns) as u32);
    let floor_tile = Tile::init("Floor".to_string(),' ');
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        if my_dungeon.map_matrix[(x/block_size_u32) as usize][(y/block_size_u32) as usize] != floor_tile {
            *pixel = image::Rgb([0, 0, 0]);
        }else {
            *pixel = image::Rgb([255, 255, 255]);
        }
    }

    imgbuf.save(output_file_name).unwrap();
}

pub fn print_dugenon_to_file(my_dungeon: &Dungeon, output_file_name: String){
    let file = File::create(output_file_name).expect("Unable to create file");
    let mut f = BufWriter::new(file);
    print_dungeon(&my_dungeon,&mut f);
    f.flush().unwrap();
}

pub fn print_dungeon(my_dungeon: &Dungeon, out_stream: &mut io::Write){
    for i in 0..my_dungeon.rows {
        for j in 0..my_dungeon.columns {
            write!(out_stream, "{}",my_dungeon.map_matrix[i][j].glyph);
        }
        write!(out_stream,"\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_starting_points_test() {
        //let single_cell_room = Room::init(Point::init(0,0),Point::init(0,0));
        //assert!(single_cell_room.contains_cell(Point::init(0,0)));
        //assert!(!single_cell_room.contains_cell(Point::init(1,0)));
        //assert!(!single_cell_room.contains_cell(Point::init(0,1)));

        //let test_room = Room::init(Point::init(2,3),Point::init(1,2));
    }

}