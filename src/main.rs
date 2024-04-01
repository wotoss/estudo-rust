/*
  Dadp que agora você conhece a forma de fazer loop no RUST, crie para mim
  um programa de tabuada , onde você irá digitar o multiplicados e o resultado
  terá que aparecer multiplicado por 2.

  Daria para colocar um (sleep - delay) para melhor a apresentação.
*/

use std::io;

fn main() {
    println!("=============================================================");
    println!("=================[Estamos na tabuada ]=======================");
    println!("=============================================================");
    println!("Digite o valor da tabuada");
    let mut valor_tabuada: String = String::new();
    //com o stdin eu consigo lêr a variavel digitada.
    io::stdin()
        .read_line(&mut valor_tabuada)
        .expect("Falha ao ler a linha");

    //faço o parse para este tipo declarado (i8)
    let valor_tabuada: i16 = valor_tabuada
        .trim()
        .parse()
        .expect("Por favor, digite o numero da tabuada..");

    for multiplicador in 1..=10 {
        println!(
            "{} X {} = {}",
            multiplicador,
            valor_tabuada,
            (multiplicador * valor_tabuada)
        );

        println!("=============================================================");
    }
}
