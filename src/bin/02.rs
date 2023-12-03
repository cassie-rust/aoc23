advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .map(|l| {
            let mut parts = l.split(':');

            let id = parts
                .next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap();

            let cubes = parts.next().unwrap();
            let (blue, green, red) = foo(cubes);

            if blue > 14 || green > 13 || red > 12 {
                0
            } else {
                id
            }
        })
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let mut parts = l.split(':');
                let _ = parts.next();
                let cubes = parts.next().unwrap();

                let (blue, green, red) = foo(cubes);
                blue * green * red
            })
            .sum(),
    )
}

fn foo(input: &str) -> (u32, u32, u32) {
    let mut highest_blue = 0;
    let mut highest_red = 0;
    let mut highest_green = 0;
    input.split(';').map(|s| s.split(',')).for_each(|mut c| {
        while let Some(val) = c.next().map(|v| v.trim()) {
            let mut s = val.split_whitespace();
            let val = s.next().unwrap().parse::<u32>().unwrap();
            match s.next() {
                Some("blue") if val >= highest_blue => highest_blue = val,
                Some("red") if val >= highest_red => highest_red = val,
                Some("green") if val >= highest_green => highest_green = val,
                _ => (),
            }
        }
    });
    (highest_blue, highest_green, highest_red)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
