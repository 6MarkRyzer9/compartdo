fn main () {
    let x=[1,-2,4]; //crear una lista
    println!("Largo de la lista es: {}", largo(x.to_vec()));
}
fn largo(x: Vec<i32>) -> i32 {
    let mut contador=0;
    for _i in x {
        contador = contador + 1;
    }
    return contador;
}

