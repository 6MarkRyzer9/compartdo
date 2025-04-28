fn main() {
    let mut suma:i32 =0; //acumular la suma de primos
    let mut x = 1; // el correlativo de numeros a evaluar
    let mut n = 0; // contar la cantidad de numeros primos encontrados
    while n<4 {
        // revisar si x es primo
        let mut contador = 0;
        // contabilizar la cantidad de numeros que dividen a x exactamente
        for i in 1.. x+1 {
            if x%i==0 {
                contador=contador+1;
            }    
        }    
        // de ser verdadero, el x es primo
        if contador <=2 {
            suma=suma+x;
            n=n+1;
        }
        // esto se encuentra  a la altura de while
        println!("{} - {} - {} - {}", x, n, suma, contador);
        x=x+1;
    }
println!("la suma de los primos son: {}", suma);
}
