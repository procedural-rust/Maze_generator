//Author: Everett Sullivan
//Date Created: 11/27/2019
//Purpose: To expeirment with maze and dungeon generation algorithms.

use std::fs::File;
use std::io::{BufWriter, Write};
use std::env;
use std::process;

mod maze;
use maze::Maze;
use maze::Direction;
use maze::GenerationType;

mod dungeon;

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
        .arg(Arg::with_name("num_rooms")
            .help("Maximum number of rooms allowed in the dungeon.")
            .index(3)
            .required(true))
        .arg(Arg::with_name("dead_end_rm_ratio")
            .help("Ratio of dead-ends to remove.")
            .index(4)
            .required(true))
        .arg(Arg::with_name("output_file")
            .help("Sets the name of the output file.")
            .index(5)
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
                   Must also state the dimension of each square in pixels. Default 10.")
            .takes_value(true)
            .short("i")
            .long("image"))
        .arg(Arg::with_name("wrapping")
            .help("The maze is allowed to pass outside the grid by wrapping to the other side.
                   Use once for a maze on a tube, and twice for a maze on a torus.")
            .takes_value(false)
            .long("wrap")
            .multiple(true))
        .arg(Arg::with_name("outside")
            .help("If persent, allows the dungeon to have exits off the map. If wrapping is present this is ignored.")
            .short("o")
            .long("outside"))
        .get_matches();

    print!("Using the following arguments to generate from:\n");
    print!("x: {}    y: {}    num_rooms: {}    dead_end_rm_ratio: {}    Caves: {}    Prim: {}    Wilson: {}    Wrapping: {}    outside: {}\n",
        matches.value_of("maze x_length").unwrap().to_string(),
        matches.value_of("maze y_length").unwrap().to_string(),
        matches.value_of("num_rooms").unwrap().to_string(),
        matches.value_of("dead_end_rm_ratio").unwrap().to_string(),
        matches.is_present("cave"),
        matches.is_present("prim"),
        matches.is_present("wilson"),
        matches.is_present("wrapping"),
        matches.is_present("outside"),
    );
    print!("output file: {}\n", matches.value_of("output_file").unwrap().to_string());

    //safe to unwrap since the arugment is required.
    let output_file_name: String = matches.value_of("output_file").unwrap().to_string();
    let rows = matches.value_of("maze x_length").unwrap().parse::<usize>().unwrap();
    let columns = matches.value_of("maze y_length").unwrap().parse::<usize>().unwrap();
    let wrap = matches.occurrences_of("wrapping");

    if matches.is_present("cave"){
        let my_cave = Cave::init_cave(rows,columns).unwrap();
        match matches.value_of("image") {
            Some(block_size) => {
                let mut block = block_size.parse::<usize>().unwrap();
                if block < 10 {
                    block = 10
                }
                print_picture_cave(&my_cave, output_file_name, block)
            },
            None => print_cave(&my_cave,output_file_name),
        }
    } else {
        let num_rooms = matches.value_of("num_rooms").unwrap().parse::<usize>().unwrap();
        let outside_exits = matches.is_present("outside");
        let prune_dead_ends = matches.value_of("dead_end_rm_ratio").unwrap().parse::<f64>().unwrap();
        let generation_method;
        let (wilson, prim, backtrack) = (matches.is_present("wilson"),matches.is_present("prim"),matches.is_present("backtrack"));
        print!("{} {} {} {}\n", wilson, prim, backtrack, wilson || prim || backtrack);
        // sanity check before generation
        if !(wilson || prim || backtrack) {
          print!("You must select (w)ilson, (p)rim or (b)acktrack when not running (c)aves. \nExiting.");
          process::exit(1);
        }
        match (wilson, prim, backtrack){
            (true,_,_) => generation_method = GenerationType::Wilson,
            (_,true,_) => generation_method = GenerationType::Prim,
            (_,_,true) => generation_method = GenerationType::Backtrack(matches.value_of("backtrack").unwrap().parse::<f64>().unwrap()),
            _ => unreachable!(),
        }
        match matches.value_of("image") {
            Some(block_size) => {
                let mut block = block_size.parse::<usize>().unwrap();
                if block < 10 {
                    block = 10
                }
                dungeon::print_dungeon_as_image(&dungeon::create_dungeon(rows, columns, wrap as usize,generation_method,num_rooms,prune_dead_ends,outside_exits).unwrap(),output_file_name,block);
            },
            None => dungeon::print_dugenon_to_file(&dungeon::create_dungeon(rows, columns, wrap as usize,generation_method,num_rooms,prune_dead_ends,outside_exits).unwrap(),output_file_name),
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
    f.flush().unwrap();
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

    if output_file_name.contains(".jpeg") || output_file_name.contains(".png") {
        imgbuf.save(output_file_name).unwrap();
    } else {
        imgbuf.save(output_file_name + ".png").unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prim_test() {
        let my_maze_test = Maze::init_rect(40,60,maze::NO_SQUARE_WRAP,GenerationType::Prim).unwrap();
        dungeon::print_dungeon_as_image(&dungeon::maze_to_map(&my_maze_test).unwrap(),"test_files\\Prim_Basic_test.png".to_string(),5);
    }

    #[test]
    fn prim_ring_test() {
        let my_maze_test = Maze::init_rect(40,60,maze::RING_SQUARE_WRAP,GenerationType::Prim).unwrap();
        dungeon::print_dungeon_as_image(&dungeon::maze_to_map(&my_maze_test).unwrap(),"test_files\\Prim_Ring_Wrap_test.png".to_string(),5);
    }

    #[test]
    fn prim_tours_test() {
        let my_maze_test = Maze::init_rect(40,60,maze::MAX_SQUARE_WRAP,GenerationType::Prim).unwrap();
        dungeon::print_dungeon_as_image(&dungeon::maze_to_map(&my_maze_test).unwrap(),"test_files\\Prim_Tours_Wrap_test.png".to_string(),5);
    }

    #[test]
    fn wilson_test() {
        let my_maze_test = Maze::init_rect(40,60,maze::NO_SQUARE_WRAP,GenerationType::Wilson).unwrap();
        dungeon::print_dungeon_as_image(&dungeon::maze_to_map(&my_maze_test).unwrap(),"test_files\\Wilson_Basic_test.png".to_string(),5);
    }

    #[test]
    fn wilson_ring_test() {
        let my_maze_test = Maze::init_rect(40,60,maze::RING_SQUARE_WRAP,GenerationType::Wilson).unwrap();
        dungeon::print_dungeon_as_image(&dungeon::maze_to_map(&my_maze_test).unwrap(),"test_files\\Wilson_Ring_Wrap_test.png".to_string(),5);
    }

    #[test]
    fn wilson_tours_test() {
        let my_maze_test = Maze::init_rect(40,60,maze::MAX_SQUARE_WRAP,GenerationType::Wilson).unwrap();
        dungeon::print_dungeon_as_image(&dungeon::maze_to_map(&my_maze_test).unwrap(),"test_files\\Wilson_Tours_Wrap_test.png".to_string(),5);
    }

    #[test]
    fn backtrack_twisty_test() {
        let my_maze_test = Maze::init_rect(40,60,maze::NO_SQUARE_WRAP,GenerationType::Wilson).unwrap();
        dungeon::print_dungeon_as_image(&dungeon::maze_to_map(&my_maze_test).unwrap(),"test_files\\Backtrack_Twisty_Basic_test.png".to_string(),5);
    }

    #[test]
    fn backtrack_twisty_ring_test() {
        let my_maze_test = Maze::init_rect(40,60,maze::RING_SQUARE_WRAP,GenerationType::Wilson).unwrap();
        dungeon::print_dungeon_as_image(&dungeon::maze_to_map(&my_maze_test).unwrap(),"test_files\\Backtrack_Twisty_Ring_Wrap_test.png".to_string(),5);
    }

    #[test]
    fn backtrack_twisty_tours_test() {
        let my_maze_test = Maze::init_rect(40,60,maze::MAX_SQUARE_WRAP,GenerationType::Wilson).unwrap();
        dungeon::print_dungeon_as_image(&dungeon::maze_to_map(&my_maze_test).unwrap(),"test_files\\Backtrack_Twisty_Tours_Wrap_test.png".to_string(),5);
    }

    #[test]
    fn test_bitmask() {

        let mut bitmask = vec![vec![true; 40]; 40];
        for i in 0..20{
            for j in 0..20 {
                bitmask[i+10][j+10] = false;
            }
        }
        let my_maze_test = Maze::init_rect_with_bitmask(40,40,maze::NO_SQUARE_WRAP, &bitmask,GenerationType::Backtrack(1.0)).unwrap();
        dungeon::print_dungeon_as_image(&dungeon::maze_to_map(&my_maze_test).unwrap(),"test_files\\Backtract_Bitmask_Doughnut_test.png".to_string(),5);
    }

    #[test]
    fn test_erase_dead_ends() {
        use rand::{Rng, StdRng};

        let mut my_maze_test = Maze::init_rect(40,40,maze::MAX_SQUARE_WRAP,GenerationType::Backtrack(0.75)).unwrap();
        let mut my_dead_ends = my_maze_test.get_dead_ends();
        let mut rng = StdRng::new().unwrap();
        rng.shuffle(&mut my_dead_ends);
        let stop = ((my_dead_ends.len() as f64)*0.5) as usize;
        for i in 0..stop {
            my_maze_test.erase_dead_end(my_dead_ends[i]);
        }
        dungeon::print_dungeon_as_image(&dungeon::maze_to_map(&my_maze_test).unwrap(),"test_files\\Backtract_Dead_End_Erase_test.png".to_string(),5);
    }

    #[test]
    fn test_dungeon() {
        dungeon::print_dungeon_as_image(&dungeon::create_dungeon(23,43,0,GenerationType::Backtrack(0.75),10,0.9,true).unwrap(),"test_files\\Dungeon_test.png".to_string(),5);
    }

}