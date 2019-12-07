//Author: Everett Sullivan
//Date Created: 11/27/2019
//Purpose To create mazes
//Notes:

use std::collections::HashSet;
use rand::Rng;

////////////////////
//Custom Error handling code
////////////////////

#[derive(Debug)]
pub enum MazeError {
    Syntax(String),
}

use std::fmt;
use std::error::Error;

impl fmt::Display for MazeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MazeError::Syntax(ref err_string) => write!(f,"{}",err_string),
        }
    }
}

impl Error for MazeError {
    fn cause(&self) -> Option<&Error> {
        match *self {
            MazeError::Syntax(ref _err_string) => None,
        }
    }
}

////////////////////
//Maze code
////////////////////

//Direction
//Purpose:
//    To denote which maze generation algorithm to use.
#[derive(Debug,Clone,Copy,PartialEq)]
pub enum GenerationType {
    Prim,
    Wilson,
    Backtrack(f64),
}

//Direction
//Purpose:
//    To be able to record direction is a 2D square maze
#[derive(Debug,Clone,Copy,PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {

    //reverse
    //Purpose:
    //    Returns the direction opposite of the current one.
    pub fn reverse(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    pub fn turn_clockwise(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }

    pub fn turn_counterclockwise(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }
}

pub fn get_all_directions() -> Vec<Direction> {
    let mut my_directions = Vec::new();
    my_directions.push(Direction::North);
    my_directions.push(Direction::South);
    my_directions.push(Direction::East);
    my_directions.push(Direction::West);
    return my_directions;
}

//Compass
//Purpose:
//    To keep track of what direction one can move in a 2d square maze.
#[derive(Debug,Clone,Copy)]
pub struct Compass {
    //Stores the information of in which directions one can move from the given square.
    north: bool,
    south: bool,
    east: bool,
    west: bool,
}

impl Compass {

    //Since we are building a maze, we will start with no existing passages.
    pub fn init() -> Compass{
        Compass{north: false,south: false,east: false,west: false}
    }

    pub fn add_dir(self,dir: Direction) -> Self {
        match dir {
            Direction::North => Compass{north: true, ..self},
            Direction::South => Compass{south: true, ..self},
            Direction::East => Compass{east: true, ..self},
            Direction::West => Compass{west: true, ..self},
        }
    }

    pub fn has_dir(self,dir: Direction) -> bool {
        match dir {
            Direction::North => self.north,
            Direction::South => self.south,
            Direction::East => self.east,
            Direction::West => self.west,
        }
    }

}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub struct Point {
    row: usize,
    col: usize,
}

#[derive(Debug)]
pub struct Maze {
    pub rows: usize,
	pub columns: usize,
	pub maze_matrix: Vec<Vec<Compass>>
}

impl Maze {

    //init_rect
    //Purpose:
    //    Creates a rectangular maze using either Prim's, Wilson's, or a Backtrack Algorithm.
    //Pre-Conditions:
    //    The variables max_rows and max_cols are non-zero.
    //Notes:
    //  Wrap indicates if the rectangle should be considered as having its sides meet up.
    //    A wrap of 0 means no wrapping, 1 means vertical wrapping, and 2 means wrapping on both pairs of sides.
    //  The backtrack method requires a parameter which must be between 0.0 and 1.0, and affect the probablility
    pub fn init_rect(my_rows: usize, my_columns: usize, wrap: usize, method: GenerationType) -> Result<Maze,MazeError> {

		if (my_rows == 0) || (my_columns == 0) {
			return Err(MazeError::Syntax("A maze requires non-zero dimensions.".to_string()));
        }
        
        let matrix;
        match method {
            GenerationType::Prim => matrix = prims_algorithm(my_rows, my_columns, wrap),
            GenerationType::Wilson => matrix = wilsons_algorithm(my_rows, my_columns, wrap),
            GenerationType::Backtrack(straightness) => matrix = bias_recursive_backtrack_algorithm(my_rows, my_columns, wrap, straightness),
        }

		Ok(Maze{rows: my_rows, columns: my_columns, maze_matrix: matrix})
    }

    pub fn init_rect_with_bitmask(my_rows: usize, my_columns: usize, wrap: usize, bitmask: Vec<Vec<bool>>, method: GenerationType) -> Result<Maze,MazeError> {
        
		if (my_rows == 0) || (my_columns == 0) {
			return Err(MazeError::Syntax("A maze requires non-zero dimensions.".to_string()));
        }

        if my_rows != bitmask.len() {
            return Err(MazeError::Syntax("Given matrix does not match given dimensions.".to_string()));
        }

        for i in 0..my_rows {
            if my_columns != bitmask[i].len() {
                return Err(MazeError::Syntax("Given matrix does not match given dimensions.".to_string()));
            }
        }

        get_starting_points(my_rows,my_columns,wrap,bitmask);
        
        let matrix;
        match method {
            GenerationType::Prim => matrix = prims_algorithm(my_rows, my_columns, wrap),
            GenerationType::Wilson => matrix = wilsons_algorithm(my_rows, my_columns, wrap),
            GenerationType::Backtrack(straightness) => matrix = bias_recursive_backtrack_algorithm(my_rows, my_columns, wrap, straightness),
        }

		Ok(Maze{rows: my_rows, columns: my_columns, maze_matrix: matrix})
    }

}

fn get_starting_points(my_rows: usize, my_columns: usize, wrap: usize, bitmask: Vec<Vec<bool>>) -> Vec<Point>{
    let mut anchor_points = Vec::new();
    let mut flood: HashSet<Point> = HashSet::new();
    let mut new_cells = HashSet::new();
    let mut added_cells;
    let all_directions = get_all_directions();
    for i in 0..my_rows {
        for j in 0..my_columns {
            if !flood.contains(&Point{row: i, col: j}) && bitmask[i][j] { //if the square hasn't been flooded and is not forbidden.
                anchor_points.push(Point{row: i, col: j});
                new_cells.insert(Point{row: i, col: j});
                added_cells = true;
                while added_cells { //flood adjcent tiles.
                    flood.extend(&new_cells);
                    let mut adjcent_cells = HashSet::new();
                    for current_cell in new_cells.iter() {
                        for a_direction in all_directions.iter() {
                            let next_cell = get_cell_in_direction(my_rows,my_columns,current_cell.row,current_cell.col,*a_direction,wrap);
                            if let Some(cell) = next_cell {
                                if !flood.contains(&cell) && !new_cells.contains(&cell) && bitmask[cell.row][cell.col] {
                                    adjcent_cells.insert(cell);
                                }
                            }
                        }
                    }
                    new_cells = adjcent_cells.clone();
                    if adjcent_cells.len() == 0 { //no new cells were added,
                        added_cells = false;
                    }
                }
            }
        }
    }
    return anchor_points;
}

//prims_algorithm
//Purpose:
//    Returns a rectangular gird with a maze that uses every square with no loops.
//Pre-Conditions:
//    The variables max_rows and max_cols are non-zero.
//Notes:
//  The alogirthm creates the maze uses Prim's algoirthm.
//  Wrap indicates if the rectangle should be considered as having its sides meet up.
//    A wrap of 0 means no wrapping, 1 means vertical wrapping, and 2 means wrapping on both pairs of sides.
fn prims_algorithm(my_rows: usize, my_columns: usize, wrap: usize) -> Vec<Vec<Compass>> {
    let mut path_matrix = vec![vec![Compass::init(); my_columns]; my_rows];
    //the check matrix will keep track of which squares are already in the maze.
    let mut check_matrix = vec![vec![false; my_columns]; my_rows];
    let mut walls = Vec::new(); //list of walls
    check_matrix[0][0] = true; //start with a square in the maze
    walls.push((Point{row: 0, col: 0},Direction::North)); //add starting walls to maze (A wall is a cell and a direction.)
    walls.push((Point{row: 0, col: 0},Direction::East));
    if wrap >= 1 {
        walls.push((Point{row: 0, col: 0},Direction::West));
    }
    if wrap >= 2 {
        walls.push((Point{row: 0, col: 0},Direction::South));
    }
    while walls.len() != 0 { //while there are still walls.
        //randomly select a wall
        let choice = rand::thread_rng().gen_range(0, walls.len());
        let current_cell = walls[choice].0;
        let current_dir = walls[choice].1;
        walls.remove(choice);//remove wall from list
        let next_cell = get_cell_in_direction(my_rows,my_columns,current_cell.row,current_cell.col,current_dir,wrap);
        if let Some(cell) = next_cell {
            if !check_matrix[cell.row][cell.col] {//if there is a cell on the other side and it hasn't been visited yet.
                walls.push((cell,Direction::East));//add walls of that cell
                walls.push((cell,Direction::West));//(Note: one of these is unnecessary, but you must compare with current_dir)
                walls.push((cell,Direction::North));//(the Wall which is not a wall will have no effect on the algoirthm,
                walls.push((cell,Direction::South));//since the cell on the other side is already part of the maze)
                check_matrix[cell.row][cell.col] = true;
                path_matrix[current_cell.row][current_cell.col] = path_matrix[current_cell.row][current_cell.col].add_dir(current_dir);
                path_matrix[cell.row][cell.col] = path_matrix[cell.row][cell.col].add_dir(current_dir.reverse());
            }
        }
    }
    return path_matrix;
}

//get_cell_in_direction
//Purpose:
//    Returns the cell reached by traveling in the directin given from the current cell (row,col).
//Pre-Conditions:
//    The point (row, col) is such that 0 <= row < max_rows and 0 <= col < max_cols.
//Notes:
//  Wrap indicates if the rectangle should be considered as having its sides meet up.
//    A wrap of 0 means no wrapping, 1 means vertical wrapping, and 2 means wrapping on both pairs of sides.
//  If one of the dimensions has size 1, and the direction move perpendictular, then it will return the original cell as the next one.
//  If a cell can not be reached then it returns None.
//  North increases the row count, South decreases the row count, East increases the column count, and West decreases the column count.
fn get_cell_in_direction(max_rows: usize, max_cols: usize, row: usize, col: usize, dir: Direction, wrap: usize) -> Option<Point> {
    let mut new_row = row;
    let mut new_col = col;
    let mut wrap_level = 0;// 0 if no wrapping, 1 if wrap around west/east, 2 if north/south and/or west/east.
    match (dir,row,col) {
        (Direction::North,row,_) if (row == max_rows-1) => {wrap_level = 2; new_row = 0}, //check if the cell is at the top while going north.
        (Direction::North,_,_) => new_row = row + 1,
        (Direction::South,0,_) => {wrap_level = 2; new_row = max_rows-1}, //check if the cell is at the bottom while going north.
        (Direction::South,_,_) => new_row = row - 1,
        (Direction::East,_,col) if (col == max_cols-1) => {wrap_level = 1; new_col = 0}, //check if the cell is at the right while going east.
        (Direction::East,_,_) => new_col = col + 1,
        (Direction::West,_,0) => {wrap_level = 1; new_col = max_cols-1}, //check if the cell is at the left while going west.
        (Direction::West,_,_) => new_col = col - 1,
    };

    if wrap_level > wrap {
        return None;
    }else {
        return Some(Point{row: new_row, col: new_col});
    }
}

//bias_recursive_backtrack_algorithm
//Purpose:
//    Returns a rectangular gird with a maze that uses every square with no loops.
//Pre-Conditions:
//    The variables max_rows and max_cols are non-zero.
//Notes:
//  The alogirthm creates the maze uses a biased recursive backtrack algorithm
//  Wrap indicates if the rectangle should be considered as having its sides meet up.
//    A wrap of 0 means no wrapping, 1 means vertical wrapping, and 2 means wrapping on both pairs of sides.
fn bias_recursive_backtrack_algorithm(my_rows: usize, my_columns: usize, wrap: usize, straightness: f64) -> Vec<Vec<Compass>> {
    let mut path_matrix = vec![vec![Compass::init(); my_columns]; my_rows];
    //the check matrix will keep track of which squares are already in the maze.
    let mut check_matrix = vec![vec![false; my_columns]; my_rows];
    let mut cells = Vec::new();
    cells.push(Point{row: 0, col: 0});
    check_matrix[0][0] = true; //start with a square in the maze
    let mut my_starting_directions = vec![Direction::North,Direction::East]; // start with a random direction (check for wrapping)
    if wrap >= 1 {
        my_starting_directions.push(Direction::West);
    }
    if wrap >= 2 {
        my_starting_directions.push(Direction::South);
    }
    let mut choice = rand::thread_rng().gen_range(0, my_starting_directions.len());
    let mut current_direction = my_starting_directions[choice];
    while cells.len() != 0 { //while there are still cells.
        let current_cell = cells[cells.len()-1]; // grab the cell at the top of the stack
        let mut nearby_cells = Vec::new();
        let mut continue_in_current_direction = Vec::new();
        let mut continue_in_other_direction = Vec::new();
        let all_directions = get_all_directions();
        for a_direction in all_directions.iter() { // get valid moves
            if let Some(cell) = get_cell_in_direction(my_rows,my_columns,current_cell.row,current_cell.col,*a_direction,wrap) {//if we can move in that direction
                if check_matrix[cell.row][cell.col] == false { // and the cell has not yet been used, add it.
                    nearby_cells.push((a_direction,cell));
                    if *a_direction == current_direction {
                        continue_in_current_direction.push((a_direction,cell));
                    }else{
                        continue_in_other_direction.push((a_direction,cell));
                    }
                }
            }
        }

        if nearby_cells.len() == 0 { //pop the cell of the stack since it is a deadend.
            cells.pop().unwrap();
        } else { //keep making a trail.
            let next_cell_data;
            if nearby_cells.len() == 1 { //there is only one option, so take that
                next_cell_data = nearby_cells[0];
            }else if continue_in_current_direction.len() != 0 { // if is possible to continue in a straight line.
                let prob = rand::thread_rng().gen_range(0.0,1.0);
                if prob <= 0.33 + (0.42*(straightness)) {
                    next_cell_data = continue_in_current_direction[0];
                }else{
                    choice = rand::thread_rng().gen_range(0, continue_in_other_direction.len());
                    next_cell_data = continue_in_other_direction[choice];
                }
            }else{
                choice = rand::thread_rng().gen_range(0, continue_in_other_direction.len());
                next_cell_data = continue_in_other_direction[choice];
            }
            let next_cell = next_cell_data.1;
            current_direction = *next_cell_data.0;
            cells.push(next_cell);
            path_matrix[current_cell.row][current_cell.col] = path_matrix[current_cell.row][current_cell.col].add_dir(current_direction);
            path_matrix[next_cell.row][next_cell.col] = path_matrix[next_cell.row][next_cell.col].add_dir(current_direction.reverse());
            check_matrix[next_cell.row][next_cell.col] = true;
        }
    }
    return path_matrix;
}

//wilsons_algorithm
//Purpose:
//    Returns a rectangular gird with a maze that uses every square with no loops.
//Pre-Conditions:
//    The variables max_rows and max_cols are non-zero.
//Notes:
//  The alogirthm creates the maze uniformly at random.
//  Wrap indicates if the rectangle should be considered as having its sides meet up.
//    A wrap of 0 means no wrapping, 1 means vertical wrapping, and 2 means wrapping on both pairs of sides.
//Bugs:
//  If the row size or column size is two, the function will can't tell if the path went north/south east/west, and will wrap around.
fn wilsons_algorithm(my_rows: usize, my_columns: usize, wrap: usize) -> Vec<Vec<Compass>> {
    let mut path_matrix = vec![vec![Compass::init(); my_columns]; my_rows];
    //the check matrix will keep track of which squares are already in the maze.
    let mut check_matrix = vec![vec![false; my_columns]; my_rows];
    check_matrix[0][0] = true;//start with a square in the maze
    for row in 0..my_rows {
        for col in 0..my_columns {
            if !check_matrix[row][col] { // if the current square is not already in the maze
                let mut trail = Vec::new();
                let mut trail_directions = Vec::new();
                let mut current_square = Point{row: row, col: col};
                let mut current_direction;
                trail.push(current_square);
                while check_matrix[current_square.row][current_square.col] != true { //preform a loop erased random walk
                    let neighbor_data = get_random_neighbor(my_rows,my_columns,current_square.row,current_square.col,wrap);
                    current_square = neighbor_data.0;
                    current_direction = neighbor_data.1;
                    trail_directions.push(current_direction);
                    if trail.contains(&current_square) { // if we loop
                        let index = trail.iter().position(|&r| r == current_square).unwrap();
                        trail.truncate(index+1); //erase loop
                        trail_directions.truncate(index);
                    } else {
                        trail.push(current_square);
                    }
                }// we have met back up with squares from the maze.
                //add new trail to the maze.

                for k in 0..(trail.len()-1) { // note that since we start at a square not already in the maze tha trail is at least 2.
                    check_matrix[trail[k].row][trail[k].col] = true;
                    path_matrix[trail[k].row][trail[k].col] = path_matrix[trail[k].row][trail[k].col].add_dir(trail_directions[k]);
                    path_matrix[trail[k+1].row][trail[k+1].col] = path_matrix[trail[k+1].row][trail[k+1].col].add_dir(trail_directions[k].reverse());
                }
            }
        }
    }
    return path_matrix;
}

//get_random_neighbor
//Purpose:
//    Returns a random neighbor of a point (row,col) in a rectangular gird of size max_rows and max_cols.
//Pre-Conditions:
//    The point (row, col) is such that 0 <= row < max_rows and 0 <= col < max_cols.
//    The conditions are such that the cells has a neighbor, if there is no wrapping and
//    max_rows = max_cols = 1 the function will crash
//    (But such a call should never happen in the first place.)
//Notes:
//  Wrap indicates if the rectangle should be considered as having its sides meet up.
//    A wrap of 0 means no wrapping, 1 means vertical wrapping, and 2 means wrapping on both pairs of sides.
fn get_random_neighbor(max_rows: usize, max_cols: usize, row: usize, col: usize, wrap: usize) -> (Point,Direction){
    let mut neighbors = Vec::new();
    if ((col + 1) < max_cols) || (wrap >= 1) {
        neighbors.push((Point{row: row, col: (col+1)%max_cols},Direction::East));
    }
    if (col > 0) || (wrap >= 1) {
        neighbors.push((Point{row: row, col: (col+max_cols-1)%max_cols},Direction::West));
    }
    if ((row + 1) < max_rows) || (wrap >= 2) {
        neighbors.push((Point{row: (row+1)%max_rows, col: col},Direction::North));
    }
    if (row > 0) || (wrap >= 2) {
        neighbors.push((Point{row: (row+max_rows-1)%max_rows, col: col},Direction::South));
    }
    let choice = rand::thread_rng().gen_range(0, neighbors.len());
    return neighbors[choice].clone();
}