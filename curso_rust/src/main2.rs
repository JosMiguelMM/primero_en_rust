pub fn main() {

    loop {
        println!("Por favor introduce tu nombre");
        let mut nombre: String = String::new();
        std::io::stdin().read_line(&mut nombre).unwrap();
        nombre = nombre.trim().to_string(); //trim es para quitar los espacios

        //OBTENER LA EDAD DE LA PERSONA
        println!("Por favor introduce tu edad");
        let mut edad_str = String::new();
        std::io::stdin().read_line(&mut edad_str).unwrap();

        //CONVERTIR ESA EDAD A UN NUMERO
        let  edad: u8 = edad_str.trim().parse().unwrap(); //parse es para convertir a numero
        println!("Tu nombre es {nombre} y tu edad es {edad}");

        if edad>=18 {
            println!("Eres mayor de edad");
            if edad >= 70 {
                println!("Persona de mucho cuidado");
            }
        } else {
            println!("Eres menor de edad");
        }

        if edad+45==46 {
            println!("Eres un genio");
            break;
        } else {
            println!("Eres un perdedor, digita nuevamente la edad ");
            break
        }
    }
}