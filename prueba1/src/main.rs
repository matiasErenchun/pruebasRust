extern crate core;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

struct Persona
{
    nombre: String,
    edad: u8,
}

impl Persona
{
    fn new_person(nombre: String, edad: u8) -> Persona
    {
        Persona {nombre,edad}
    }

    fn print_person(&self)
    {
        println!("mi nombre es {} y tengo {} años", self.nombre,self.edad)
    }
}

fn read_text(url_file: &String, array: &mut Vec<String>) -> std::io::Result<()>
{
    println!("url: {}",url_file);
    let archivo = File::open(url_file)?;

    let lector = BufReader::new(archivo);
    for line in lector.lines() {
        match line {
            Ok(line)=> array.push(line),
            Err(..)=>println!("fin palabras")
        }

    }
    Ok(())
}

fn main()
{
    let mut my_arr: Vec<String> = Vec::new();
    let mut i :i64 = 0; // si nos e le agrega mut la variable no se puede cambiar
    while i<10
    {
        println!("i: {i}");
        i+=1;
    }
    let si = String::from("..\\prueba1\\src\\si.txt");
    read_text(&si, &mut my_arr).expect("TODO: panic message");//pasamos los valores con &para prestarlos
    println!("largo: {}",my_arr.len());
    for line in my_arr
    {
        for palabra in line.split(" ")
        {
            println!("{}", palabra)
        }
    }
    let persona1: Persona = Persona::new_person("matias".to_string(),30);
    persona1.print_person();
}
