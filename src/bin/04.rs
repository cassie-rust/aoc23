use std::collections::{HashMap, HashSet};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(parse_line)
            .map(|(_, w, m)| w.intersection(&m).count())
            .map(|v| {
                if v > 0 {
                    2u32.pow((v - 1).try_into().unwrap())
                } else {
                    0
                }
            })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards: HashMap<u32, u32> = HashMap::new();
    let total_cards = input.lines().count() as u32;

    input
        .lines()
        .map(parse_line)
        .map(|(card, w, m)| (card, w.intersection(&m).count()))
        .for_each(|(card, win_count)| {
            let card: u32 = card.try_into().unwrap();
            let win_count: u32 = win_count.try_into().unwrap();

            for i in (card + 1)..(card + win_count + 1) {
                let mut copies = 0;

                copies += cards.get(&i).unwrap_or(&0);
                copies += 1;
                copies += cards.get(&card).unwrap_or(&0);
                cards.insert(i, copies);
            }
        });

    Some(cards.values().sum::<u32>() + total_cards)
}

fn parse_line(line: &str) -> (i32, HashSet<u32>, HashSet<u32>) {
    let (game, rest) = line.split_once(':').unwrap();
    let id = game
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let (winning_nos, my_nos) = rest.split_once('|').unwrap();

    let winning_nos = to_hashset(winning_nos);
    let my_nos = to_hashset(my_nos);

    (id, winning_nos, my_nos)
}

fn to_hashset(input: &str) -> HashSet<u32> {
    input
        .split_whitespace()
        .map(|v| v.parse::<u32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_pow() {
        // just gotta make sure I math correctly orz
        let v = 4;
        let expected = 8;
        let actual = 2i32.pow(v - 1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_line() {
        let input = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let (card_id, winning_nos, my_nos) = parse_line(input);

        let expected_winning_nos: HashSet<u32> = vec![31, 18, 13, 56, 72].into_iter().collect();
        let expected_my_nos: HashSet<u32> =
            vec![74, 77, 10, 23, 35, 67, 36, 11].into_iter().collect();
        assert_eq!(card_id, 6);
        assert_eq!(winning_nos, expected_winning_nos);
        assert_eq!(my_nos, expected_my_nos);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
