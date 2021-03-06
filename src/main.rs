extern crate rand;
mod cube;

use cube::Cube;
use cube::Side;
use std::thread;

static NUMBER_OF_MOVES: u8 = 19; //Number of random moves applied to cube. Setting this higher will take a long time by a factor of 11^N 

static mut SOLVE_DEPTH: u8 = NUMBER_OF_MOVES + 1;

fn main() {
    let cube = build_cube().scramble_cube(NUMBER_OF_MOVES);

    cube.print_cube();
    println!(
        "Applied {} random moves to cube: {:?}",
        cube.num_moves, cube.previous_moves
    );

    let cube = cube.forget_moves(); //Blanks the list of previous moves, keeps the state of the cube faces.

    let handles: Vec<_> = starter_cubes(cube) //12 threads, based on the first 12 states of the cube.
        .into_iter()
        .map(|c| {
            thread::spawn(move || {
                solve_cube(c);
            })
        }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
fn starter_cubes(old_cube: Cube) -> Vec<Cube> {
    //The first 12 cubes for thread seed data.
    let in_cube = old_cube.copy_cube();
    let mut out_cubes = vec![];
    out_cubes.push(in_cube.rotate_back_clockwise());
    out_cubes.push(in_cube.rotate_back_counter_clockwise());
    out_cubes.push(in_cube.rotate_down_clockwise());
    out_cubes.push(in_cube.rotate_down_counter_clockwise());
    out_cubes.push(in_cube.rotate_front_clockwise());
    out_cubes.push(in_cube.rotate_front_counter_clockwise());
    out_cubes.push(in_cube.rotate_left_clockwise());
    out_cubes.push(in_cube.rotate_left_counter_clockwise());
    out_cubes.push(in_cube.rotate_right_clockwise());
    out_cubes.push(in_cube.rotate_right_counter_clockwise());
    out_cubes.push(in_cube.rotate_up_clockwise());
    out_cubes.push(in_cube.rotate_up_counter_clockwise());
    out_cubes
}
fn solve_cube(in_cube: Cube) -> () {
    unsafe {
        if in_cube.num_moves >= SOLVE_DEPTH {
            return;
        }
    }
    if in_cube.is_solved() {
        unsafe {
            if in_cube.num_moves < SOLVE_DEPTH {
                SOLVE_DEPTH = in_cube.num_moves;
            }
        }
        println!(
            "Found solution in {} moves:  {:?}",
            in_cube.num_moves, in_cube.previous_moves
        );
    } else {
        //This statement prevents doing the opposite of the previous move, which would revert the previous move and put it to a state that has already been checked.
        //From benchmarking tests, this reduces runtime by about 30%
        // need to also stop f b f' b' f b f' b' loops
//OLD WAY:        let last_move = in_cube.previous_moves.last();
        let last_moves = in_cube.previous_moves.iter().rev().take(4).collect::<Vec<_>>();
        //last_move='z';
        let mut last_move = in_cube.previous_moves.last();
//println!("{:?}",last_moves);
if in_cube.num_moves >=5 {
    if last_moves[0] == last_moves[2]{
    //println!("SAME!!!!");
    last_move = Some(last_moves[2]);
}}
    //TODO IF AXIS IS OVERLOADED, FORCE PERPENDICULAR TURN
    // D2 U2 disallows d' u' D U and forces FRBL(')
    //CASES Z X AND Y MARK THE LOADED AXIS
    //ghjkto cases need to be added
        match last_move {
            Some('F') => {
                solve_cube(in_cube.rotate_back_clockwise());
                solve_cube(in_cube.rotate_back_counter_clockwise());
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());

                solve_cube(in_cube.rotate_back_twice());
                solve_cube(in_cube.rotate_up_twice());
                solve_cube(in_cube.rotate_right_twice());
                solve_cube(in_cube.rotate_left_twice());
                //solve_cube(in_cube.rotate_front_twice());
                solve_cube(in_cube.rotate_down_twice());
                
            }
            Some('f') => {
                solve_cube(in_cube.rotate_back_clockwise());
                solve_cube(in_cube.rotate_back_counter_clockwise());
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());

                solve_cube(in_cube.rotate_back_twice());
                solve_cube(in_cube.rotate_up_twice());
                solve_cube(in_cube.rotate_right_twice());
                solve_cube(in_cube.rotate_left_twice());
                //solve_cube(in_cube.rotate_front_twice());
                solve_cube(in_cube.rotate_down_twice());
            }
            Some('U') => {
                solve_cube(in_cube.rotate_back_clockwise());
                solve_cube(in_cube.rotate_back_counter_clockwise());
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                solve_cube(in_cube.rotate_front_clockwise());
                solve_cube(in_cube.rotate_front_counter_clockwise());
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());

                solve_cube(in_cube.rotate_back_twice());
                //solve_cube(in_cube.rotate_up_twice());
                solve_cube(in_cube.rotate_right_twice());
                solve_cube(in_cube.rotate_left_twice());
                solve_cube(in_cube.rotate_front_twice());
                solve_cube(in_cube.rotate_down_twice());
            }

            Some('u') => {
                solve_cube(in_cube.rotate_back_clockwise());
                solve_cube(in_cube.rotate_back_counter_clockwise());
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                solve_cube(in_cube.rotate_front_clockwise());
                solve_cube(in_cube.rotate_front_counter_clockwise());
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());

                solve_cube(in_cube.rotate_back_twice());
                //solve_cube(in_cube.rotate_up_twice());
                solve_cube(in_cube.rotate_right_twice());
                solve_cube(in_cube.rotate_left_twice());
                solve_cube(in_cube.rotate_front_twice());
                solve_cube(in_cube.rotate_down_twice());
            }
            Some('D') => {
                solve_cube(in_cube.rotate_back_clockwise());
                solve_cube(in_cube.rotate_back_counter_clockwise());
                solve_cube(in_cube.rotate_front_clockwise());
                solve_cube(in_cube.rotate_front_counter_clockwise());
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());

                solve_cube(in_cube.rotate_back_twice());
                solve_cube(in_cube.rotate_up_twice());
                solve_cube(in_cube.rotate_right_twice());
                solve_cube(in_cube.rotate_left_twice());
                //solve_cube(in_cube.rotate_front_twice());
                solve_cube(in_cube.rotate_down_twice());
            }
            Some('d') => {
                solve_cube(in_cube.rotate_back_clockwise());
                solve_cube(in_cube.rotate_back_counter_clockwise());
                solve_cube(in_cube.rotate_front_clockwise());
                solve_cube(in_cube.rotate_front_counter_clockwise());
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());

                solve_cube(in_cube.rotate_back_twice());
                solve_cube(in_cube.rotate_up_twice());
                solve_cube(in_cube.rotate_right_twice());
                solve_cube(in_cube.rotate_left_twice());
                solve_cube(in_cube.rotate_front_twice());
                //solve_cube(in_cube.rotate_down_twice());
            }
            Some('L') => {
                solve_cube(in_cube.rotate_back_clockwise());
                solve_cube(in_cube.rotate_back_counter_clockwise());
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                solve_cube(in_cube.rotate_front_clockwise());
                solve_cube(in_cube.rotate_front_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());

                solve_cube(in_cube.rotate_back_twice());
                solve_cube(in_cube.rotate_up_twice());
                solve_cube(in_cube.rotate_right_twice());
                //solve_cube(in_cube.rotate_left_twice());
                solve_cube(in_cube.rotate_front_twice());
                solve_cube(in_cube.rotate_down_twice());
            }
            Some('l') => {
                solve_cube(in_cube.rotate_back_clockwise());
                solve_cube(in_cube.rotate_back_counter_clockwise());
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                solve_cube(in_cube.rotate_front_clockwise());
                solve_cube(in_cube.rotate_front_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());

                solve_cube(in_cube.rotate_back_twice());
                solve_cube(in_cube.rotate_up_twice());
                solve_cube(in_cube.rotate_right_twice());
                //solve_cube(in_cube.rotate_left_twice());
                solve_cube(in_cube.rotate_front_twice());
                solve_cube(in_cube.rotate_down_twice());
            }
            Some('R') => {
                solve_cube(in_cube.rotate_back_clockwise());
                solve_cube(in_cube.rotate_back_counter_clockwise());
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                solve_cube(in_cube.rotate_front_clockwise());
                solve_cube(in_cube.rotate_front_counter_clockwise());
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());

                solve_cube(in_cube.rotate_back_twice());
                solve_cube(in_cube.rotate_up_twice());
                //solve_cube(in_cube.rotate_right_twice());
                solve_cube(in_cube.rotate_left_twice());
                solve_cube(in_cube.rotate_front_twice());
                solve_cube(in_cube.rotate_down_twice());
            }
            Some('r') => {
                solve_cube(in_cube.rotate_back_clockwise());
                solve_cube(in_cube.rotate_back_counter_clockwise());
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                solve_cube(in_cube.rotate_front_clockwise());
                solve_cube(in_cube.rotate_front_counter_clockwise());
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());

                solve_cube(in_cube.rotate_back_twice());
                solve_cube(in_cube.rotate_up_twice());
                //solve_cube(in_cube.rotate_right_twice());
                solve_cube(in_cube.rotate_left_twice());
                solve_cube(in_cube.rotate_front_twice());
                solve_cube(in_cube.rotate_down_twice());
            }
            Some('B') => {
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                solve_cube(in_cube.rotate_front_clockwise());
                solve_cube(in_cube.rotate_front_counter_clockwise());
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());

                //solve_cube(in_cube.rotate_back_twice());
                solve_cube(in_cube.rotate_up_twice());
                solve_cube(in_cube.rotate_right_twice());
                solve_cube(in_cube.rotate_left_twice());
                solve_cube(in_cube.rotate_front_twice());
                solve_cube(in_cube.rotate_down_twice());
            }
            Some('b') => {
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                solve_cube(in_cube.rotate_front_clockwise());
                solve_cube(in_cube.rotate_front_counter_clockwise());
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());

                //solve_cube(in_cube.rotate_back_twice());
                solve_cube(in_cube.rotate_up_twice());
                solve_cube(in_cube.rotate_right_twice());
                solve_cube(in_cube.rotate_left_twice());
                solve_cube(in_cube.rotate_front_twice());
                solve_cube(in_cube.rotate_down_twice());
            }
            Some('x') => {
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                solve_cube(in_cube.rotate_front_clockwise());
                solve_cube(in_cube.rotate_front_counter_clockwise());
                //X
                //X
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());
                solve_cube(in_cube.rotate_back_clockwise());
                solve_cube(in_cube.rotate_back_counter_clockwise());

                solve_cube(in_cube.rotate_back_twice());
                solve_cube(in_cube.rotate_up_twice());
                
                solve_cube(in_cube.rotate_front_twice());
                solve_cube(in_cube.rotate_down_twice());
            }
            Some('y') => {
                //Y
                solve_cube(in_cube.rotate_front_clockwise());
                solve_cube(in_cube.rotate_front_counter_clockwise());
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                //Y
                solve_cube(in_cube.rotate_back_clockwise());
                solve_cube(in_cube.rotate_back_counter_clockwise());

                solve_cube(in_cube.rotate_up_twice());
                solve_cube(in_cube.rotate_right_twice());
                solve_cube(in_cube.rotate_left_twice());
                solve_cube(in_cube.rotate_down_twice());
            }
            Some('z') => {
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                //Z
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());
                //Z

                solve_cube(in_cube.rotate_back_twice());
                solve_cube(in_cube.rotate_right_twice());
                solve_cube(in_cube.rotate_left_twice());
                solve_cube(in_cube.rotate_front_twice());
            }

            Some('g') => {
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                solve_cube(in_cube.rotate_front_clockwise());
                solve_cube(in_cube.rotate_front_counter_clockwise());
                //X
                //X
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());

                solve_cube(in_cube.rotate_up_twice());
                solve_cube(in_cube.rotate_right_twice());
                solve_cube(in_cube.rotate_left_twice());
                solve_cube(in_cube.rotate_front_twice());
                solve_cube(in_cube.rotate_down_twice());
            }
            Some('h') => {
                //Y
                solve_cube(in_cube.rotate_front_clockwise());
                solve_cube(in_cube.rotate_front_counter_clockwise());
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                //Y
                solve_cube(in_cube.rotate_back_clockwise());
                solve_cube(in_cube.rotate_back_counter_clockwise());
                solve_cube(in_cube.rotate_back_twice());
                solve_cube(in_cube.rotate_up_twice());
                solve_cube(in_cube.rotate_right_twice());
                solve_cube(in_cube.rotate_left_twice());
                solve_cube(in_cube.rotate_down_twice());
            }
            Some('j') => {
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                //Z
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());
                //Z
                solve_cube(in_cube.rotate_back_twice());
                solve_cube(in_cube.rotate_up_twice());
                solve_cube(in_cube.rotate_right_twice());
                solve_cube(in_cube.rotate_left_twice());
                solve_cube(in_cube.rotate_front_twice());
            }
            
            Some('k') => {
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                solve_cube(in_cube.rotate_front_clockwise());
                solve_cube(in_cube.rotate_front_counter_clockwise());
                //X
                //X
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());
                solve_cube(in_cube.rotate_back_clockwise());
                solve_cube(in_cube.rotate_back_counter_clockwise());

                solve_cube(in_cube.rotate_back_twice());
                solve_cube(in_cube.rotate_right_twice());
                solve_cube(in_cube.rotate_left_twice());
                solve_cube(in_cube.rotate_front_twice());
                solve_cube(in_cube.rotate_down_twice());
            }
            Some('t') => {
                //Y
                solve_cube(in_cube.rotate_front_clockwise());
                solve_cube(in_cube.rotate_front_counter_clockwise());
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                //Y
                solve_cube(in_cube.rotate_back_clockwise());
                solve_cube(in_cube.rotate_back_counter_clockwise());

                solve_cube(in_cube.rotate_back_twice());
                solve_cube(in_cube.rotate_up_twice());
                solve_cube(in_cube.rotate_right_twice());
                solve_cube(in_cube.rotate_front_twice());
                solve_cube(in_cube.rotate_down_twice());
            }
            Some('o') => {
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                //Z
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());
                //Z
                solve_cube(in_cube.rotate_back_twice());
                solve_cube(in_cube.rotate_up_twice());
                solve_cube(in_cube.rotate_left_twice());
                solve_cube(in_cube.rotate_front_twice());
                solve_cube(in_cube.rotate_down_twice());
            }
            _ => {
                solve_cube(in_cube.rotate_back_clockwise());
                solve_cube(in_cube.rotate_back_counter_clockwise());
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                solve_cube(in_cube.rotate_front_clockwise());
                solve_cube(in_cube.rotate_front_counter_clockwise());
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());
            }
        }
    }
}

fn build_cube() -> Cube {
    //Builds a solved cube
    Cube {
        sides: vec![
            build_side('R'),
            build_side('W'),
            build_side('B'),
            build_side('Y'),
            build_side('G'),
            build_side('O'),
        ],
        previous_moves: vec![],
        num_moves: 0,
        test: vec![],
    }
}
fn build_side(colour: char) -> Side {
    //Builds a side of all the same colour faces
    Side {
        faces: vec![colour; 9],
    }
}

#[test]
fn test_sides() {
    let side1 = build_side('W');
    let side2 = Side {
        faces: vec!['B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'W'],
    };
    assert_eq!(side1.is_solved(), true);
    assert_eq!(side2.is_solved(), false);
}
#[test]
fn test_cubes() {
    let cube1 = build_cube();
    assert_eq!(cube1.is_solved(), true);

    let cube1 = cube1.rotate_down_clockwise();
    assert_eq!(cube1.is_solved(), false);

    let mut cube2 = build_cube();
    cube2.sides[0].faces[0] = 'B';
    assert_eq!(cube2.is_solved(), false);

    let cube3 = build_cube() //rotate one way, and rotate back. Should result in no change.
        .rotate_front_clockwise()
        .rotate_front_counter_clockwise()
        .rotate_up_clockwise()
        .rotate_up_counter_clockwise()
        .rotate_right_clockwise()
        .rotate_right_counter_clockwise()
        .rotate_down_clockwise()
        .rotate_down_counter_clockwise()
        .rotate_left_clockwise()
        .rotate_left_counter_clockwise()
        .rotate_back_clockwise()
        .rotate_back_counter_clockwise();
    assert_eq!(cube3.is_solved(), true);
    assert_eq!(cube3.num_moves, 12);

    let cube4 = build_cube() //Four rotations in same direction should result in no change.
        .rotate_front_clockwise()
        .rotate_front_clockwise()
        .rotate_front_clockwise()
        .rotate_front_clockwise()
        .rotate_up_clockwise()
        .rotate_up_clockwise()
        .rotate_up_clockwise()
        .rotate_up_clockwise()
        .rotate_right_clockwise()
        .rotate_right_clockwise()
        .rotate_right_clockwise()
        .rotate_right_clockwise()
        .rotate_back_clockwise()
        .rotate_back_clockwise()
        .rotate_back_clockwise()
        .rotate_back_clockwise()
        .rotate_left_clockwise()
        .rotate_left_clockwise()
        .rotate_left_clockwise()
        .rotate_left_clockwise()
        .rotate_down_clockwise()
        .rotate_down_clockwise()
        .rotate_down_clockwise()
        .rotate_down_clockwise();
    assert_eq!(cube4.is_solved(), true);
    assert_eq!(cube4.num_moves, 24);
}
