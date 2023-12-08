use std::{cmp::Reverse, collections::HashMap};

use array_init::array_init;

use num_derive::FromPrimitive;

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

// from primitive allows me to cast to a card instead of writing out 2-9 lol
// others are for use in hashmap, sorting, copying on cards
#[derive(Copy, Clone, FromPrimitive, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

#[derive(Copy, Clone)]
struct HandAndBid(Hand, i32);

impl From<u8> for Card {
    // panics if value is not a valid card
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
                num::FromPrimitive::from_u8(digit - b'1').unwrap() // assuming Two = 1
            }
        }
    }
}

trait HandExt {
    fn from_part1(value: &str) -> Self;
    fn from_part2(value: &str) -> Self;

    fn count_hand(&self) -> HashMap<Card, i32>;
    fn get_hand_type(&self) -> HandType;
}

impl HandExt for Hand {
    fn from_part1(value: &str) -> Self {
        let cards_as_u8: [u8; 5] = value.as_bytes().try_into().unwrap();
        array_init(|i| cards_as_u8[i].into())
    }

    // in part 2, Js are Jokers instead of Jacks
    fn from_part2(value: &str) -> Self {
        let cards_as_u8: [u8; 5] = value.as_bytes().try_into().unwrap();
        array_init(|i| match cards_as_u8[i] {
            b'J' => Card::Joker,
            card => card.into(),
        })
    }

    fn count_hand(&self) -> HashMap<Card, i32> {
        let mut result = HashMap::new();

        for &card in self.iter() {
            result.entry(card).and_modify(|val| *val += 1).or_insert(1);
        }

        result
    }

    fn get_hand_type(&self) -> HandType {
        use HandType::*;

        let mut counts_map = self.count_hand();

        // if hand contains a joker
        if counts_map.contains_key(&Card::Joker) {
            // if only contains joker, then five of a kind
            if counts_map.remove(&Card::Joker).unwrap() == 5 {
                return FiveOfAKind;
            }

            // otherwise, get the best hand type when joker is changed to a diff card in the hand
            // assumption: all Jokers should change to the same card for the best hand
            return counts_map
                .keys()
                .map(|&other_card| {
                    let hand_with_other_card: Hand = array_init(|i| match self[i] {
                        Card::Joker => other_card,
                        card => card,
                    });
                    hand_with_other_card.get_hand_type()
                })
                .max()
                .unwrap();
        }

        // sort the card amounts from highest to lowest
        let mut counts = counts_map.values().copied().collect::<Vec<_>>();
        counts.sort_unstable_by_key(|&x| Reverse(x));

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
            _ => HighCard,
        }
    }
}

impl From<&str> for HandAndBid {
    // converts one line of input into a hand and bid pair
    // ex: "3A399 27" => HandAndBid([Three, Ace, Three, Nine, Nine], 27)
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

    // sum the bid (hab.1) multiplied by rank (i + 1)
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
