pub fn tower_of_hanoi(n: u32, a: &str, b: &str, c: &str) {
    if n == 1 {
        println!("({a}) -> ({c})");
        return;
    }
    tower_of_hanoi(n - 1, a, c, b);
    tower_of_hanoi(1, a, b, c);
    tower_of_hanoi(n - 1, b, a, c);
}
