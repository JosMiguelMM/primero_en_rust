//librerias
use csv::{ReaderBuilder, StringRecord};
use std::{fs};
use std::collections::{HashMap};
use std::thread::current;

const FILENAME: &str = "history.csv";
//nombre del archivo .csv para el juego
const FIRST_TAG: &str = "INICIO";

//TIPO; TAG; TEXTO; VIDA
#[derive(Debug)]
struct DatoHistoria {
    //se crea una estructura para poder mostrar los datos del archivo .csv
    tipo: String,
    tag: String,
    texto: String,
    vida: i32,
    opciones: Vec<DatoHistoria>,
}

impl DatoHistoria {
    //se crea una implementacion para poder mostrar los datos del archivo .csv
    fn new(row: StringRecord) -> DatoHistoria {
        let vida = row.get(3).unwrap().trim(); //el dato numerico vida al ser string se debe convertir a i32
        let vida: i32 = vida.parse().unwrap_or(0); //se convierte a i32

        return DatoHistoria {
            //se muestra cada dato en una columna que corresponde
            tipo: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            texto: row.get(2).unwrap().trim().to_string(),
            vida: vida,
            opciones: vec![],
        };
    }
}

//funcion principal
fn main() {
    let mut vida: i32 = 100;
    let mut tag_actual: &str = FIRST_TAG;

    let mut last_record: String = "".to_string();

    let mut datos_historia: HashMap<String, DatoHistoria> = HashMap::new();

    let content = fs::read_to_string(FILENAME).unwrap(); //lee el archivo .csv y poder mostrar los datos a travez de la consola
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes()); //lee content y separarlo por el delimitador (;)

    //atraves del siguiente for se muestran los datos del archivo .csv en columnas
    for result in rdr.records() {
        let result = result.unwrap(); //se trae del for
        let dato = DatoHistoria::new(result); //se muestra en columnas
        if dato.tipo == "SITUACION" {
            let record_tag = dato.tag.clone();//se muestran los datos de las situaciones
            datos_historia.insert(record_tag.clone(), dato);
            last_record= record_tag;
        } else if dato.tipo == "OPCION" {
            if let Some(data) = datos_historia.get_mut(&last_record) {//se muestran los datos de las opciones
                (*data).opciones.push(dato);
                //println!("{}", (*data).tag); //muestra los tags de las opciones
            }
        }
    }
    //gamelopp
    loop {
        println!("Tienes de vida: {}", vida); //se imprime la cantidad de vida que tiene

        if let Some(data) = datos_historia.get(tag_actual) {
            //println!("{:?}", data.texto);

            for (indice,option) in data.opciones.iter().enumerate(){
                println!("[{}] {}", indice, option.texto);
            }

            let mut seleccion=String::new();
            std::io::stdin().read_line(&mut seleccion).unwrap();
            let seleccion =seleccion.trim().parse().unwrap_or(99);

            if let Some(opcion_elegida)=data.opciones.get(seleccion){
                tag_actual=&opcion_elegida.tag;
            }else{
                println!("Comando no valido");
            }

            vida+=data.vida;
            println!(" ");


        }else {
            break;
        }
        //si la vida es menor que 0
        if vida<=0 {
            println!("Has perdido");
            break;
        }
    }
}