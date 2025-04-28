fn main () {
    let n:i32=5;
    let fibo_iter:i32 = fb_iter(n);
    println!("El fibonacci interactivo de {} es {}", n, fibo_iter);
}

fn fb_iter(n:i32)->i32{
    if n==0{
        0
    } else if n==1{
        1
    } else {
        fb_iter(n-1)+fb_iter(n-2)
    }
}
