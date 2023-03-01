fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    // TODO
    let min = input.iter().min();
    let max = input.iter().max();
    println!("{} is largest and {} is smallest", max.unwrap(), min.unwrap());
}
