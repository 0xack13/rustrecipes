// using trait with struct
struct Player {
    first_name: String,
    last_name: String,
}

trait FullName {
    fn full_name(&self) -> String;
}

impl FullName for Player {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn main() {
    let player = Player {
        first_name: "Muhammed".to_string(),
        last_name: "Ali".to_string(),
    };

    println!("Player: {}", player.full_name());
}