fn main () {
    let mut nombre = String::from("Juan");
    let mut edad: i32=18;
    if edad < 18 {
        nombre = nombre + ", Don";
    } else {
        nombre = nombre + ", Jr.";
    }
    println!("{}", nombre);
}

