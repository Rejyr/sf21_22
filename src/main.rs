use sf21_22::board::Board;

fn main() {
    for i in 3..=8 {
        let board = Board::new(i);
        println!("{}", board);
    }
}
