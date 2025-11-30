fn process_p1() -> u8 {
    0
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_works() {
        assert_eq!(process_p1(), 0);
    }
}
