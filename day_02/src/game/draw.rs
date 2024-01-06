mod parser;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Draw {
    pub blue: u32,
    pub red: u32,
    pub green: u32,
}

pub const MAX_DRAW: Draw = Draw {
    blue: 14,
    red: 12,
    green: 13,
};

pub type Draws = Vec<Draw>;

impl Draw {
    pub fn parse(cubes: &str) -> Draws {
        parser::parse(cubes)
    }

    pub fn fits_into(&self, other: &Draw) -> bool {
        (self.blue <= other.blue) && (self.red <= other.red) && (self.green <= other.green)
    }

    pub fn power(&self) -> u32 {
        self.blue * self.red * self.green
    }
}

#[cfg(test)]
mod tests {
    use crate::game::draw::Draw;
    use googletest::prelude::*;
    use rstest::rstest;

    #[googletest::test]
    #[rstest]
    #[case(1, 2, 4)]
    #[case(0, 3, 2)]
    #[case(0, 0, 0)]
    fn test_new(#[case] blue: u32, #[case] red: u32, #[case] green: u32) {
        let draw = Draw { blue, red, green };
        expect_that!(draw.blue, eq(blue));
        expect_that!(draw.red, eq(red));
        expect_that!(draw.green, eq(green));
    }

    #[googletest::test]
    #[rstest]
    #[case(Draw{ blue: 0, red: 0, green: 0}, Draw{ blue: 1,red: 2, green: 3}, true)]
    #[case(Draw{ blue: 1, red: 2, green: 3}, Draw{ blue: 1,red: 2, green: 3}, true)]
    #[case(Draw{ blue: 1, red: 2, green: 3}, Draw{ blue: 2,red: 2, green: 3}, true)]
    #[case(Draw{ blue: 1, red: 2, green: 3}, Draw{ blue: 1,red: 3, green: 3}, true)]
    #[case(Draw{ blue: 1, red: 2, green: 3}, Draw{ blue: 1,red: 2, green: 4}, true)]
    #[case(Draw{ blue: 1, red: 2, green: 3}, Draw{ blue: 2,red: 2, green: 4}, true)]
    #[case(Draw{ blue: 1, red: 2, green: 3}, Draw{ blue: 0,red: 3, green: 4}, false)]
    #[case(Draw{ blue: 1, red: 2, green: 3}, Draw{ blue: 2,red: 1, green: 4}, false)]
    #[case(Draw{ blue: 1, red: 2, green: 3}, Draw{ blue: 2,red: 3, green: 2}, false)]
    fn test_fits_into(#[case] first: Draw, #[case] second: Draw, #[case] fits: bool) {
        expect_that!(first.fits_into(&second), eq(fits))
    }
}
