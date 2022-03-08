fn main() {
    let mut product = 1;
    let array = [1, 2, 3, 4, 5];

    for x in array {
        product = product * x;
    }

    let mut prod_array: [i32; 5] = [0; 5];
    let mut y = 0;
    while y < prod_array.len() {
        prod_array[y] = product / array[y];
        y = y + 1;
    }

    for z in prod_array {
        println!("{}", z);
    }
}
