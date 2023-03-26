fn main() {

    let mut nombres=Vec::new();

    for y in 0..5 {

        let mut nombre=String::new();
        println!("Ingresa un nombre: ");
        std::io::stdin().read_line(&mut nombre).expect("Error");
        nombres.push(nombre);

    }


    for i in 0..nombres.len(){
        println!("Vector {} es: {}",i, nombres[i]);
    }
    let arreglo:[i32;5]=[1,2,3,4,5];
    for i in 0..arreglo.len(){
        println!("Arreglo {} es: {}",i, arreglo[i]);
    }
}
