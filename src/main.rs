//INFO
//When game tells you previous moves last number is player that moved there
//Please only input integers between 0 and 126
//any number not in range will be set to range boundary
//Game is played on cubic grid with sidelength of 3 so along each axis you can place your go at 0, 1 or 2.

//Struct containing all important game info
fn clamp(x:i8, min:i8, max:i8) -> i8{
    if x < min{return min}
    if x > max{return max}
    x
}

fn vector_subtract(x: Vec<i8>, y: Vec<i8>) -> Vec<i8>{
    let mut new_vec: Vec<i8> = Vec::new();
    for i in 0..x.len(){
        new_vec.push(x[i]-y[i]);
    }
    new_vec
}

fn multi_to_single(coords: Vec<i8>) -> i32{
    let mut slot: i32 = 0;
    for i in 0..coords.len(){ slot += (3 as i32).pow(i as u32) * coords[i] as i32; }
    slot
}

fn single_to_multi(slot: i32, size: i8) -> Vec<i8>{
    let mut coords: Vec<i8> = Vec::new();
    for _i in 0..size{coords.push(0)}

    let mut s = slot;

    for i in 0..size{
        let index = size-i-1;
        let v = s as f32 / ((3 as i32).pow((index) as u32)) as f32;
        coords[index as usize] = v.trunc() as i8;
        s = s - ((3 as i32).pow((size-i-1) as u32)) as i32 * v.trunc() as i32
    }

    coords
}

struct MdncGame{
    grid: Vec<i8>,
    num_dimensions: i8,
    num_players: i8
}

//Functions implementations
impl MdncGame{
    fn generate_grid(&self) -> MdncGame{

        let mut new_grid: Vec<i8> = Vec::new();
        let max_size: i8 = 3;

        for _i in 0..max_size.pow(self.num_dimensions as u32){
            new_grid.push(0);
        }

        MdncGame{grid: new_grid, num_dimensions: self.num_dimensions, num_players: self.num_players}
    }

   fn check_for_line(&self, slot: i32, player: i8) -> bool{
        let max = (3 as i32).pow(self.num_dimensions as u32);
        for i in 1..max{
            if slot - i >= 0 && slot + i < max {
                if self.grid[(slot - i) as usize] == player && self.grid[slot as usize] == player && self.grid[(slot+i) as usize] == player{
                    let pos1 = single_to_multi(slot - i, self.num_dimensions);
                    let pos2 = single_to_multi(slot, self.num_dimensions);
                    let pos22 = single_to_multi(slot, self.num_dimensions);
                    let pos3 = single_to_multi(slot + i, self.num_dimensions);
                    if vector_subtract(pos1, pos2) == vector_subtract(pos22, pos3){return true}
                }
            }
            // Check out this bit cause its not walways working
            if slot - 2 * i >= 0{
                if self.grid[(slot - 2 * i) as usize] == player && self.grid[(slot - i) as usize] == player && self.grid[slot as usize] == player{
                    let pos1 = single_to_multi(slot-2*i, self.num_dimensions);
                    let pos2 = single_to_multi(slot - i, self.num_dimensions);
                    let pos22 = single_to_multi(slot - i, self.num_dimensions);
                    let pos3 = single_to_multi(slot, self.num_dimensions);
                    if vector_subtract(pos1, pos2) == vector_subtract(pos22, pos3){return true}
                }
            }
            if slot + 2 * i < max{
                if self.grid[slot as usize] == player && self.grid[(slot + i) as usize] == player && self.grid[(slot + 2 * i) as usize] == player{
                    let pos1 = single_to_multi(slot, self.num_dimensions);
                    let pos2 = single_to_multi(slot + i, self.num_dimensions);
                    let pos22 = single_to_multi(slot + i, self.num_dimensions);
                    let pos3 = single_to_multi(slot + 2 * i, self.num_dimensions);
                    if vector_subtract(pos1, pos2) == vector_subtract(pos22, pos3){return true}
                }
            }
            
        }
        false
   }
}

fn main() {
    use std::io::{self, Write};
    println!("Hi, Welcome to Multi-Dimensional Noughts and Crosses!");

    let mut nplayers: i8;
    print!("How many people want to play? ");
    io::stdout().flush().unwrap();
    let mut string = String::new();
    let _z = std::io::stdin().read_line(&mut string);
    nplayers = string.trim().parse::<i8>().expect("Please input only digits");
    
    let mut ndimensions: i8;
    print!("How many dimensions do you want to enable? (must be int) ");
    io::stdout().flush().unwrap();
    let mut string = String::new();
    let _z = std::io::stdin().read_line(&mut string);
    ndimensions = string.trim().parse::<i8>().expect("Please input number below 127");

    ndimensions = clamp(ndimensions, 1, 127);
    nplayers = clamp(nplayers, 1, 127);

    let mut game = MdncGame{
        grid: Vec::new(),
        num_dimensions: ndimensions,
        num_players: nplayers
    };
    game = game.generate_grid();

    let mut previous_moves:Vec<Vec<i8>> = Vec::new();
    loop{
        for i in 1..=game.num_players{
            println!();
            println!("Previous moves:{:?}",previous_moves);
            let mut new_coords = Vec::new();
            let mut new_coords2 = Vec::new();
            let mut new_coords3 = Vec::new();
            for i2 in 0..game.num_dimensions{
                print!("Player {:?}, where would you like move on axis {:?}? ", i, i2);
                io::stdout().flush().unwrap();
                let mut ans = String::new();
                let _y = io::stdin().read_line(&mut ans);
                new_coords.push(clamp(ans.trim().parse::<i8>().expect("Please input number below 127"),0,2));
                new_coords2.push(clamp(ans.trim().parse::<i8>().expect("Please input number below 127"),0,2));
                new_coords3.push(clamp(ans.trim().parse::<i8>().expect("Please input number below 127"),0,2));
            }
            game.grid[(multi_to_single(new_coords)) as usize] = i;
            new_coords3.push(i);
            previous_moves.push(new_coords3);
            if game.check_for_line(multi_to_single(new_coords2), i){
                println!("Player {:?} wins!",i);
                println!("Grid: {:?}", game.grid);
                std::process::exit(0);
             }
        }
    }
}