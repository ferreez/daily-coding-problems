fn main() {
    let k = 17;
    let array = [10, 15, 3, 7, 4, 19];

    if unsorted_nested_loop(k, &array) {
        println!("True!");
    } else {
        println!("False!)");
    }
    if sort_single_pass(k, &array) {
        println!("True!");
    } else {
        println!("False!");
    }
}

fn unsorted_nested_loop(k: i32, array: &[i32]) -> bool {
    let mut x = 0;
    while x < array.len() {
        for y in &array[x + 1..] {
            println!("x:{} y:{}", array[x], y);
            if array[x] + y == k {
                return true;
            }
        }
        x = x + 1;
    }
    false
}

fn sort_single_pass(k: i32, array: &[i32]) -> bool {
    let mut x = 0;
    let mut y = array.len() - 1;
    let mut i = 0;
    let mut sorted = array.to_owned();
    sorted.sort();

    while i < sorted.len() {
        println!("x:{} y:{}", sorted[x], sorted[y]);
        if array[x] + array[y] == k {
            return true;
        } else if k - sorted[y] > sorted[x] {
            x = x + 1;
        } else if k - sorted[x] < sorted[y] {
            y = y - 1;
        }
        if y == x || x > y {
            break;
        }
        i = i + 1;
    }
    false
}
