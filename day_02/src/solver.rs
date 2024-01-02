pub fn solve(_: &str) -> Option<()> {
    panic!("Not implemented")
}

#[cfg(test)]
mod tests {
    use crate::input::DATA;
    use crate::solver::*;

    #[test]
    #[should_panic(expected = "Not implemented")]
    fn test_input() {
        _ = solve(DATA)
    }
}
