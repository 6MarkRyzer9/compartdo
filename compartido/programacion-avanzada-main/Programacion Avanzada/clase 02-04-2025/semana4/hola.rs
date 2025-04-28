fn main (){
    let x: i32=2;
    let y: i32=3;
    if y==0 {
        res=1;
    } else {
        if x==0 {
            res = 0;
        } else {
            res = cal_potencia(x,y);
        }
    }
    println!("{}",res);
}

fn cal_potencia(base:i32,potencia: i32) -> i32 {
    let mut r = 1;
    for _i in 0.. potencia{
        r = r*base;
    }
    return r;
}