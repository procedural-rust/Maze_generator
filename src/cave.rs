//Author: Everett Sullivan
//Date Created: 11/28/2019
//Purpose To create caves
//Notes:

use rand::Rng;

const NUM_OF_ITERS: usize = 3;
const DEFULT_WALL_PROB: f64 = 0.45;

////////////////////
//Custom Error handling code
////////////////////

#[derive(Debug)]
pub enum CaveError {
    Syntax(String),
}

use std::fmt;
use std::error::Error;

impl fmt::Display for CaveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CaveError::Syntax(ref err_string) => write!(f,"{}",err_string),
        }
    }
}

impl Error for CaveError {
    fn cause(&self) -> Option<&Error> {
        match *self {
            CaveError::Syntax(ref _err_string) => None,
        }
    }
}

////////////////////
//Cave code
////////////////////

#[derive(Debug)]
pub struct Cave {
    pub rows: usize,
	pub columns: usize,
	pub cave_matrix: Vec<Vec<bool>>
}

impl Cave {

    pub fn init_cave(my_rows: usize, my_columns: usize) -> Result<Cave,CaveError> {
		let condition_grid = vec![vec![1; my_columns]; my_rows];

		return Cave::init_cave_with_conditions(my_rows,my_columns,DEFULT_WALL_PROB,condition_grid);
    }

    //init_cave_with_conditions
    //Purpose:
    //    Creates a rectangular cave using
    //Pre-Conditions:
    //    The variables max_rows and max_cols are non-zero.
    //Notes:
    //  Wrap indicates if the rectangle should be considered as having its sides meet up.
    //    A wrap of 0 means no wrapping, 1 means vertical wrapping, and 2 means wrapping on both pairs of sides.
    pub fn init_cave_with_conditions(my_rows: usize, my_columns: usize, wall_prob: f64, condition_grid: Vec<Vec<usize>>) -> Result<Cave,CaveError> {
		if (my_rows == 0) || (my_columns == 0) {
			return Err(CaveError::Syntax("A cave requires non-zero dimensions.".to_string()));
		}else{
			if (my_rows as usize) != condition_grid.len() {
				return Err(CaveError::Syntax("Given matrix does not match given dimensions.".to_string()));
			}

			for i in 0..my_rows {
				if (my_columns as usize) != condition_grid[i as usize].len() {
					return Err(CaveError::Syntax("Given matrix does not match given dimensions.".to_string()));
				}
			}
		}

        let mut matrix = init_cave_matrix(my_rows, my_columns, wall_prob, &condition_grid);

        for _i in 0..NUM_OF_ITERS {
            cell_auto_iter(my_rows, my_columns, &mut matrix, &condition_grid)
        }

		Ok(Cave{rows: my_rows, columns: my_columns, cave_matrix: matrix})
    }

}

fn init_cave_matrix(my_rows: usize, my_columns: usize, wall_prob: f64, condition_grid: &Vec<Vec<usize>>) -> Vec<Vec<bool>> {
    let mut cave_matrix = vec![vec![false; my_columns]; my_rows];
    for i in 0..my_rows {
        for j in 0..my_columns {
            if condition_grid[i][j] == 2 { //If we must have a wall, create a wall
                cave_matrix[i][j] = true;
            } else if condition_grid[i][j] == 1 { //If we randomly create a wall, create all wall with probability wall_prob
                let rand_value = rand::thread_rng().gen_range(0.0, 1.0);
                if rand_value <= wall_prob {
                    cave_matrix[i][j] = true;
                }
            }
        }
    }
    return cave_matrix;
}

fn cell_auto_iter(my_rows: usize, my_columns: usize, cave_matrix: &mut Vec<Vec<bool>>, condition_grid: &Vec<Vec<usize>>){
    for i in 0..my_rows {
        for j in 0..my_columns {
            if condition_grid[i][j] == 1 { //If a wall is not predetmined to either be there or not, use cell laws to advance.
                let neightbor_ratio = neighbor_wall_ratio(my_rows,my_columns,i,j,&cave_matrix);
                if (neightbor_ratio >= 0.6) && (cave_matrix[i][j] == false) {
                    cave_matrix[i][j] = true;
                }else if (neightbor_ratio < 0.5) && (cave_matrix[i][j] == true){
                   cave_matrix[i][j] = false;
                }
            }
        }
    }
}

fn neighbor_wall_ratio(my_rows: usize, my_columns: usize, my_row: usize, my_column: usize, cave_matrix: &Vec<Vec<bool>>) -> f64{
    let mut neighbors = 0.0;
    let mut neighboring_walls = 0.0;
    for i in 0..3 {
        for j in 0..3 {
            if (i != 1) || (j != 1) {
                if (my_row + i >= 1) && (my_row + i < my_rows + 1) && (my_column + j >= 1) && (my_column + j < my_columns + 1){ // usize, so must make sure not to subtract from zero
                    neighbors = neighbors + 1.0;
                    if cave_matrix[my_row + i-1][my_column+j-1] == true { //if there is a wall
                        neighboring_walls = neighboring_walls + 1.0;
                    }
                }
            }
        }
    }
    return neighboring_walls/neighbors;
}