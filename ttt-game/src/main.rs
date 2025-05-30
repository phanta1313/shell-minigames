pub mod utils;

use std::io;
use utils::{cls, all_equal};



fn get_dimension() -> Result<u8, Box<dyn std::error::Error>>{
    let valids = (3, 6, 9);

    let mut input = String::new();
    println!("Enter the dimension of field: (3,6,9)");
    io::stdin().read_line(&mut input)?;

    let d: u8 = input.trim().parse()?;
    
    
    if !(valids.0 == d || valids.1 == d || valids.2 == d) {
        return Err("Invalid dimension".into())
    }

    return Ok(d);
}


fn create_field(d: u8) -> Vec<Vec<char>> {
    vec![vec!['-'; d as usize]; d as usize]
}


fn render_field(field: &Vec<Vec<char>>){
    for i in 0..field.len(){
        for j in 0..field.len(){
            print!("{} ", field[i][j]);
        }
        println!();
    }
}


fn set_val(field: &mut Vec<Vec<char>>, val: char) -> Result<(usize, usize), Box<dyn std::error::Error>>{
    let mut input = String::new();
    println!("Enter coords for {}: ", val);
    io::stdin().read_line(&mut input).unwrap();

    let coords = input.trim().chars().collect::<Vec<char>>();   

    if coords.len() != 2 {
        println!("Set valid coords");
    }

    let x: usize = coords[0].to_string().parse()?;
    let y: usize = coords[1].to_string().parse()?;

    println!("{}", field[x][y]);
    field[x][y] = val;

    Ok((x, y))
}


fn check_state(field: &Vec<Vec<char>>, dim: usize) -> Option<char>{    
    for i in 0..dim{
        if field[i][0] == '-'{
            continue;
        }
        if all_equal(&field[i]) {
            return Some(field[i][0]);
        }
    }
    //todo: check colums and 2 diagonals also
    None
}




fn main() {
    let dim = loop {
            match get_dimension() {
            Ok(val) => break val,
            Err(_) => println!("Invalid dimension. ")
        }
    };

    println!("Choosed dimension - {}", dim);

    let mut field = create_field(dim);

    render_field(&field);

    let mut used_coords: Vec<(usize, usize)> = vec!();
    let mut curr_val = 'O';

    //GAME LOOP
    loop {
        loop {
            match set_val(&mut field, curr_val){
                Ok((x, y)) => {
                    cls();
                    render_field(&field);
                    if used_coords.contains(&(x, y)){
                        println!("Val is already here.");
                    } else {
                        used_coords.push((x, y));
                        break;
                    }
                },
                Err(_) => println!("Invalid coords."),
            };
        }
        
        if curr_val == 'O'{
            curr_val = 'X';
        } else {
            curr_val = 'O';
        }

        let cs = check_state(&field, dim.into());
        if cs != None {
            println!("{} - won.", cs.unwrap());
            break;
        }
        cls();
        render_field(&field);
    };
}