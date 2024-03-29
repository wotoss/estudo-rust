/* 
Entendimento do que vamos estudar!
(by copy) são dados primitivos, armazenados na memoria (stack)
(não e copy) são dados que são armazenados na memoria (HEAP) como (String, Array, Vetor)

Então vamos ver (variaveis by copy) e (variaveis by ref)
*/


/* 
   Neste caso usamos exemplo(1) (by copy)
   Lembrando: (by copy) são dados primitivos, armazenados na memoria (stack)
*/
// fn main () {
//    //se eu não declaro o bit ele usa como padrão o i32
//    //mas escovando bits eu passo i8
//     let x: i8 = 5;
//     //(by copy) são dados primitivos, armazenados na memoria (stack)
//     //vou fazer uma copia (diretamente) do valor x para y (tipos primitivos e todos os tipos primitivos armazenam na memoria stack)
//     //por ser dados leve e estar na memoria stack.
//     //Rust cria um dois endereço de memoria
//     let y: i8 = x;

//    println!("x: {}, y: {}", x ,y); 

// }


/*
   Este exemplo (2): (não é by copy)
   String estaaramazenado na memoria (HEAP) eu não posso pegar um (dado - grande como uma String, array, vetor) 
   e ficar trocando de (variavel) o que seria comum em C#, Java, JavaScript, mas no RUST se fazemos isto ele não aceita 
   passar as responsabilidades diretamente.
   
   Então fazer isto vocÊ terá que fazer um (clone) para realmente dizer que você quer duplicar os dados em memoria. (Lembrando que desta forma não é tão performatico).
   Ou
   Você envia este dados por (referencia). (Neste caso é mais perfomatico do que o clone).

   Agora outro ponto importante: Caso eu esteja usando uma (slice e não String) aí eu consigo fazer tranquilamente afinal o (Rust vai entender que é pouco dados esta declaração)
   e que você esta na memoria (STACK)

   Solucionamos usando o (clone) Lembrando que estou duplicando a informação ou seja (criando duas variaveis diferentes) e criando um novo endereço de memoria.
*/

//Modelo  via clone (2-a)
//   fn main () {
//       //criei uma varivel x com valor de uma string
//       let x: String = String::from("Alunos de Rust");


//       /* 
//       Exemplo este bloco esta com exemplo que dá erro, estamos solucionando abaixo
//       //estou pegando esta variavel x e passando para variavel y
//       //esta forma de passar a responsabilidade vai dar erro quando vou printar
//        let y: String = x;
//       */

//       //esta forma sim consigo passar a responsabilidade via (clone)
//       //Lembrando que estou duplicando a informação ou seja (criando duas variaveis diferentes) e criando um novo endereço de memoria.
//       let y: String = x.clone();

//       //eu tentando printar em tela meu codigo da erro
//       println!("x: {}, y: {}", x, y);
//   }

  /*
   Modelo  via clone (2-b)
     Agora vamos resolver de outra forma usando o modelo (by refe) ou seja pegando a referencia de memoria.
  */
  fn main () {
   //criei uma varivel x com valor de uma string
   let x: String = String::from("Alunos de Rust");
   /* 
     Quando colocamos o  (& - E-comerical) ao invês de termos uma (String) teremos um (ponteiro).
     então vou apontar para o ponteiro da variavel (x) ou seja estarei fazendo por (referencia).
     perceba que ao invês de (String) ele virou um (ponteiro de String(&String)).
     Neste caso eu tenho uma performace melhor com o (ponteiro) do que com o (clone).
     Porque o (clone duplica a informação) e o (ponteiro faz a referencia) ambos na memoria (HEAP).

     Ponto importante se você, invalida a variavel x a variavel y é invalidada tambem ou seja não 
     deixando dados em memoria.
   */
   let y: &String = &x;

   //eu tentando printar em tela meu codigo da erro
   println!("x: {}, y: {}", x, y);
}




 /*
    Modelo  via clone (2-c)
    Resolução com (slice) ao invês de (String) dependendo do tamanho do dado.
    Lembrando que (slice é armazenado na memoria STACK) e (String, Array e Vetor - são aramazenados na memoria - HEAP)
    este é o mesmo exemplo acima do numero (2) mas daria certo
    pois estamos com uma (slice) 
 */

// fn main () {
//    //criei uma varivel x com valor de uma string
//    //let x: String = String::from("Alunos de Rust");

//    let x: &str = "Alunos de Rust";
//    //estou pegando esta variavel x e passando para variavel y
//    let y: &str = x;

//    //eu tentando printar em tela meu codigo da erro
//    println!("x: {}, y: {}", x, y);
// }

