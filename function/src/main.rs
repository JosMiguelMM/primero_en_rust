fn main() {
    let retorno=sumar_uno(12.653);
    sumar_uno(retorno);
    sumar_uno(20.653534343);
    sumar_uno(20.0);
}

fn sumar_uno(numero:f32)->f32{
    let  numerofinal:f32=numero+1.0;
    println!("El numero final es: {}",numerofinal);

    return numerofinal;
}
