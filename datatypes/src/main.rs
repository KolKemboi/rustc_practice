fn main() {
    println!("Hello, world!");

    let mut x: u8 = 2;
    println!("{x}", );

    x = 1;
    println!("{x}", );

    let is_obs = true;
    println!("{is_obs}", );

    let mut letter: char = 'a';
    println!("{letter}", );

    letter = 'b';
    println!("{letter}", );

    let mut tup: (u8, u8, bool, char) = (1, 2, true, 's');
    tup.1 = 1;
    println!("{}", tup.1);

    let arr: [u8; 7] = [1, 2, 3, 4, 5, 6, 7];

    println!("{:?}", arr[5]);
}
