fn main () {
    let l:[i32;10]=[4,4,5,-6,0,10,-25,-23,1,0]; 
    let mut cantidad:i32=0;
    for i in l {
        if i > 0 { 
            cantidad=cantidad+1
        }
    }
    println!("La cantidad de numeros positivos son : {}",cantidad);
}