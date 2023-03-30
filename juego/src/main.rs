//librerias
use csv::{ReaderBuilder, StringRecord};
use std::{fs};

const FILENAME: &str = "history.csv"; //nombre del archivo .csv para el juego

//TIPO; TAG; TEXTO; VIDA
#[derive(Debug)]
struct DatoHistoria { //se crea una estructura para poder mostrar los datos del archivo .csv
    tipo: String,
    tag: String,
    texto: String,
    vida: i32,
}

impl DatoHistoria{//se crea una implementacion para poder mostrar los datos del archivo .csv
    fn new (row: StringRecord)->DatoHistoria{
        let vida=row.get(3).unwrap().trim(); //el dato numeroco vida al ser string se debe convertir a i32
        let vida:i32=vida.parse::<i32>().unwrap_or(0); //se convierte a i32

        return  DatoHistoria {
            //se muestra cada dato en una columna que corresponde
            tipo: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string() ,
            texto: row.get(2).unwrap().trim().to_string(),
            vida:vida,
        };
    }
}

fn main() {
    let mut datos_historia: Vec<DatoHistoria>= vec![];
    let content = fs::read_to_string(FILENAME).unwrap(); //lee el archivo .csv y poder mostrar los datos a travez de la consola
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes()); //lee content y separarlo por el delimitador (;)

    //atraves del siguiente for se muestran los datos del archivo .csv en columnas
    for result in rdr.records() {

        let result = result.unwrap(); //se trae del for
        let dato=DatoHistoria::new(result); //se muestra en columnas
        //datos_historia.push(dato);
        println!("{:?}", dato)
    }
    println!("{:?}", datos_historia);

}
