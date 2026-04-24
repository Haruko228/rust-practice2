fn test_scope() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("x: {}, y: {}", x, y);
    }
    println!("x: {}", x);
}

fn test_shadowing() {
    let x: i32 = 5;
    {
        let x = x * 2;
        assert_eq!(x, 10);
    }
    let x = x + 1;
    assert_eq!(x, 6);
    println!("Final x: {}", x);
}

fn test_unused() {
    let _x = 100;
    println!("Unused check: done");
}

fn main() {
    test_scope();
    test_shadowing();
    test_unused();
}
