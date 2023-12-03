use std::{collections::HashMap, ops::Range};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let (value_map, gear_map) = map_grid(input, 140);

    let mut result = 0u32;

    for (k, _) in gear_map {
        for (i, v) in &value_map {
            if i.contains(k) {
                result += *v as u32;
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (value_map, gear_map) = map_grid(input, 140);

    let indices = gear_map
        .into_iter()
        .filter(|(_, c)| c == &'*')
        .map(|(idx, _)| idx)
        .collect::<Vec<_>>();

    let mut gear_adjancies: HashMap<usize, Vec<i32>> = HashMap::new();

    value_map.into_iter().for_each(|(coords, v)| {
        for i in &indices {
            if coords.contains(*i) {
                gear_adjancies
                    .entry(*i)
                    .and_modify(|e| e.push(v))
                    .or_insert(vec![v]);
            }
        }
    });

    Some(
        gear_adjancies
            .into_values()
            .map(|vs| {
                if vs.len() == 2 {
                    vs.into_iter().map(|v| v as u32).product::<u32>()
                } else {
                    0
                }
            })
            .sum(),
    )
}

fn map_grid(input: &str, width: usize) -> (HashMap<Coords, i32>, HashMap<usize, char>) {
    let input = pre_process(input, width as i32);

    let mut value_map = HashMap::new();
    let mut gear_map = HashMap::new();

    let mut value = String::new();
    for (idx, c) in input.chars().enumerate() {
        if c.is_ascii_digit() {
            value.push(c);
            continue;
        }

        if c != '.' {
            gear_map.insert(idx + 1, c);
        }

        if value.is_empty() {
            continue;
        }

        // value is present, we've stepped "past" it and need to record its
        // coordinates

        let current_row_start = idx - value.len();
        let current_row_end = idx + 2; // non-inclusive

        let prev_row = (current_row_start - (width + 2))..(current_row_end - width - 2);
        let next_row = (current_row_start + (width + 2))..(current_row_end + (width + 2));

        value_map.insert(
            Coords {
                prev_row,
                curr_row: current_row_start..current_row_end,
                next_row,
            },
            value.parse::<i32>().unwrap(),
        );
        value = String::new();
    }

    (value_map, gear_map)
}

/// Pads the input with `.` above, below, and to the sides
/// to avoid dealing with edge cases
fn pre_process(input: &str, width: i32) -> String {
    let mut s = String::new();
    for _ in 0..width + 1 {
        s.push('.');
    }

    for line in input.lines() {
        s.push('.');
        s.push_str(line);
        s.push('.');
    }

    for _ in 0..width + 1 {
        s.push('.');
    }

    s
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Coords {
    prev_row: Range<usize>,
    curr_row: Range<usize>,
    next_row: Range<usize>,
}

impl Coords {
    fn contains(&self, idx: usize) -> bool {
        self.prev_row.contains(&idx) || self.curr_row.contains(&idx) || self.next_row.contains(&idx)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_pre_process() {
        let input = ".1.";
        let expected = "......1......";
        assert_eq!(pre_process(input, 3), expected);
    }

    #[test]
    fn test_value_map() {
        let input = ".1.";

        let (actual, _) = map_grid(input, 3);
        let mut expected = HashMap::new();
        expected.insert(
            Coords {
                prev_row: 1..4,
                curr_row: 6..9,
                next_row: 11..14,
            },
            1,
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gear_map() {
        let input = ".1*";

        let (_, actual) = map_grid(input, 3);
        let mut expected = HashMap::new();
        expected.insert(8, '*');

        assert_eq!(actual, expected);
    }

    #[test]
    #[cfg(disable)]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    #[cfg(disable)]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
