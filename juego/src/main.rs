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

fn main() {
    let mut datos_historia: Vec<DatosHistoria>= vec![];
    let content = fs::read_to_string(FILENAME).unwrap(); //lee el archivo .csv y poder mostrar los datos a travez de la consola
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes()); //lee content y separarlo por el delimitador (;)

    //a travez del siguiente for se muestran los datos del archivo .csv en columnas
    for result in rdr.records() {
        let dato = DatosHistoria {
            tipo: result.as_ref().unwrap().get(0).unwrap().trim().to_string(),
            tag: result.as_ref().unwrap().get(1).unwrap().trim().to_string() ,
            texto: result.as_ref().unwrap().get(2).unwrap().trim().to_string(),
            vida:0,
        };
        //datos_historia.push(dato);
        println!("{:?}", dato)
    }
    println!("{:?}", datos_historia);

}
