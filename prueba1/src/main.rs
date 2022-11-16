extern crate core;

use std::io;
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

fn QQ()
{
    let mut i: i8=1;
    let mut j:i8;
    while i<10
    {
        j=1;
        while j<10
        {
            println!("{}x{}={}",i,j,i*j);
            j+=1;
        }
        i+=1;
    }
}

fn printN(n:i64)
{
    let mut i :i64 = 0; // si nos e le agrega mut la variable no se puede cambiar
    while i<n
    {
        println!("i: {i}");
        i+=1;
    }
}

fn readtext()
{
    let mut my_arr: Vec<String> = Vec::new();
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
}

fn crear_imprimir_persona()
{
    let persona1: Persona = Persona::new_person("matias".to_string(),30);
    persona1.print_person();
}

fn three_mountains()
{
    let mut contiar: bool= true;
    let mut first:i64=0;
    let mut second:i64=0;
    let mut third:i64=0;
    for i in 0..10
    {
        let mut input = String::new();
        match io::stdin().read_line(&mut input)
        {
            Ok(n)
            => {
                let mut valor: i64 =0;
                match input.trim().parse::<i64>() {
                    Ok(n) => valor=n,
                    Err(e) => valor=0,
                }
                if valor>first
                {
                    third=second;
                    second=first;
                    first=valor;
                }
                else if valor>second
                {
                    third=second;
                    second=valor;
                }
                else if valor>third
                {
                    third=valor;
                }

            }
            Err(error) => contiar=false,
        }

    }
    println!("{}",first);
    println!("{}",second);
    println!("{}",third);
}

//en el compilador de la competencia tira un error lo resolvi con python al final, a mi me funcionaba pero bueno
fn digit_number()->io::Result<()>
{
    let mut lines = io::stdin().lock().lines();
    let mut salida:Vec<i64>=Vec::new();

    while let Some(line) = lines.next() {
        let last_input = line.unwrap();

        // stop reading
        if last_input.len() == 0 {
            break;
        }
        else
        {
            let mut valor:i64=0;
            for palabra in last_input.split(" ") {
                valor+=palabra.replace('\n', "").parse::<i64>().unwrap();
            }
            let mut p:i64=0;
            while valor>0
            {
                valor = valor/10;
                p+=1
            }
            salida.push(p);
        }

    }
    for i in salida
    {
        println!("{}",i);
    }
    Ok(())
}

fn isita_right_triangle()
{
    let mut input = String::new();
    let mut bytes:usize=io::stdin().read_line(&mut input).ok().expect("error de lectura primera linea");
    let mut n:i64 =input.trim().parse::<i64>().ok().expect("no se puede parsear la entrada");
    let mut i:i64=0;
    while i<n
    {
        input = String::new();
        let bytes:usize=io::stdin().read_line(&mut input).ok().expect("error de lectura");
        let mut a:i64=0;
        let mut b:i64=0;
        let mut c:i64=0;
        let mut j:i64=0;
        for i in input.split(" ")
        {
            if j==0
            {
                a=i.trim().parse::<i64>().ok().expect("error al trasformar a");
            }
            if j==1
            {
                b=i.trim().parse::<i64>().ok().expect("error al trasformar b");
            }
            if j==2
            {
                c=i.trim().parse::<i64>().ok().expect("error al trasformar c");
            }
            j+=1;
        }
        if ((a*a)+(b*b)) == (c*c)
        {
            println!("YES")
        }
        else if ((a*a)+(c*c)) == (b*b)
        {
            println!("YES")
        }
        else if ((b*b)+(c*c)) == (a*a)
        {
            println!("YES")
        }
        else
        {
            println!("NO")
        }
        i+=1;
    }
}
fn main()
{


}
