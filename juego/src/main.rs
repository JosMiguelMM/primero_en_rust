//librerias
use csv::{ReaderBuilder, StringRecord};
use std::{fs};

const FILENAME: &str = "history.csv"; //nombre del archivo .csv

//TIPO; TAG; TEXTO; VIDA
#[derive(Debug)]
struct DatosHistoria {
    tipo: String,
    tag: String,
    texto: String,
    vida: i32,
}

//impl DatoHistoria{

///}

fn main() {
    let mut datos_historia: Vec<DatosHistoria>= vec![];
    let content = fs::read_to_string(FILENAME).unwrap(); //lee el archivo .csv y poder mostrar los datos a travez de la consola
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes()); //lee content y separarlo por el delimitador (;)

    //a travez del siguiente for se muestran los datos del archivo .csv en columnas
    for result in rdr.records() {

        let result = result.unwrap();
        let vida=result.get(3).unwrap().trim();
        let vida:i32=vida.parse::<i32>().unwrap_or(0);

        let dato = DatosHistoria {
            tipo: result.get(0).unwrap().trim().to_string(),
            tag: result.get(1).unwrap().trim().to_string() ,
            texto: result.get(2).unwrap().trim().to_string(),
            vida:vida,
        };
        //datos_historia.push(dato);
        println!("{:?}", dato)
    }
    println!("{:?}", datos_historia);

}
