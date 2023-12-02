fn day1(input: &str) -> i32 {
    input.lines()
        .map(|l| {
            let mut parts = l.split(':');
            
            let id = parts.next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            
            let cubes = parts.next().unwrap();
            let (blue, green, red) = foo(cubes);
            
            if blue > 14 || green > 13 || red > 12 {
                0
            } else {
                id
            }
        })
        .sum()
}

fn day2(input: &str) -> i32 {
    input.lines()
        .map(|l| {
            let mut parts = l.split(':');
            let _ = parts.next();
            let cubes = parts.next().unwrap();
            
            let (blue, green, red) = foo(cubes);
            blue * green * red
        })
        .sum()
}

fn main() {
   println!("{}", day2(P1));
}


fn foo(input: &str) -> (i32, i32, i32) {
    let mut highest_blue = 0;
    let mut highest_red = 0;
    let mut highest_green = 0;
    input.split(';')
        .map(|s| s.split(','))
        .for_each(|mut c| {
            while let Some(val) = c.next().map(|v| v.trim()) {
                let mut s = val.split_whitespace();
                let val = s.next().unwrap().parse::<i32>().unwrap();
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

const P1: &'static str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
