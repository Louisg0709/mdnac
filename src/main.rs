//Struct containing all important game info
fn multi_to_single(coords: Vec<i8>) -> i32{
    let mut slot: i32 = 0;
    for i in 0..coords.len(){ slot += (3 as i32).pow(i as u32) * coords[i] as i32; }
    slot
}

fn single_to_multi(slot: i32) -> Vec<i8>{
    Vec::new()
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

    fn check_for_line(&self, slot: i32){}
}

fn main() {
    let mut game = MdncGame{
        grid: Vec::new(),
        num_dimensions: 4,
        num_players: 1
    };

    game = game.generate_grid();
    println!("Grid: {:?}", game.grid);
}