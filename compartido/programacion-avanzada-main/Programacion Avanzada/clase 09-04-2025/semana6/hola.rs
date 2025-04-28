fn main(){
    let areas=[120, 350, 80, 200, 150];
    let umbral=100;
    let mut total =0
    suma_umbral(areas.to_vec(),umbral, &total);
    println!("Areas afectadas: {}",total);
}

fn suma_umbral(h:Vec<i32>, u :i32, &mut s:i32) -> i32{
    //let mut suma=0;
    let n=largo(&h);
    for i in 0..n {
        if h[i] > u {
           *s += h[i];
        }
    }
    //return suma;
}

fn largo(l:&Vec<i32>)-> usize{
    let mut contador=0;
    for i in l{
        contador += 1;
    }
    return contador;
}