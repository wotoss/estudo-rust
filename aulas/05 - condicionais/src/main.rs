/*
   Dado que eu tenha um ano de nascimento, e faço a subtração pelo ano atual,
   então dev ter o valor da idade da pessoa.

   Para resolver este solução 
   => Ano Atual
   => Ano de nascimamento 
   => Fazer a operação de substração

   Resolução feita...
   1º Extração da data por regular explexon
   2º Extração da data por (split)
   3º Convertendo os valores da data nas variavels (dia_usuario, mes_usuario, ano_usuario)
   4º Usamos chrono para pegar a data atual da maquina, servidor ou host de hopedagem
   no formato (padrão UTC - Tempo Universal Coordenado) é um padrão de tempo utilizado como referência para manter a hora exata em todo o mundo.
   5º 
 */

 //para manipulação de input e output (entrada/saida)
 use std::io;
 //instalei o chrono e vou utilizar como uma ferramenta de data
 use chrono::{Utc, Datelike};
 //instalamos a dependencia do (regex) trazida no nosso gerenciador de pacote crates.
 //estou importando tambem a regex para trabalhar com a formatação de minha data.
 use regex::Regex;
 fn main () {
   
   //quero que digite o ano de nascimento neste formato
   println!("Digite a data de seu nascimento (dd/mm/yyyy):");

   //temos uma variavel mutavel (mut) ela pode sofrer alterações.
   //estou criando uma variavel mutavel sobre (string)
   let mut data_nascimento: String = String::new();
   //caso de falha no momento de capturar ele irá mostrar em tela.
   io::stdin().read_line(&mut data_nascimento).expect("Falha ao ler a entrada");

   /*
     trabalhando com a (formatação em regex). 
     nesta base de formato => (dd/mm/yyyy)
     vamos usar a (Regex) para validar a data de nascimento.
     r"\d{2} teremos {2} caracter inteiro(\d)
     Olhado a nossa base que é a data eu preciso de um scape são duas barras \/
     /mm => Agora um outro \/\d com mais dois caracteres inteiro.
     na ultima parte que é o ano yyyy precisaremos \/\d{4} d-numeros inteiros {4} a quantidade de carcter representando o ano
     \d{2}\/\d{2}\/\d{4}
     Lembrando que o stdin() recebe a entrada do usuário apartir do console.

     Detalha a String não é uma variavel (by copy) e sim uma variavel (by ref).
     String é um conteudo armazenado na memoria HEAP então pensando na performace é interressante como opção passar mos a (referencia de memoria &)
   */

    let data_regex: Regex = Regex::new(r"^\d{2}\/\d{2}\/\d{4}$").unwrap();
    /* 
      preciso ver se a data digitada é inválida
      Detalha a String não é uma variavel (by copy) e sim uma variavel (by ref).
      String é um conteudo armazenado na memoria HEAP então pensando na performace é interressante como opção passar mos a (referencia de memoria &)
      Já o if eu digo se(! - não é uma data_nascimento valida entra no if e envio a mensagem no console)
    */
    if ! data_regex.is_match(&data_nascimento.trim()) {
      //neste caso estou utilizando o (.trim())) para retirar o (/n - quebra de linha)
       println!("A data que você digitou ({}) não esta no formato permitido", data_nascimento.trim());

       //entrando no if deu erro e vou encerrar esta função, com return eu encerro o fluxo de vida desta funçaõ.
       return;
    }

    /*
       a data sendo valida perciso fazer as extrações.
       vou usar o slpit para separa a data em variveis diferentes
       como estamos montando um objeto de split, então temos como logica que na posição [0] pegamos o ano [1] pegamos o mês [2] e na posição dois pegamos o ano]
       Lembrando que o (.collect() - faz com que me retorne um vetor).
     */
    let data_split: Vec<&str> = data_nascimento.split("/").collect();

   /* 
    aqui eu retiro os (espaços e o \n) com (trim) e faço o (parse) deste ano transformando de 
    string para (i32)inteiro. 
    Detalhe: quando colocamos o parse ele já verica (ou seja enxerga) o (tipo que declarei na varivel) e entende para a conversão. 
    caso eu colocasse um let ano: f32 ele entenderia como float.
    colocando u32 ele não permite inserir negativo neste numero inteiro. Ou seja (unsignes - são tipos de variaveis que não aceitam negativos).
    let ano: i32 = ano_string.trim().parse().expect("Por favor digite o número válido");

    Outras linguagem:
    C# no Java nós diriamos para ele no que converter bem no parse exemplo parseInt(), parseFloat(), parseDouble();
    Já no RUST nós declaramos na variavel o tipo para conversão no parse().

    Esta (variavel data_split, esta me retornando um vetor - eu passando a aposição consigo ter o que eu preciso na data - (data_split[2]) )

    Neste momento eu tenho todo o detalhe da data de aniversário desta pessoa nas variaveis (dia_usuario, mes_usuario, ano_usuario).

  */
   let dia_usuario: u32 = data_split[0].trim().parse().expect("Por favor digite o número válido");
   let mes_usuario: u32 = data_split[1].trim().parse().expect("Por favor digite o número válido");
   let ano_usuario: i32 = data_split[2].trim().parse().expect("Por favor digite o número válido");
   
   /* 
     neste momento estou pegando o ano atual e recebo em i32, no i16 não dá certo.
     nota: que o ano atual esta sendo buscado na [maquina local ou no  servidor de hospedagem]
     Lembrando que o (padrão UTC - Tempo Universal Coordenado) é um padrão de tempo utilizado como referência para manter a hora exata em todo o mundo.

     Neste momento eu tenho todo o detalhe da data atual vinda do (servidor ou da maquina local ou host de hospedagem) desta pessoa nas variaveis (dia_usuario, mes_usuario, ano_usuario).
   */
    let data_atual = Utc::now();
    let dia_atual: u32 = data_atual.day();
    let mes_atual: u32 = data_atual.month();
    let ano_atual: i32 = data_atual.year();
  
   //isolei e fiz a conversão das duas variavee para u8 com o (ano_atual - ano_usuario) as u8;
   let mut idade_usuario :u8 = (ano_atual - ano_usuario) as u8;
   /* 
     Importante: lembrar nesta verificações que ao ter a data do usuario redonda depois fazemos as verificações do dia e do mês de
     aniversário. Daí conseguimos o retorno correto, para mostrar em tela.
     verificação se o (mês do usuario for maior que o mês atual eu retiro - 1)
   */
   if mes_usuario > mes_atual {
  //preciso que a minha (variavel idade_usuario seja mut - mutavel) para eu poder fazer a alteração.
     idade_usuario -= 1;
  
  //se o dia_usuario for maior que a data atual ele ainda não fez aniversário 
  //então o retorno da data será -1   
   } else if  dia_usuario > dia_atual {
       idade_usuario -= 1;
   }

   println!("A sua idade é: {}", idade_usuario);
   println!("continuar.....")
 }