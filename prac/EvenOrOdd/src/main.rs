fn main() {
    is_even(239);
    is_even(129);
    is_even(9023);
}

fn is_even(x: i32) -> bool {
    if (x % 2 == 0) {
        print!("Your value: {x}, is even\n");
        return true;
    }
    else {
        print!("your value: {x}, is odd\n");
        return false;
    }
}


