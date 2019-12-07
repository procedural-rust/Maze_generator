//Author: Everett Sullivan
//Date Created: 11/27/2019
//Purpose: To expeirment with maze and dungeon generation algorithms.

use std::fs::File;
use std::io::{BufWriter, Write};

mod maze;
use maze::Maze;
use maze::Direction;
use maze::GenerationType;

mod cave;
use cave::Cave;

extern crate clap;
use clap::{Arg, App, ArgGroup};

fn main() {

        let matches = App::new("Maze Maker")
        .version("1.0")
        .author("Everett Sullivan")
        .about("Create Mazes")
        .arg(Arg::with_name("maze x_length")
                                    .help("Sets the width of the maze. Must be a positive integer.")
                                    .index(1)
                                    .required(true))
        .arg(Arg::with_name("maze y_length")
                                    .help("Sets the depth of the maze. Must be a positive integer.")
                                    .index(2)
                                    .required(true))
        .arg(Arg::with_name("output_file")
                                    .help("Sets the name of the output file.")
                                    .index(3)
                                    .required(true))
        .arg(Arg::with_name("cave")
            .help("The program will generate a cave.")
            .short("c")
            .long("cave"))
        .arg(Arg::with_name("wilson")
            .help("The program will generate the maze with Wilson's Algoirthm")
            .short("w")
            .long("wilson"))
        .arg(Arg::with_name("prim")
            .help("The program will generate the maze with Prim's Algoirthm.")
            .short("p")
            .long("prim"))
        .arg(Arg::with_name("backtrack")
            .help("The program will generate the maze with a backtrack algorithm, requires a number between 0 and 1.")
            .takes_value(true)
            .short("b")
            .long("backtrack"))
        .group(
            ArgGroup::with_name("Generation Method")
                .args(&["wilson","prim","backtrack","cave"]))
        .arg(Arg::with_name("image")
            .help("The program will encode the maze as a png image instead of a text image.
                   Must also state the dimension of each square in pixels.")
            .takes_value(true)
            .short("i")
            .long("image"))
        .arg(Arg::with_name("wrapping")
            .help("The maze is allowed to pass outside the grid by wrapping to the other side.
                   Use once for a maze on a tube, and twice for a maze on a torus.")
            .takes_value(false)
            .long("wrap")
            .multiple(true))
        .get_matches();

    //safe to unwrap since the arugment is required.
    let output_file_name: String = matches.value_of("output_file").unwrap().to_string();
    let rows = matches.value_of("maze x_length").unwrap().parse::<usize>().unwrap();
    let columns = matches.value_of("maze y_length").unwrap().parse::<usize>().unwrap();
    let wrap = matches.occurrences_of("wrapping");

    if matches.is_present("cave"){
        let my_cave = Cave::init_cave(rows,columns).unwrap();
        match matches.value_of("image") {
            Some(block_size) => print_picture_cave(&my_cave,output_file_name,block_size.parse::<usize>().unwrap()),
            None => print_cave(&my_cave,output_file_name),
        }
    } else { //we generate a maze
        let (wilson,prim,backtrack) = (matches.is_present("wilson"),matches.is_present("prim"),matches.is_present("backtrack"));
        let my_maze;
        match (wilson,prim,backtrack){
            (true,_,_) => my_maze = Maze::init_rect(rows,columns,wrap as usize,GenerationType::Wilson).unwrap(),
            (_,true,_) => my_maze = Maze::init_rect(rows,columns,wrap as usize,GenerationType::Prim).unwrap(),
            (_,_,true) => my_maze = Maze::init_rect(rows,columns,wrap as usize,GenerationType::Backtrack(matches.value_of("backtrack").unwrap().parse::<f64>().unwrap())).unwrap(),
            _ => unreachable!(),
        }
    
        match matches.value_of("image") {
            Some(block_size) => print_picture_maze(&my_maze,output_file_name,block_size.parse::<usize>().unwrap()),
            None => print_maze(&my_maze,output_file_name),
        }
    }
}

fn print_cave(my_cave: &Cave, output_file_name: String){
    let file = File::create(output_file_name).expect("Unable to create file");
    let mut f = BufWriter::new(file);
    for i in 0..my_cave.rows {
        for j in 0..my_cave.columns {
            if my_cave.cave_matrix[i][j] { //if there is a wall
                f.write("#".as_bytes()).unwrap();
            }else{
                f.write(" ".as_bytes()).unwrap();
            }
        }
        f.write("\n".as_bytes()).unwrap();
    }
}

fn print_picture_cave(my_cave: &Cave, output_file_name: String, block_size: usize){
    let block_size_u32 = block_size as u32;
    let mut imgbuf = image::ImageBuffer::new((block_size*my_cave.columns) as u32, (block_size*my_cave.rows) as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        if my_cave.cave_matrix[(x/block_size_u32) as usize][(y/block_size_u32) as usize] == true {
            *pixel = image::Rgb([0, 0, 0]);
        }else {
            *pixel = image::Rgb([255, 255, 255]);
        }
    }

    imgbuf.save(output_file_name).unwrap();
}

fn print_maze(my_maze: &Maze, output_file_name: String){
    let file = File::create(output_file_name).expect("Unable to create file");
    let mut f = BufWriter::new(file);
    f.write("#".as_bytes()).unwrap();
    for i in 0..my_maze.columns {
        if my_maze.maze_matrix[my_maze.rows-1][i].has_dir(Direction::North) == true {
            f.write(" ".as_bytes()).unwrap();
        }else{
            f.write("#".as_bytes()).unwrap();
        }
        f.write("#".as_bytes()).unwrap();
    }
    f.write("\n".as_bytes()).unwrap();
    for i in (0..my_maze.rows).rev() {
        if my_maze.maze_matrix[i][0].has_dir(Direction::West) == true {
            f.write(" ".as_bytes()).unwrap();
        }else{
            f.write("#".as_bytes()).unwrap();
        }
        for j in 0..my_maze.columns{
            f.write(" ".as_bytes()).unwrap();
            if my_maze.maze_matrix[i][j].has_dir(Direction::East) == true {
            f.write(" ".as_bytes()).unwrap();
            }else{
                f.write("#".as_bytes()).unwrap();
            }
        }
        f.write("\n".as_bytes()).unwrap();
        f.write("#".as_bytes()).unwrap();
        for j in 0..my_maze.columns {
            if my_maze.maze_matrix[i][j].has_dir(Direction::South) == true {
                f.write(" ".as_bytes()).unwrap();
            }else{
                f.write("#".as_bytes()).unwrap();
            }
            f.write("#".as_bytes()).unwrap();
        }
        f.write("\n".as_bytes()).unwrap();
    }
    f.flush().unwrap();
}

fn print_picture_maze(my_maze: &Maze, output_file_name: String, block_size: usize){

    let block_size_u32 = block_size as u32;
    let mut imgbuf = image::ImageBuffer::new((block_size*(2*my_maze.columns + 1)) as u32, (block_size*(2*my_maze.rows + 1)) as u32);
    let mut wall_matrix = vec![vec![0; 2*my_maze.rows+1]; 2*my_maze.columns+1];
    wall_matrix[0][2*my_maze.rows] = 1;
    for i in 0..my_maze.columns {
        if my_maze.maze_matrix[my_maze.rows-1][i].has_dir(Direction::North) == false {
            wall_matrix[2*i+1][2*my_maze.rows] = 1;
        }
        wall_matrix[2*i+2][2*my_maze.rows] = 1;
    }
    for i in (0..my_maze.rows).rev() {
        if my_maze.maze_matrix[i][0].has_dir(Direction::West) == false {
            wall_matrix[0][2*my_maze.rows-1-2*i] = 1;
        }
        for j in 0..my_maze.columns{
            if my_maze.maze_matrix[i][j].has_dir(Direction::East) == false {
                wall_matrix[2*j+2][2*my_maze.rows-1-2*i] = 1;
            }
        }
        wall_matrix[0][2*my_maze.rows-2-2*i] = 1;
        for j in 0..my_maze.columns {
            if my_maze.maze_matrix[i][j].has_dir(Direction::North) == false {
                wall_matrix[2*j+1][2*my_maze.rows-2-2*i] = 1;
            }
            wall_matrix[2*j+2][2*my_maze.rows-2-2*i] = 1;
        }
    }

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        if wall_matrix[(x/block_size_u32) as usize][(y/block_size_u32) as usize] == 1 {
            *pixel = image::Rgb([0, 0, 0]);
        }else {
            *pixel = image::Rgb([255, 255, 255]);
        }
    }

    imgbuf.save(output_file_name).unwrap();
}
