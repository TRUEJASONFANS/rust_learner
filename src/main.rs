fn main() {
    println!("Hello, world!");
}

pub fn add(a:u32 , b: u32)->u32 {
    a+b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(add(2,3), 5);
    }
}