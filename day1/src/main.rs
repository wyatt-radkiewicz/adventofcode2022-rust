fn main() {
    let input = std::fs::read_to_string("day1.txt").unwrap();
    let mut max = input.split_terminator("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<usize>().unwrap()).sum())
        .collect::<Vec<usize>>();
    max.sort_by(|a, b| b.cmp(a));
    
    println!("{:?}", &max[0..3].iter().sum::<usize>());
}
