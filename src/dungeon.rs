//Author: Everett Sullivan
//Date Created: 12/12/2019
//Purpose To create for general dungeon layouts
//Notes:

use crate::maze;
use maze::Direction;
use maze::Point;
use maze::Wall;
use maze::Maze;
use maze::GenerationType;

use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use std::collections::HashSet;
use rand::{Rng, StdRng};

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
pub struct Tile{
    pub name: &'static str,
	pub glyph: char,
}

impl Tile {

    pub fn init(name: &'static str, glyph: char) -> Tile {
        Tile{name: name, glyph: glyph}
    }

}

pub const WALL_TILE: Tile = Tile{name: "Wall", glyph: '#'};
pub const FLOOR_TILE: Tile = Tile{name: "Floor", glyph: ' '};
pub const ROOM_TILE: Tile = Tile{name: "Room", glyph: ' '};
pub const EXIT_TILE: Tile = Tile{name: "Exit", glyph: ' '};

////////////////////
//Room
////////////////////

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub struct Room {
    pub base_point: Point, //the base point occers at the bottom left of the room
	pub dimensions: Point, //The dimension of the room. Note that this repsent the displacement needed to get the top right corner in the room.
}

impl Room {

    pub fn init(base_point: Point, dimensions: Point) -> Room {
        Room{base_point: base_point, dimensions: dimensions}
    }

    //get_base_point
    //Purpose:
    //    Returns the ower left square in the room.
    pub fn get_base_point(self) -> Point {
        return self.base_point;
    }

    //get_width
    //Purpose:
    //    Returns the width of the room
    pub fn get_width(self) -> usize {
        return self.dimensions.get_x();
    }

    //get_height
    //Purpose:
    //    Returns the width of the room
    pub fn get_height(self) -> usize {
        return self.dimensions.get_y();
    }

    //contains_cell
    //Purpose:
    //    Returns true if the given cell is inside the room.
    pub fn contains_cell(my_room: Room, cell: Point) -> bool {
        let in_x_dim = (my_room.base_point.row <= cell.row) && (cell.row <= (my_room.base_point.row + my_room.dimensions.row));
        let in_y_dim = (my_room.base_point.col <= cell.col) && (cell.col <= (my_room.base_point.col + my_room.dimensions.col));
        return in_x_dim && in_y_dim;
    }

    //check_multi_room_contains_cell
    //Purpose:
    //    Returns true if my_cell is contained in any of the rooms in other_rooms.
    pub fn check_multi_room_contains_cell(my_cell: Point, other_rooms: &Vec<Room>) -> bool {

        for other_room in other_rooms.iter(){
            if Room::contains_cell(*other_room,my_cell){
                return true;
            }
        }
        return false;
    }

    //check_intersection
    //Purpose:
    //    Returns true if the two rooms overlap.
    pub fn check_intersection(first_room: Room, second_room: Room) -> bool {
        //First case: A room has a conner inside the other room.
        if Room::contains_cell(first_room,second_room.base_point) {
            return true;
        }
        let second_room_top_left = Point::init(second_room.base_point.row,second_room.base_point.col + second_room.dimensions.col);
        if Room::contains_cell(first_room,second_room_top_left) {
            return true;
        }
        let second_room_bottom_right = Point::init(second_room.base_point.row + second_room.dimensions.row ,second_room.base_point.col);
        if Room::contains_cell(first_room,second_room_bottom_right) {
            return true;
        }
        if Room::contains_cell(second_room,first_room.base_point) {
            return true;
        }
        let first_room_top_left = Point::init(first_room.base_point.row,first_room.base_point.col + first_room.dimensions.col);
        if Room::contains_cell(second_room,first_room_top_left) {
            return true;
        }
        let first_room_bottom_right = Point::init(first_room.base_point.row + first_room.dimensions.row ,first_room.base_point.col);
        if Room::contains_cell(second_room,first_room_bottom_right) {
            return true;
        }
        //Second case: the two rooms form a cross,
        if (second_room.base_point.row < first_room.base_point.row) && (second_room.base_point.row + second_room.dimensions.row > first_room.base_point.row){
            if (first_room.base_point.col < second_room.base_point.col) && (first_room.base_point.col + first_room.dimensions.col > second_room.base_point.col){
                return true;
            }
        } else if (first_room.base_point.row < second_room.base_point.row) && (first_room.base_point.row + first_room.dimensions.row > second_room.base_point.row) {
            if (second_room.base_point.col < first_room.base_point.col) && (second_room.base_point.col + second_room.dimensions.col > first_room.base_point.col) {
                return true;
            }
        }

        return false;
    }

    //check_multi_room_intersection
    //Purpose:
    //    Returns true if my_room intersects any of the rooms in other_rooms.
    pub fn check_multi_room_intersection(my_room: Room, other_rooms: &Vec<Room>) -> bool {

        for other_room in other_rooms.iter(){
            if Room::check_intersection(my_room,*other_room){
                return true;
            }
        }
        return false;
    }

    //generate_room_with_dimensions_in_bounds
    //Purpose:
    //    Returns a room with that is contained within the bounds (Given by a room) and
    //    has the given dimensions.
    //    If a room can not be generated within the given number of attempts, returns None.
    //Notes:
    //    bounds: A room that gives the bounds the generated room should be contained in.
    //    room_dim: dimensions of the generated room
    //    attempts: the number of times to try to generate a room
    //    avoid: a vector of rooms that the generated room should not intersect.
    pub fn generate_room_with_dimensions_in_bounds(bounds: Room, room_dim: Point, attempts: usize, avoid: &Vec<Room>) -> Option<Room> {
        //if the room is bigger than the bounds, no room can be generated.
        if (bounds.dimensions.row < room_dim.row) || (bounds.dimensions.col < room_dim.col) {
            return None;
        }else {
            let max_x = bounds.dimensions.row - room_dim.row;
            let max_y = bounds.dimensions.col - room_dim.col;
            for _my_attempt in 0..(attempts+1) {
                let base_point_x = rand::thread_rng().gen_range(0,max_x+1);
                let base_point_y = rand::thread_rng().gen_range(0,max_y+1);
                let base_point = Point::init(base_point_x + bounds.base_point.row, base_point_y + bounds.base_point.col);
                let created_room = Room::init(base_point,room_dim);
                if !Room::check_multi_room_intersection(created_room,avoid) {
                    return Some(created_room);
                }
            }
        }
        return None;
    }

    //get_walls
    //Purpose:
    //    Returns a Vec of all Walls of the room, given by cells inside the room.
    pub fn get_walls(&self) -> Vec<Wall>{
        let mut all_walls = Vec::new();
        for i in 0..(self.dimensions.col+1) {
            let south_wall_cell = Point::init(self.base_point.row, self.base_point.col + i);
            let north_wall_cell = Point::init(self.base_point.row + self.dimensions.row, self.base_point.col + i);
            all_walls.push(Wall::init(north_wall_cell,Direction::North));
            all_walls.push(Wall::init(south_wall_cell,Direction::South));
        }
        for j in 0..(self.dimensions.row+1) {
            let west_wall_cell = Point::init(self.base_point.row + j, self.base_point.col);
            let east_wall_cell = Point::init(self.base_point.row + j, self.base_point.col + self.dimensions.col);
            all_walls.push(Wall::init(west_wall_cell,Direction::West));
            all_walls.push(Wall::init(east_wall_cell,Direction::East));
        }
        return all_walls;
    }

}

////////////////////
//Dungeon
////////////////////

//Dungeon
//Purpose:
//    An array of Tiles to represent the a map.
#[derive(Debug)]
pub struct Dungeon {
    pub rows: usize,
	pub columns: usize,
	pub map_matrix: Vec<Vec<Tile>>
}

//maze_to_map
//Purpose:
//    Given a maze object which state the directions, creates a dungeon which is an array of tiles.
//Notes:
//    Since the cells in a map are connected by direction,
//      the actual map must be expaned to allows for cells in those directions.
pub fn maze_to_map(my_maze: &maze::Maze) -> Result<Dungeon,DungeonError>{
    let maze_columns = my_maze.columns;
    let maze_rows = my_maze.rows;
    let mut map_matrix = vec![vec![WALL_TILE; 2*maze_columns + 1]; 2*maze_rows + 1];
    for i in 0..maze_rows {
        for j in 0..maze_columns {
            if my_maze.maze_matrix[i][j].get_number_of_exits() != 0 {
                map_matrix[2*i + 1][2*j + 1] = FLOOR_TILE.clone();
            }
            if my_maze.maze_matrix[i][j].has_dir(Direction::North) == true {
                map_matrix[2*i + 2][2*j + 1] = FLOOR_TILE.clone();
            }
            if my_maze.maze_matrix[i][j].has_dir(Direction::East) == true {
                map_matrix[2*i + 1][2*j + 2] = FLOOR_TILE.clone();
            }
        }
    }
    for i in 0..maze_rows {
        if my_maze.maze_matrix[i][0].has_dir(Direction::West) == true {
            map_matrix[2*i + 1][0] = FLOOR_TILE.clone();
        }
    }
    for j in 0..maze_columns {
        if my_maze.maze_matrix[0][j].has_dir(Direction::South) == true {
            map_matrix[0][2*j + 1] = FLOOR_TILE.clone();
        }
    }

    Ok(Dungeon{rows: (2*maze_rows + 1), columns: (2*maze_columns + 1), map_matrix: map_matrix})
}

//connect_dugeon
//Purpose:
//    Given a maze object add cells and direction to make the dugeon contiguous (evey cell can be reached by every other cell)
//Notes:
//    To make sure the dungeon is contiguous, a flood-fill type alogirthm is used.
//    Starte with cell (0,0) which always exists and find all of the directions out of the cell.
//    Now moving in these directions will give new cells, mark these cells as visited and add the directions out of those cells.
//    Any time a cell is visiited check if it has already been visited, if not add the directions out of the new cell.
//    Once all possible direction reachable from (0,0) have been done, check if there are cells still to be visited.
//    If so, find one that is adjacent to a already visited cell and create a passage between the two and add all directions out of this new cell.
//    Contiue this until every cell has been visited.
//Post-Conditions:
//    The maze object will be modifed.
pub fn connect_dugeon(my_maze: &mut maze::Maze, wrap: usize){
    let mut check_matrix = vec![vec![false; my_maze.columns]; my_maze.rows]; //indicates which cells we have already visited.
    let mut cell_moves = Vec::new(); //moves that can be taken by cells we have reached.
    let mut current_cell = Point::init(0,0);
    let mut current_dir = Direction::North;

    //start with cell (0,0) which will always exist.
    //push all valid directions out of that cell.
    let start_exits = my_maze.maze_matrix[0][0].get_exits();
    for exit in start_exits {
        cell_moves.push(Wall::init(current_cell,exit));
    }
    check_matrix[current_cell.row][current_cell.col] = true;


    //while every cell is not yet reachable
    let mut is_contiguous = false;
    while !is_contiguous {
        while cell_moves.len() != 0 {// there are still cells to check
            let move_to = cell_moves.pop().unwrap();
            current_cell = move_to.cell;
            //get the cell that the move goes to.
            let next_cell_check = maze::get_cell_in_direction(my_maze.rows, my_maze.columns, current_cell.row, current_cell.col, move_to.dir, wrap);
            match next_cell_check { //if we can actualy move in this direction (could be stopped by the boundary of the map if no wrapping)
                Some(next_cell) => {
                    if !check_matrix[next_cell.row][next_cell.col] { //If this next cell is not already connected, add it and push directions.
                        check_matrix[next_cell.row][next_cell.col] = true;
                        for exit in my_maze.maze_matrix[next_cell.row][next_cell.col].get_exits() {
                            cell_moves.push(Wall::init(next_cell,exit));
                        }
                    }
                },
                None => (),
            }
        }

        //check if the dungeon is contiguous
        let mut new_break = false;
        let mut all_filled = true;
        for i in 0..my_maze.rows {
            for j in 0..my_maze.columns {
                //Once we found a cell that has not been visited we don't need to look for more.
                if !new_break && (check_matrix[i][j] == false) {
                    all_filled = false; //An unvisited cell was found
                    new_break = true; //We found a unvisited cell, no need to keep looking.
                    check_matrix[i][j] = true; //It will now be visited.

                    let mut added_dir = false; //We need to add an exit from this cell to one that has been visited, but only one.
                    for direction in Direction::get_all_square_directions() { //check all ways out of this cell.
                        if !added_dir {
                            match maze::get_cell_in_direction(my_maze.rows, my_maze.columns, i, j, direction, wrap) {
                                Some(break_out_cell) => {
                                    if check_matrix[break_out_cell.row][break_out_cell.col] == true {
                                        added_dir = true;
                                        for exit in my_maze.maze_matrix[i][j].get_exits() {
                                            cell_moves.push(Wall::init(Point::init(i,j),exit));
                                        }
                                        my_maze.remove_wall(Wall::init(break_out_cell,direction.reverse()),wrap);
                                    }
                                },
                                None => (),
                            }
                        }
                    }
                }
            }
        }
        is_contiguous = all_filled;
    }
}

//create_dungeon
//Purpose:
//    Creates a dungeon with the given perameters.
//Notes:
//    my_rows: the number of rows in the underlying MAZE, the actual dungeon will have 2*my_rows+1
//    my_columns: the number of columns in the underlying MAZE, the actual dungeon will have 2*my_columns+1
//    wrap: Allows the maze to wrap around the edges. If any wrapping is allowed then outside_exits is ignored.
//    method: The method used to create the underlying maze.
//    num_room: the maximum number of rooms the dungeon can have.
//      Each room in placed randomly, it the random placement overlappes with another room it tries again.
//      If after ROOM_PLACEMENT_ATTEMPTS a valid place has not been found then the room is discarded.
//    prune_dead_ends_ratio: The ratio of dead_ends to remove, 0.0 means nothing is removed.
//      1.0 means that dungeon will have no dead ends.
//    outside_exits: If true allow cells to exit the edge of the map, if any wrapping is allowed this value is ignored.
//
//    To make the dungeon the rooms are generated first.
//    A maze is then made around the rooms.
//    Exits from rooms are then made into the maze, exits default to going into the maze (as opposed to another room) is possible.
//    The dungeon is then made to be contiguous (odd room and exit placement can cause the dungeon to be disconnected.)
//    Dead ends are removed AFTER the maze is made contiguous.
//Pre-Conditions:
//    prune_dead_ends_ratio must be between  0.0 and 1.0 inclusive. (verified)
pub fn create_dungeon(my_rows: usize, my_columns: usize, wrap: usize, method: GenerationType, num_rooms: usize, prune_dead_ends_ratio: f64, outside_exits: bool) -> Result<Dungeon,DungeonError>{

    const ROOM_MIN_WIDTH: usize = 2;
    const ROOM_MIN_HEIGHT: usize = 2;
    const ROOM_MAX_WIDTH: usize = 5;
    const ROOM_MAX_HEIGHT: usize = 5;

    const ROOM_PLACEMENT_ATTEMPTS: usize = 10;

    const ROOM_MIN_EXITS: usize = 1;
    const ROOM_MAX_EXITS: usize = 3;

    const OUTSIDE_MIN_EXITS: usize = 1;
    const OUTSIDE_MAX_EXITS: usize = 4;

    //verify that prune_dead_ends_ratio is betweenn 0.0 and 1.0, if not move it to closest valid value.
    let prune_dead_ends_ratio_cleaned;
    if prune_dead_ends_ratio < 0.0 {
        prune_dead_ends_ratio_cleaned = 0.0;
    }else if prune_dead_ends_ratio > 1.0 {
        prune_dead_ends_ratio_cleaned = 1.0
    }else{
        prune_dead_ends_ratio_cleaned = prune_dead_ends_ratio;
    }


    //create rooms for dungeon.
    let mut my_rooms = Vec::new();
    let dungeon_bounds = Room::init(Point::init(0,0),Point::init(my_rows-1,my_columns-1));
    for _i in 0..num_rooms {
        let new_room_dim = Point::init(rand::thread_rng().gen_range(ROOM_MIN_WIDTH-1,ROOM_MAX_WIDTH),rand::thread_rng().gen_range(ROOM_MIN_HEIGHT-1,ROOM_MAX_HEIGHT));
        match Room::generate_room_with_dimensions_in_bounds(dungeon_bounds,new_room_dim,ROOM_PLACEMENT_ATTEMPTS ,&my_rooms) {
            Some(new_room) => {
                my_rooms.push(new_room);
            },
            None => (),
        }
    }


    //the passages will need to be crated around the rooms, so room cells are forbidden from being used in the maze.
    let mut room_bit_mask = vec![vec![true; my_columns]; my_rows];
    for room in my_rooms.iter() {
        let room_base_x = room.get_base_point().get_x();
        let room_base_y = room.get_base_point().get_y();
        for i in 0..(room.get_width()+1){
            for j in 0..(room.get_height()+1){
                room_bit_mask[room_base_x + i][room_base_y + j] = false;
            }
        }
    }

    //crate the maze.
    let mut my_maze = Maze::init_rect_with_bitmask(my_rows,my_columns,0,&room_bit_mask,method).unwrap();

    //Connect rooms interiors (useful for determine connectivity of dugeon.)
    let mut room_bit_mask = vec![vec![true; my_columns]; my_rows];
    for room in my_rooms.iter() {
        let room_base_x = room.get_base_point().get_x();
        let room_base_y = room.get_base_point().get_y();
        for i in 0..(room.get_width()+1){
            for j in 0..(room.get_height()+1){
                if i != 0{
                    my_maze.maze_matrix[room_base_x + i][room_base_y + j].add_dir(Direction::South);
                }
                if j != 0 {
                    my_maze.maze_matrix[room_base_x + i][room_base_y + j].add_dir(Direction::West);
                }
                if i != room.get_width() {
                    my_maze.maze_matrix[room_base_x + i][room_base_y + j].add_dir(Direction::North);
                }
                if j != room.get_height() {
                    my_maze.maze_matrix[room_base_x + i][room_base_y + j].add_dir(Direction::East);
                }
            }
        }
    }

    //have rooms exit into maze.
    for room in my_rooms.iter() {

        let num_exits = rand::thread_rng().gen_range(ROOM_MIN_EXITS,ROOM_MAX_EXITS+1);

        //determine which exits go into the maze vs other rooms, vs off the map.
        let my_walls = room.get_walls();
        let mut exits_to_outside = Vec::new();
        let mut exits_to_another_room = Vec::new();
        let mut exits_to_maze = Vec::new();
        for wall in my_walls.clone() {
            let current_cell = wall.cell;
            let current_dir = wall.dir;
            match maze::get_cell_in_direction(my_rows,my_columns,current_cell.row,current_cell.col,current_dir,wrap) {
                Some(cell) => {
                    if Room::check_multi_room_contains_cell(cell,&my_rooms) {
                        exits_to_another_room.push(wall);
                    }else{
                        exits_to_maze.push(wall);
                    }
                },
                None => exits_to_outside.push(wall),
            }
        }

        //have exits default to exiting into the maze otherwise exit into antother room.
        //If no such exits exit, don't create an exit.
        for _i in 0..num_exits{
            if exits_to_maze.len() != 0 {
                let choice = rand::thread_rng().gen_range(0, exits_to_maze.len());
                let current_wall = exits_to_maze[choice];
                my_maze.remove_wall(current_wall, wrap);
                exits_to_maze.swap_remove(choice); //remove the choice so that a new exit will be chosen.
            }else if exits_to_another_room.len() != 0{
                let choice = rand::thread_rng().gen_range(0, exits_to_another_room.len());
                let current_wall = exits_to_another_room[choice];
                my_maze.remove_wall(current_wall, wrap);
                exits_to_another_room.swap_remove(choice); //remove the choice so that a new exit will be chosen.
            }
        }
        
    }

    //make the maze contiguous
    connect_dugeon(&mut my_maze, wrap);

    //make exits off the map
    if outside_exits && (wrap == maze::NO_SQUARE_WRAP) { //to have outside exits, they must be enabled and no wrapping.
        let outside_exits = dungeon_bounds.get_walls();
        let num_outside_exits = rand::thread_rng().gen_range(OUTSIDE_MIN_EXITS, OUTSIDE_MAX_EXITS+1);
        for _i in 0..num_outside_exits{
            let choice = rand::thread_rng().gen_range(0, outside_exits.len());
            let current_wall = outside_exits[choice];
            my_maze.remove_wall(current_wall,wrap);
        }
    }

    //erase dead ends.
    if prune_dead_ends_ratio_cleaned != 0.0 { // if we don't erase any dead ends there is no point in even finding them.

        //get dead ends
        let my_dead_ends = my_maze.get_dead_ends();
        let mut actual_dead_ends = Vec::new();
        for dead_end in my_dead_ends.iter() {
            let mut in_room = false;
            for room in my_rooms.iter() {
                if Room::contains_cell(*room,*dead_end){
                    in_room = true;
                }
            }
            if !in_room{
                actual_dead_ends.push(dead_end);
            }
        }
    
        //erase dead ends.
        let mut rng = StdRng::new().unwrap();
        rng.shuffle(&mut actual_dead_ends);
        let stop = ((actual_dead_ends.len() as f64)*prune_dead_ends_ratio_cleaned ) as usize;
        for i in 0..stop {
            //it could be that two dead-ends lead to each other and so erasing one erases the other.
            if my_maze.maze_matrix[actual_dead_ends[i].row][actual_dead_ends[i].col].get_number_of_exits() != 0 {
                my_maze.erase_dead_end(*actual_dead_ends[i]);
            }
        }
    }

    //turn maze into a dungeon. (This should not return a error)
    let mut my_dungeon = maze_to_map(&my_maze).unwrap();

    //place cells in dungeon for rooms.
    for room in my_rooms.iter() {
        let room_base_x = 2*room.get_base_point().get_x()+1;
        let room_base_y = 2*room.get_base_point().get_y()+1;
        for i in 0..(2*room.get_width()+1){
            for j in 0..(2*room.get_height()+1){
                my_dungeon.map_matrix[room_base_x+i][room_base_y+j] = ROOM_TILE.clone();
            }
        }
    }

    return Ok(my_dungeon);

}

//print_dungeon_as_image
//Purpose:
//    Creates a image file displaying the dugeon.
pub fn print_dungeon_as_image(my_dungeon: &Dungeon, output_file_name: String, block_size: usize){
    let block_size_u32 = block_size as u32;
    let mut imgbuf = image::ImageBuffer::new((block_size*my_dungeon.rows) as u32, (block_size*my_dungeon.columns) as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        if my_dungeon.map_matrix[(x/block_size_u32) as usize][(y/block_size_u32) as usize] == FLOOR_TILE {
            *pixel = image::Rgb([255, 255, 255]);
        }else if my_dungeon.map_matrix[(x/block_size_u32) as usize][(y/block_size_u32) as usize] == WALL_TILE {
            *pixel = image::Rgb([0, 0, 0]);
        }else if my_dungeon.map_matrix[(x/block_size_u32) as usize][(y/block_size_u32) as usize] == EXIT_TILE {
            *pixel = image::Rgb([0, 255, 0]);
        }else{
            *pixel = image::Rgb([255, 0, 0]);
        }
    }

    if output_file_name.contains(".jpeg") || output_file_name.contains(".png") {
        imgbuf.save(output_file_name).unwrap();
    } else {
      imgbuf.save(output_file_name + ".png").unwrap();
    }
}

//print_dugenon_to_file
//Purpose:
//    Creates a textfile displaying the dugeon as a ASCII map.
pub fn print_dugenon_to_file(my_dungeon: &Dungeon, output_file_name: String){
    let file = File::create(output_file_name).expect("Unable to create file");
    let mut f = BufWriter::new(file);
    print_dungeon(&my_dungeon,&mut f);
    f.flush().unwrap();
}

//print_dugenon_to_file
//Purpose:
//    Displays the dugeon as a ASCII map on the terminal.
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
    fn room_contain_cell_test() {
        let single_cell_room = Room::init(Point::init(0,0),Point::init(0,0));
        assert!(Room::contains_cell(single_cell_room,Point::init(0,0)));
        assert!(!Room::contains_cell(single_cell_room,Point::init(1,0)));
        assert!(!Room::contains_cell(single_cell_room,Point::init(0,1)));

        let test_room = Room::init(Point::init(2,3),Point::init(1,2));
        for i in 2..((2+1) + 1) {
            for j in 3..((3+ 2) + 1) {
                assert!(Room::contains_cell(test_room,Point::init(i,j)));
            }
        }
        assert!(!Room::contains_cell(test_room,Point::init(4,3)));
        assert!(!Room::contains_cell(test_room,Point::init(2,6)));
    }

    #[test]
    fn room_intersection_test() {
        let single_cell_room = Room::init(Point::init(0,0),Point::init(0,0));
        let room_1 = Room::init(Point::init(0,0),Point::init(2,2));
        let room_2 = Room::init(Point::init(3,0),Point::init(2,2));
        let room_3 = Room::init(Point::init(0,3),Point::init(2,2));
        let room_4 = Room::init(Point::init(3,3),Point::init(2,2));
        let room_5 = Room::init(Point::init(2,2),Point::init(1,1));
        let room_6 = Room::init(Point::init(1,1),Point::init(3,3));

        let room_7 = Room::init(Point::init(1,1),Point::init(0,2));
        let room_8 = Room::init(Point::init(1,3),Point::init(2,0));

        //A room should intersect itself.
        assert!(Room::check_intersection(single_cell_room,single_cell_room));

        //Intersections should be symmetric.
        //These rooms should not intersect.
        assert!(!Room::check_intersection(room_1,room_2));
        assert!(!Room::check_intersection(room_2,room_1));

        assert!(!Room::check_intersection(room_1,room_3));
        assert!(!Room::check_intersection(room_3,room_1));

        assert!(!Room::check_intersection(room_1,room_4));
        assert!(!Room::check_intersection(room_4,room_1));

        assert!(!Room::check_intersection(room_2,room_3));
        assert!(!Room::check_intersection(room_3,room_2));

        assert!(!Room::check_intersection(room_2,room_4));
        assert!(!Room::check_intersection(room_4,room_2));

        assert!(!Room::check_intersection(room_4,room_3));
        assert!(!Room::check_intersection(room_3,room_4));

        //These rooms should intersect.
        assert!(Room::check_intersection(room_1,room_5));
        assert!(Room::check_intersection(room_5,room_1));

        assert!(Room::check_intersection(room_2,room_5));
        assert!(Room::check_intersection(room_5,room_2));

        assert!(Room::check_intersection(room_3,room_5));
        assert!(Room::check_intersection(room_5,room_3));

        assert!(Room::check_intersection(room_4,room_5));
        assert!(Room::check_intersection(room_5,room_4));

        assert!(Room::check_intersection(room_1,room_6));
        assert!(Room::check_intersection(room_6,room_1));

        assert!(Room::check_intersection(room_2,room_6));
        assert!(Room::check_intersection(room_6,room_2));

        assert!(Room::check_intersection(room_3,room_6));
        assert!(Room::check_intersection(room_6,room_3));

        assert!(Room::check_intersection(room_4,room_6));
        assert!(Room::check_intersection(room_6,room_4));

        assert!(Room::check_intersection(room_6,room_5));
        assert!(Room::check_intersection(room_5,room_6));

        assert!(Room::check_intersection(room_7,room_8));
        assert!(Room::check_intersection(room_8,room_7));

        let room_list_1 = vec![room_1,room_2,room_3];
        let room_list_2 = vec![room_1,room_2];

        assert!(Room::check_multi_room_intersection(room_1,&room_list_1));
        assert!(Room::check_multi_room_intersection(room_1,&room_list_2));

        assert!(Room::check_multi_room_intersection(room_3,&room_list_1));
        assert!(!Room::check_multi_room_intersection(room_3,&room_list_2));

        assert!(!Room::check_multi_room_intersection(room_4,&room_list_1));
        assert!(!Room::check_multi_room_intersection(room_4,&room_list_2));
    }

    #[test]
    fn create_room_test() {
        let dimensions_1 = Point::init(0,0);
        let dimensions_2 = Point::init(5,5);
        let dimensions_3 = Point::init(7,7);
        let test_bound_1 = Room::init(Point::init(0,0),dimensions_1);
        let test_bound_2 = Room::init(Point::init(0,0),dimensions_2);
        let test_bound_3 = Room::init(Point::init(6,6),dimensions_2);

        //testing that oversized room can't be generated
        assert!(!Room::generate_room_with_dimensions_in_bounds(test_bound_1,dimensions_1,1,&vec![]).is_none());
        assert!(Room::generate_room_with_dimensions_in_bounds(test_bound_1,dimensions_2,1,&vec![]).is_none());
        assert!(Room::generate_room_with_dimensions_in_bounds(test_bound_1,dimensions_3,1,&vec![]).is_none());

        assert!(!Room::generate_room_with_dimensions_in_bounds(test_bound_2,dimensions_1,1,&vec![]).is_none());
        assert!(!Room::generate_room_with_dimensions_in_bounds(test_bound_2,dimensions_2,1,&vec![]).is_none());
        assert!(Room::generate_room_with_dimensions_in_bounds(test_bound_2,dimensions_3,1,&vec![]).is_none());

        assert!(!Room::generate_room_with_dimensions_in_bounds(test_bound_3,dimensions_1,1,&vec![]).is_none());
        assert!(!Room::generate_room_with_dimensions_in_bounds(test_bound_3,dimensions_2,1,&vec![]).is_none());
        assert!(Room::generate_room_with_dimensions_in_bounds(test_bound_3,dimensions_3,1,&vec![]).is_none());

        //testing intersections
        assert!(Room::generate_room_with_dimensions_in_bounds(test_bound_1,dimensions_1,1,&vec![test_bound_1]).is_none());
        assert!(!Room::generate_room_with_dimensions_in_bounds(test_bound_3,dimensions_1,1,&vec![test_bound_2]).is_none());
    }

}