

//2º exem printar dados na tela

//para trabalhar com entrada e saída de dados utilizamos a biblioteca IO (input / Output - Entrada e Saída)
use std::io;
/*
   main é o ponto de entrada do programa...
   Já o fn => nós caminha para uma nova função.
   nos indica que aqui não há parametro.
   e inicia o bloco {}
*/
fn main() {

    /* 
       definições de string.
       Importante quando você redeclara a mesma variavel o RUST limpa o valor da variavel anterios. 
       Lembrando que o RUST não tem (garbage collection).
       O que faz que você tenha algo rapio e seguro, porque nos ciclos de vidas eles são encerrados
       CICLO DE VIDA É O BLOCO OU SEJA A FUNÇÃO QUE ELA ESTA: Quando saí da quela função as variaveis 
       são limpas ou seja encerradas automaticamente...

       duas forma de setar o valor da string 
       com 1º forma com a conversão.
    */

    //println como (! exclamação) é que ele vai escrever algo na tela (pulando linha)    
    println!("Advinhe o número!");

    println!("Digite o seu palpite.");

    let mut palpite = String::new();

    io::stdin()
         /* estou passando uma referencia de (memoria mutavel => &mut)
            lembrando que unsize não pode ser colocado numeros negativos */
        .read_line(&mut palpite)
        .expect("Falha ao ler entrada");

    println!("Você disse: {}", palpite);
}
