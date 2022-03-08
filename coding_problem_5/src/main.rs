
//cons(a, b) constructs a pair, and car(pair) and cdr(pair) returns the first and last element of that pair.
//For example, car(cons(3,4)) returns 3, and cdr(cons(3, 4)) returns 4.
// given this implementation of cons:
// def cons(a, b)
//  def pair(f):
//        return f(a, b)
//    return pair

fn cons(a: i32, b: i32) -> (i32, i32) {
    (a,b)
}

fn car(pair: (i32, i32)) -> i32 {
    pair.0
}

fn cdr(pair: (i32, i32)) -> i32 {
    pair.1
}


fn main() {
    cons(3, 4);
    car((3, 4));
    cdr((3,4));
}

#[test]
fn test () {
    assert_eq!(car(cons(3, 4)), 3);
    assert_eq!(cdr(cons(3, 4)), 4);
}
