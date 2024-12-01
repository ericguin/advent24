fn part1(file: &str) {
    let contents = std::fs::read_to_string(file).expect("Could not load file!!");
    let lines = contents.split("\n");

    let mut gp1 : std::vec::Vec<i32> = std::vec::Vec::new();
    let mut gp2 : std::vec::Vec<i32> = std::vec::Vec::new();

    for line in lines {
        let mut parts = line.split("   ");
        let first = parts.next().unwrap();
        let second = parts.next().unwrap();
        let numf: i32 = first.parse().unwrap();
        let numl: i32 = second.parse().unwrap();
        gp1.push(numf);
        gp2.push(numl);
        println!("Uhh? {numf}, {numl}");
    }

    gp1.sort();
    gp2.sort();

    let mut diff: i32 = 0;
    for pair in std::iter::zip(gp1, gp2) {
        diff += (pair.0 - pair.1).abs();
    }

    println!("Answer: {diff}");
}

fn part2(file: &str) {
    let contents = std::fs::read_to_string(file).expect("Could not load file!!");
    let lines = contents.split("\n");

    let mut gp1 : std::vec::Vec<i32> = std::vec::Vec::new();
    let mut gp2 : std::vec::Vec<i32> = std::vec::Vec::new();

    for line in lines {
        let mut parts = line.split("   ");
        let first = parts.next().unwrap();
        let second = parts.next().unwrap();
        let numf: i32 = first.parse().unwrap();
        let numl: i32 = second.parse().unwrap();
        gp1.push(numf);
        gp2.push(numl);
        println!("Uhh? {numf}, {numl}");
    }

    let mut score: i32 = 0;
    
    for item in gp1 {
        let cnt : i32 = gp2.iter().filter(|&o| *o == item).count().try_into().unwrap();
        score += item * cnt;
    }

    println!("Answer: {score}");
}

fn main() {
    //part1("src/bin/day01/input.txt");
    part2("src/bin/day01/input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        part1("src/bin/day01/ex01.txt");
    }

    #[test]
    fn ex2() {
        part2("src/bin/day01/ex01.txt");
    }
}