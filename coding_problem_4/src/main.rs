fn main() {
    //let array: [i32; 4] = [2, 4, -1, 1];
    //let array = [1, 2, 0];
    let array: [i32; 9] = [1, 2, 0, -1, 1, 4, 2, 20, 18];
    let mut test_array: [bool; 9] = [false; 9];
    let mut missing_val = array.len();
    let mut x = 0;
    
    while x < array.len() {
        if array[x] < array.len().try_into().unwrap() && array[x] > 0 {
            test_array[(array[x]) as usize - 1] = true;
        }
        x += 1;
    }
    let mut y = 0;
    while y < array.len() {
        if test_array[y] == false {
            missing_val = y + 1;
            break;
        }
        y += 1;
    }

    println!("the smallest missing val {}", missing_val);

}
