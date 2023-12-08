use std::{array, cmp::Reverse, iter::repeat};

use num::FromPrimitive;
use num_derive::FromPrimitive;
use variant_count::VariantCount;

// allow to sort on hand type (for example five of a kind is best)
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

/*
 * VariantCount is just used to get the amount of types of cards (14)
 * for use in counting cards into an array of size (num of types of card)
 * where index = card as usize,
 * and value is the number of times the card appears in a hand.
 *
 * from primitive allows me to cast to a card instead of writing out 2-9 lol.
 *
 * others are for use in sorting on and copying cards
 */
#[derive(Copy, Clone, FromPrimitive, PartialEq, Eq, PartialOrd, Ord, VariantCount /*, Hash */)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

type Hand = [Card; 5];

struct HandAndBid(Hand, i32);

impl From<u8> for Card {
    // panics if value is not a valid card
    // uses J as Jack
    fn from(value: u8) -> Self {
        use Card::*;
        match value {
            b'A' => Ace,
            b'K' => King,
            b'Q' => Queen,
            b'J' => Jack,
            b'T' => Ten,
            digit => {
                assert!(digit >= b'2' && digit <= b'9');
                Card::from_u8(digit - b'1').unwrap() // assuming Two = 1
            }
        }
    }
}

type CardCounts = [i32; Card::VARIANT_COUNT];

trait HandExt {
    // both panic if the value is not a valid hand
    // ex of a valid hand: "46645"
    fn from_part1(value: &str) -> Self;
    fn from_part2(value: &str) -> Self;

    fn count_hand(&self) -> CardCounts;
    fn get_hand_type(&self) -> HandType;
}

impl HandExt for Hand {
    fn from_part1(value: &str) -> Self {
        let cards_as_u8: [u8; 5] = value.as_bytes().try_into().unwrap();
        array::from_fn(|i| cards_as_u8[i].into())
    }

    // in part 2, Js are Jokers instead of Jacks
    fn from_part2(value: &str) -> Self {
        let cards_as_u8: [u8; 5] = value.as_bytes().try_into().unwrap();
        array::from_fn(|i| match cards_as_u8[i] {
            b'J' => Card::Joker,
            card => card.into(),
        })
    }

    fn count_hand(&self) -> CardCounts {
        // sets all the values to 0 at start
        let mut result: CardCounts = Default::default();

        for &card in self.iter() {
            result[card as usize] += 1;
        }

        result
    }

    fn get_hand_type(&self) -> HandType {
        use HandType::*;

        let mut counts_map: CardCounts = self.count_hand();

        let joker_amt = counts_map[Card::Joker as usize];
        if joker_amt == 5 {
            return FiveOfAKind;
        }

        // if hand contains a joker, set it to the most common non joker
        if joker_amt != 0 {
            let most_common_non_joker = counts_map
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != Card::Joker as usize)
                .max_by(|(_, count1), (_, count2)| count1.cmp(count2))
                .unwrap();

            counts_map[most_common_non_joker.0] += counts_map[Card::Joker as usize];
            counts_map[Card::Joker as usize] = 0;
        }

        // ex: AAQ2A => [3, 1, 1, 0, 0]
        // place the card amounts in an array on the stack then fill excess spots with 0
        // iterator is infinite so unwrap won't panic :)
        let mut counts: [i32; 5] =
            array_init::from_iter(counts_map.into_iter().filter(|x| *x != 0).chain(repeat(0)))
                .unwrap();

        // sort the card amounts from highest to lowest
        counts.sort_unstable_by_key(|&x| Reverse(x));

        // get the hand type from the counts
        match counts[0] {
            5 => FiveOfAKind,
            4 => FourOfAKind,
            3 => {
                if counts[1] == 2 {
                    FullHouse
                } else {
                    ThreeOfAKind
                }
            }
            2 => {
                if counts[1] == 2 {
                    TwoPair
                } else {
                    OnePair
                }
            }
            1 => HighCard,
            _ => unreachable!(),
        }
    }
}

impl From<&str> for HandAndBid {
    // converts one line of input into a hand and bid pair
    // ex: "3J399 27" => HandAndBid([Three, Jack, Three, Nine, Nine], 27)
    fn from(line: &str) -> Self {
        let (cards, bid_str) = line.split_once(' ').unwrap();
        HandAndBid(Hand::from_part1(cards), bid_str.parse().unwrap())
    }
}

impl HandAndBid {
    // use the part2 conversion instead (J becomes Joker, not Jack)
    // ex: "3J399 27" => HandAndBid([Three, Joker, Three, Nine, Nine], 27)
    fn from_part2(line: &str) -> Self {
        let (cards, bid_str) = line.split_once(' ').unwrap();
        HandAndBid(Hand::from_part2(cards), bid_str.parse().unwrap())
    }
}

fn parse_input_part1(puzzle_input: &str) -> Vec<HandAndBid> {
    puzzle_input.lines().map(HandAndBid::from).collect()
}

// use the part2 conversion instead (J becomes Joker, not Jack)
fn parse_input_part2(puzzle_input: &str) -> Vec<HandAndBid> {
    puzzle_input.lines().map(HandAndBid::from_part2).collect()
}

fn total_winnings(mut hands_and_bids: Vec<HandAndBid>) -> i32 {
    // sort by cards first, then sort by hand type while preserving the order from cards
    hands_and_bids.sort_unstable_by_key(|hab| hab.0);
    hands_and_bids.sort_by_key(|hab| hab.0.get_hand_type());

    // sum the bids (hab.1) multiplied by rank (i + 1)
    hands_and_bids
        .iter()
        .enumerate()
        .map(|(i, hab)| ((i + 1) as i32) * hab.1)
        .sum()
}

fn part1(puzzle_input: &str) -> i32 {
    total_winnings(parse_input_part1(puzzle_input))
}

fn part2(puzzle_input: &str) -> i32 {
    total_winnings(parse_input_part2(puzzle_input))
}

fn main() {
    let file_contents = std::fs::read("input.txt").unwrap();
    let puzzle_input = std::str::from_utf8(&file_contents).unwrap();

    println!("{}", part1(puzzle_input));
    println!("{}", part2(puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "32T3K 765\n\
                              T55J5 684\n\
                              KK677 28\n\
                              KTJJT 220\n\
                              QQQJA 483";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 5905);
    }
}
