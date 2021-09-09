use std::fs::read_to_string;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn part1(input: &str) {
    let (mut two_total, mut three_total) = (0, 0);
    for value in input.lines() {
        let mut occurences = [0; 256];
        value.chars().for_each(|c| occurences[c as usize] += 1);

        if occurences.iter().any(|&count| count == 2) {
            two_total += 1;
        }
        if occurences.iter().any(|&count| count == 3) {
            three_total += 1;
        }
    }

    assert_eq!(two_total * three_total, 5368);
    println!("Part1: {}", two_total * three_total);
}

fn part2(input: &str) {
    for value in input.lines() {
        for value2 in input.lines() {
            if count_differences(value, value2) == 1 {
                let common = common_letters(value, value2);
                assert_eq!(common, "cvgywxqubnuaefmsljdrpfzyi".to_string());
                println!("Part2: {}", common);
                return;
            }
        }
    }
}

fn count_differences(str1: &str, str2: &str) -> usize {
    str1.chars()
        .zip(str2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}

fn common_letters(str1: &str, str2: &str) -> String {
    str1.chars()
        .zip(str2.chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|(c1, _)| c1)
        .collect()
}

fn main() -> Result<()> {
    let input = read_to_string("input.txt").expect("Missing input file");

    part1(&input);
    part2(&input);

    Ok(())
}
