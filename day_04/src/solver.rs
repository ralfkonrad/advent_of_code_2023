use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    own_numbers: Vec<u32>,
}

impl Card {
    fn parse(line: &str) -> Self {
        let (card, numbers) = line.split_once(':').expect("Each line should contain ':'");
        let id = card
            .split_whitespace()
            .last()
            .expect("Card prefix should contain an ID")
            .parse::<u32>()
            .expect("Card ID should parse to a u32");
        let (winning_numbers, own_numbers) = numbers
            .split_once('|')
            .expect("Card should have winning and own numbers separated by '|'");

        Self {
            id,
            winning_numbers: winning_numbers
                .split_whitespace()
                .map(|x| x.parse().expect("Winning number should parse to u32"))
                .collect(),
            own_numbers: own_numbers
                .split_whitespace()
                .map(|x| x.parse().expect("Own number should parse to u32"))
                .collect(),
        }
    }

    fn get_matching_numbers_count(&self) -> usize {
        let winning_numbers = self.winning_numbers.iter().copied().collect::<HashSet<_>>();

        self.own_numbers
            .iter()
            .filter(|number| winning_numbers.contains(number))
            .count()
    }

    fn get_points(&self) -> u32 {
        let matches = self.get_matching_numbers_count();
        if matches == 0 {
            return 0;
        }

        2_u32.pow((matches as u32) - 1)
    }
}

pub fn solve(input: &str) -> (u32, u32) {
    let cards = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(Card::parse)
        .collect::<Vec<_>>();

    let result1 = cards.iter().map(Card::get_points).sum::<u32>();

    let mut copies = vec![1_u32; cards.len()];
    for card_index in 0..cards.len() {
        let count = cards[card_index].get_matching_numbers_count();
        let own_copies = copies[card_index];

        for next_index in (card_index + 1)..(card_index + 1 + count).min(cards.len()) {
            copies[next_index] += own_copies;
        }
    }

    let result2 = copies.iter().sum::<u32>();

    (result1, result2)
}

#[cfg(test)]
mod tests {
    use crate::input::tests::TEST_DATA;
    use crate::input::tests::TEST_DATA_RESULT;
    use crate::input::tests::TEST_DATA_RESULT_2;
    use crate::solver::Card;
    use googletest::prelude::*;

    #[googletest::test]
    fn test_card_parse() {
        let card = Card::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");

        expect_that!(card.id, eq(1));
        expect_that!(card.winning_numbers.as_slice(), eq(&[41, 48, 83, 86, 17]));
        expect_that!(
            card.own_numbers.as_slice(),
            eq(&[83, 86, 6, 31, 17, 9, 48, 53])
        );
    }

    #[googletest::test]
    fn test_points() {
        let card = Card::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        expect_that!(card.get_points(), eq(8));

        let card = Card::parse("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36");
        expect_that!(card.get_points(), eq(0));
    }

    #[googletest::test]
    fn test_solve() {
        expect_that!(
            super::solve(TEST_DATA),
            eq((TEST_DATA_RESULT, TEST_DATA_RESULT_2))
        )
    }
}
