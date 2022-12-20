use position::Position;
use solver::Solver;

mod position;
mod solver;
mod transposition_table;
mod move_sorter;
mod opening_book;

fn main() {
    let mut position = Position::new(); 
    let mut solver = Solver::new();

    let seq = std::env::args().nth(1).expect("no seq"); 

    position.play_seq(&seq); 
    solver.load_book(".book"); 

    println!("{:?}", solver.analyze(&position, true)); 
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Write;
    use std::time::Instant;
    use std::fs;
    use std::io;

    fn foo(bench_path: &str) {
        let f = fs::read_to_string(bench_path).expect("Failed to open benchmark file");
        let mut s = Solver::new();
        s.load_book("7x6.book"); 
    
        for i in f.lines() {
            let mut p = Position::new();
            let mut split = i.split(" ");
            let seq = split.next().unwrap();
            let exp = split.next().unwrap().parse::<i32>().unwrap();
            
            p.play_seq(seq); 
            assert_eq!(exp, s.solve(&p, true)); 
        }
    }
    
    #[test]
    fn test_L3_R1() {
        foo("benchmarks/Test_L3_R1")
    }
    
    #[test]
    fn test_L2_R1() {
        foo("benchmarks/Test_L2_R1")
    }
    
    #[test]
    fn test_L2_R2() {
        foo("benchmarks/Test_L2_R2")
    }
    
    #[test]
    fn test_L1_R1() {
        foo("benchmarks/Test_L1_R1")
    }
    
    #[test]
    fn test_L1_R2() {
        foo("benchmarks/Test_L1_R2")
    }
    
    #[test]
    fn test_L1_R3() {
        foo("benchmarks/Test_L1_R3")
    }
}

