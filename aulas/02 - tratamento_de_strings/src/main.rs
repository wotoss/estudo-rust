// fn main() {
//     println!("Hello, world!");
// }

/*
   # formação
   # syntaxe basica
   # data type
   # lint
   # rust-docs
*/

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

    //estamos com string não mutaveis por padrão do (RUST).
    let s: String  = "ola".to_string();
    let s: String  = String::from("Olá");
    
    //duas forma de passar string empty ou vazia
    let s: String  = String::new();
    let s: String  = "".to_string(); 

    //neste exemplo ela não é limpa e sim acrescentada
    //estamos em uma string mutavel  mut
    let mut z: String = String::new();
    z += "1 r";

    println!("{}", s);

    
}
