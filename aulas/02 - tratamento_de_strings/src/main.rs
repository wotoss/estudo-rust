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

//MUITOS BONS EXEMPLOS EM FUNÇÕES SEPARADAS DE TRATAMENTO DE STRINGS

//neste momento estamos fazendo tratamento de strings.

//fn main() {

   //1º vamos utilizar o trim e ele serve para que possamos remover espaço
   //let removendo_espaço: &str = "  Hello, word!  ".trim();
   //observação: o comando: trim() serve para tirar os espaços
   //do inicio e do fim e não do meio.

   //2º com o comando trim_end() ele tira os espaços somente do final.
   //let removendo_espaço: &str = "  Hello, word!  ".trim_end();
   
   //3º neste caso com o comando: trim_start() estamos tirando os espaços 
   //somente do começo.

//    let removendo_espaço: &str = "  Hello, word!  ".trim_start();
//    println!("-{}-",removendo_espaço);
 
//}

//vamos para o exemplo letra maisculas
//fn main(){
    //1º tranformando o texto que estava em (letras minusculas) em (letras maiusculas)
    // let letras_maiusculas: String = "hello".to_uppercase();
    // println!("{}", letras_maiusculas);
    
    //2º Transformando letrar MAIUSCULAS EM  letras minusculas.
    // let letras_minusculas: String = "HELLO".to_lowercase();
    // println!("{}", letras_minusculas);
//}

//vamos fazer um replace uma alteração de conteudo
// fn main(){
//     //1º veja que alteramos a 2º parte do texto após a virgula
//     //mas poderiamos sim alterar a 1º ou o texto todo
//     let texto_original: &str = "Olá, mudaremos";
//     let replaced: String = texto_original.replace("mudaremos","foi alterada para Rust");

//     //Já neste exemplo alteramos o texto todo.
//     //let replaced: String = texto_original.replace("Olá, mudaremos","Mudaremos o texto todo!");
//     println!("{}", replaced); //Olá, foi alterada para Rust
// }

//veremos o uso dos tipos de strings que podemos trabalhar.
//ou seja padrão de quantificação.
//crates.io onde conseguimos tunar o nosso codigo é o nosso gerenciador de pacotes
//retirei do crates.io
//acabei de adicionar dentro do arquivo Cargo.toml => Inflector = "0.11.4"
//como se fosse nuget
// ou maven do java
//npm do nodejs
//ou composer

//use inflector::Inflector;
//com o uso do inflector nós conseguimos fazer estas formatações no texto
// [to_camel_case, to_snake_case, to_pascal_case]

// fn main() {

//     let x: String = "hello_world".to_camel_case();
//     println!("{}", x);

//     let j: String = "helloWorld".to_snake_case();
//     println!("{}", j);

//     let r: String = "helloWorld".to_pascal_case();
//     println!("{}", r);
// }


    //muitas vezes vamos ter que converter o texto para char para conseguir contar 
    //a string
    // fn main() {
         
    //     //sem inverter os dados 
    //     let w: String = "hello".chars().collect();

    //     //neste exemplo com a inversão dos dados usando o rev();
    //     //let w: String = "hello".chars().rev().collect();
    //     println!("{}", w);


    // }

    //contando uma quantidade de caracteres no Rust

    // fn main () {
    //     //a quantidade de carcter ou seja bits ele me devolve 5
    //     //let quantidade: usize = "hello".len();
    //     //mas se tiver o acento ele devolve 6 bits
    //     //isto acontece porque a maquina usa mais de um bit no (acento) e no (e)
    //     //let quantidade: usize = "héllo".len();

    //     //vamos agora contar do jeito certo.
    //     //Aí sim você vai contar a quantidade de char que você tem.
    //     //E não a quantidade de bits....
    //     //vocÊ pega a sua slice e conta a quantidade de Char aí tera o valor certo
    //     let quantidade: usize = "héllo".chars().count();
    //     //agora sim ! o retorno dos carcter ou char que o usuário digitou foram 5
    //     println!("Quantidade de: {}", quantidade);
    // }


    //trabalhando com contains 
    //você esta pegando dados de uma API 
    //e precisa verificar se contem ou contains algum tipo de palavra.

    // fn main() {

    //     //tanto faz se estou usando uma slice no !º exemplo
    //     //ou estou usando uma String 2º exemplo eu consigo utilizar o método contains().

    //     //este contains é um método dentro do slice  que vocÊ consegue verificar 
    //     //se tem a palavra ou frase solicitada.
    //     //eu tenho hello, world".contains("world") e faço a verificação se contains("world")
    //     //dentro do meu texto.
    //     let contains_substring: bool = "hello, world".contains("world");
    //     //existindo como é um boleano ele retornará o valor de true
    //     println!("existe ?: {}", contains_substring);

    //     //2º exemplo utilizando String 
    //     //neste caso deu como false porque não existe (worlds).
    //     let contains_substring: bool = String::from("hello, world").contains("worlds");
    //     println!("existe ?: {}", contains_substring);

    // }


    //Outro ponto que é muito utilizado para quem quer fazer migração e tratamento
    //de string.
    //Split ele vai pegar os espaços entre o texto e transformar isto em uma coleção.
    //o retorno será um vetor com os items separados, montando uma coleção.

// fn main () {
//     let texto: &str = "Hello, world! Welcome to Rust programming.";

//     //irá dividir a string pelo espaço.
//     //estou recebendo um vetor de (slice ou $str)
//     //ele me retorna a coleção atraves da collect()
//     let palavras: Vec<&str> = texto.split(' ').collect();

//     //Detalhe: o vetor é um objeto fechado e se eu quero abrir ele eu tenho que fazer uma 
//     //especie de (inspec). Ou seja (inspecionar) para poder listar os dados no console do terminal.
    
//     println!("{:?}", palavras);
    
// }




//neste exemplo eu quero pegar parte de uma string
//forma como tratamos uma substring
// fn main () {
//     //Lembrando que toda slice é um array de char ou array de bit
//     let b: &str = "hello";

//     //eu quero que começe com 0 e vai até 2 caracter.
//     //a contagem é mesma que fazemos em um array se inicia do zero
//     let substring: &str = &b[0..2];
//     //desta forma conseguimos pegar parte do char ou carcter que queremos tratar.

//     println!("{}", substring);
// }



//vamos falar de Regex ou Regular Expression

//use regex::Regex;

// fn main () {
//     /*
//     Detalhe: w ou seja word é uma palavra reservada, então eu coloco o \ ou a \ para ele entender que é qualquer texto
//       1º (\w+@) significa word + @ => Qualquer texto que tenha @
//       2º (\w+\.) qualquer texto que tenha . ponto
//       3º (\w+$) tem que encerrar com texto, se eu encerrar com (.) esta errado.
//      */
//   let email_regex = Regex::new(r"^\w+@\w+\.\w+$").unwrap();
//   let email: &str = "wotoss10@gmail.com";

//   if email_regex.is_match(email) {
//     println!("{} é um email válido.", email);
//   } else {
//     println!("{} é um email inválido.", email);
//   }
// }

//continuando com regex transformando slice com string
// fn main () {
//     let phone_regex = Regex::new(r"\(\d{2}\) \d{4,5}-\d{4}").unwrap();
//     let texto: &str = "O meu telefone é (12) 94704-7361. dasdo feito";

//     match phone_regex.captures(texto) {
//         Some(caps) => println!("Número encontrado: {}", caps.get(0).unwrap().as_str()),
//        None => println!("Não foi encontrado número."),
//     }

// }
