pub fn greet(name: &str) {
    println!("Hello from {}!", name);
}

pub fn validate_data(data: &str) -> bool {
    !data.is_empty()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
