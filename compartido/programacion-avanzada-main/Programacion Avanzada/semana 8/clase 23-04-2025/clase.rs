fn main () {
    let n:i32=5;
    let mut factorial:i32=1;
    if n > 0{
        for i in 1..=n{
            factorial *=i;
        }
    }
    println!("El factorial de {} es {}", n, factorial);
    factorial=fn_factorial(n);
    println!("El factorial de {} es {}", n, factorial);
}

fn fn_factorial(n:i32)->i32{
    if n==0{
        1
    } else {
        n*fn_factorial(n-1)
    }
}