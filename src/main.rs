//Struct containing all important game info
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
        for i in 0..max{
            if slot - i >= 0 && slot + i < max {
                if self.grid[(slot - i) as usize] == player && self.grid[slot as usize] == player && self.grid[(slot+i) as usize] == player{
                    let pos1 = single_to_multi(slot - i, self.num_dimensions);
                    let pos2 = single_to_multi(slot, self.num_dimensions);
                    let pos22 = single_to_multi(slot, self.num_dimensions);
                    let pos3 = single_to_multi(slot + i, self.num_dimensions);
                    if vector_subtract(pos1, pos2) == vector_subtract(pos22, pos3){return true}
                }
            }
            if slot - 2 * i >= 0{
                if self.grid[(slot - 2 * i) as usize] == player && self.grid[(slot - i) as usize] == player && self.grid[slot as usize] == player{
                    let pos1 = single_to_multi(slot - 2 * i, self.num_dimensions);
                    let pos2 = single_to_multi(- i, self.num_dimensions);
                    let pos22 = single_to_multi(- i, self.num_dimensions);
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
        return false
   }
}

fn main() {
    println!("Hi, Welcome to Multi-Dimensional Noughts and Crosses!");

    let mut game = MdncGame{
        grid: Vec::new(),
        num_dimensions: 2,
        num_players: 1
    };
    game = game.generate_grid();
}