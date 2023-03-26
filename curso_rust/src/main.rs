mod main2;

fn main() {
    const  EDAD:f64= 20.7;
    let mut numero:i8=-2; //mut es para que la variable sea modificable
    let nombre:&str="miguel";
    //declrar una variable deciml

    numero=numero-108;

    println!("{}", EDAD);
    println!("{EDAD}");
    println!("Soy {nombre} y tengo { } años {numero}", EDAD);
    println!(" ");

    //ARCHIVO 2
    main2::main();
}
