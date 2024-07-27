
fn main() {

    let _x = 21;
    let _y = 221;

    println!("Hello, world!");
    println!("x + y = {}", _x + _y );

    let arr = [1, 2, 3];
    let _slice = &arr[1 .. 2];

    let res = is_even(32);

    print!("{}", res);
}

pub fn is_even(num : u32) -> bool {
    let digit = num % 2;
    digit == 0
}
