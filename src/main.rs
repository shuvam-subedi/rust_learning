mod calculator;

fn main(){
    let s: i32 = sum(1, 2);
    assert_eq!(s, 3);
    println!("Done");
}

fn sum(a: i32, b: i32) -> i32{
    a + b
}