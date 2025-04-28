fn main (){
    let n: i32= 10;
    let mut factorial:i32 =1;
    for i in 1..n+1 {
        factorial = factorial * i;
    }
    println!("el factorial de {} es {}",n, factorial);
}
