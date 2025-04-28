//fn distans (incendio: i32, bombero: i32, ) -> i32{
//    let mut res=0;
//    resu= 
//}

struct Punto {
    latitud: f64,
    longitud: f64,
}
struct Poligono {
    puntos: Vec<Punto>,
}
impl Poligono {
    fn agregar_poligono (&mut self, punto:Punto){
        self.puntos.push(punto);
    }
}
fn main (){
    let mut poligono = Poligono {puntos: Vec::new()};
    poligono.agregar_poligono(Punto {latitud: 2.0, longitud: 4.0});
    poligono.agregar_poligono(Punto {latitud: 5.0, longitud: 6.0});
    poligono.agregar_poligono(Punto {latitud: 5.0, longitud: 1.0});
    poligono.agregar_poligono(Punto {latitud: 3.0, longitud: 3.0});
    println!("Poligono con {} puntos",poligono.puntos.len());
}
