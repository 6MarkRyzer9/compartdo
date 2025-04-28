fn main () {
    let mut l:[i32;6]=[6,2,4,8,10,20]; // "l" es puntero (es la direccion en la cual se encuentra la memoria)
    let mut mayor:i32=l[0];
    for i in &mut l[1..5] { //entrega una referencia sin el &
        if *i > mayor { //ayuda a asignar el valor "20" no la referencia 
            mayor=*i
        }
    }
    println!("{}",mayor);
}