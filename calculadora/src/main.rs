use regex::Regex;

fn main() {
    //REDEX
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap(); //EXPRESION REGULAR PARA SUMAR
    let re_mult = Regex::new(r"(\d+)\s?\*\s?(\d*)").unwrap(); //EXPRESION REGULAR PARA MULTIPLICAR
    let re_resta = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap(); //EXPRESION REGULAR PARA RESTAR
    let re_divisi = Regex::new(r"(\d+)\s?/\s?(\d+)").unwrap(); //EXPRESION REGULAR PARA DIVISION


    //TRAER DATOS DEL USUARIO
    println!("Por favor introduce tu expresion: "); //MUESTRA EL MENSAJE PARA PEDIR EL MENSAJE
    let mut expresion: String = String::new(); //SE DECLARA Y TOMA LA EXPRESION
    std::io::stdin().read_line(&mut expresion).unwrap();

    //OPERACIONES MATEMATICAS

    //DIVISION
    loop {
        let caps = re_divisi.captures(&mut expresion.as_str()); //CAPTURA LA EXPRESION SEGUN LA EXPRESION REGULAR Y ANALISA SI HAY COINCICDENCIA CON  LA ENTRADA "re_divisi"

        if caps.is_none() { //SI LA EXPRESION CAPS ESTA VACIA SALGA DEL BUCLE
            break
        }
        let caps=caps.unwrap();// SI NO CONTINUA EL CICLO Y DESEMPAQUETA EL ARCHIVO

        let caps_expression = caps.get(0).unwrap().as_str();  //TOMA LA EXPRESION QUE ESTA "+" Y LO QUE ESTE DESPUES DE ESTE SIGNO
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap(); //TOMA EL VALOR QUE ESTA A LA IZQUIERDA DE LA EXPRESION
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap(); //TOMA LO QUE ESTE AL LADO DERECHO DEL SIGNO "+"

        let addition = left_value / right_value; //REALIZA LA SUMA DE LOS DATOS left_value + right_value
        expresion = expresion.replace(caps_expression, &addition.to_string());  //TOMA LA VARIABLE EXPRESION
        println!(" izquierda: { }  derecha:  { } lo guardado va {}", left_value, right_value, expresion);
    }

    //MULTIPLICACION
    loop {
        let caps = re_mult.captures(&mut expresion.as_str()); //CAPTURA LA EXPRESION SEGUN LA EXPRESION REGULAR Y ANALISA SI HAY COINCICDENCIA CON  LA ENTRADA "re_add"

        if caps.is_none() { //SI LA EXPRESION CAPS ESTA VACIA SALGA DEL BUCLE
            break
        }
        let caps=caps.unwrap();// SI NO CONTINUA EL CICLO Y DESEMPAQUETA EL ARCHIVO

        let caps_expression = caps.get(0).unwrap().as_str();  //TOMA LA EXPRESION QUE ESTA "+" Y LO QUE ESTE DESPUES DE ESTE SIGNO
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap(); //TOMA EL VALOR QUE ESTA A LA IZQUIERDA DE LA EXPRESION
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap(); //TOMA LO QUE ESTE AL LADO DERECHO DEL SIGNO "+"

        let addition = left_value * right_value; //REALIZA LA SUMA DE LOS DATOS left_value + right_value
        expresion = expresion.replace(caps_expression, &addition.to_string());  //TOMA LA VARIABLE EXPRESION
        println!(" izquierda: { }  derecha:  { } lo guardado va {}", left_value, right_value, expresion);
    }

    //SUMA
    loop {
        let caps = re_add.captures(&mut expresion.as_str()); //CAPTURA LA EXPRESION SEGUN LA EXPRESION REGULAR Y ANALISA SI HAY COINCICDENCIA CON  LA ENTRADA "re_add"

        if caps.is_none() { //SI LA EXPRESION CAPS ESTA VACIA SALGA DEL BUCLE
            break
        }
        let caps=caps.unwrap();// SI NO CONTINUA EL CICLO Y DESEMPAQUETA EL ARCHIVO

        let caps_expression = caps.get(0).unwrap().as_str();  //TOMA LA EXPRESION QUE ESTA "+" Y LO QUE ESTE DESPUES DE ESTE SIGNO
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap(); //TOMA EL VALOR QUE ESTA A LA IZQUIERDA DE LA EXPRESION
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap(); //TOMA LO QUE ESTE AL LADO DERECHO DEL SIGNO "+"

        let addition = left_value + right_value; //REALIZA LA SUMA DE LOS DATOS left_value + right_value
        expresion = expresion.replace(caps_expression, &addition.to_string());  //TOMA LA VARIABLE EXPRESION
        println!(" izquierda: { }  derecha:  { } lo guardado va {}", left_value, right_value, expresion);
    }

    //resta
    loop {
        let caps = re_resta.captures(&mut expresion.as_str()); //CAPTURA LA EXPRESION SEGUN LA EXPRESION REGULAR Y ANALISA SI HAY COINCICDENCIA CON  LA ENTRADA "re_add"

        if caps.is_none() { //SI LA EXPRESION CAPS ESTA VACIA SALGA DEL BUCLE
            break
        }
        let caps=caps.unwrap();// SI NO CONTINUA EL CICLO Y DESEMPAQUETA EL ARCHIVO

        let caps_expression = caps.get(0).unwrap().as_str();  //TOMA LA EXPRESION QUE ESTA "+" Y LO QUE ESTE DESPUES DE ESTE SIGNO
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap(); //TOMA EL VALOR QUE ESTA A LA IZQUIERDA DE LA EXPRESION
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap(); //TOMA LO QUE ESTE AL LADO DERECHO DEL SIGNO "+"

        let addition = left_value - right_value; //REALIZA LA SUMA DE LOS DATOS left_value + right_value
        expresion = expresion.replace(caps_expression, &addition.to_string());  //TOMA LA VARIABLE EXPRESION
        println!(" izquierda: { }  derecha:  { } lo guardado va {}", left_value, right_value, expresion);
    }
    //MOSTRAR EL RESULTADO
    println!("El resultado es: {}", expresion);
}
