fn get_input() -> &'static str {
    return "199
200
208
210
200
207
240
269
260
263";
}

fn main() {
    let lines: Vec<usize> = get_input()
        .lines()
        .map(|line| {
            let parsed: usize = line.parse().expect("something bad happend");

            return parsed;
        })
        .collect();

    let mut curr: [&usize; 1] = [&lines[1]];
    let mut prev: [&usize; 1] = [&lines[0]];
    let mut idx = 1;
    let mut count: usize = 0;
    println!("lines: {:?}", lines);

    for num in lines.iter().skip(2) {
        println!("curr: {}, prev: {}", curr[0], prev[0]);
        if curr[0] > prev[0] {
            count += 1;
        }

        prev[0] = curr[idx % 1];
        curr[0] = num;

        idx += 1;
    }
    println!("count is: {}", count);
}
