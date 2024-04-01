//3º exemplo - um novo exemplo while
fn main() {
    //varialvel mutavel (mut) ela pode sofrer alterações.
    let mut resolucao_while: i32 = 1;

    //(while - enquanto) esta variavel resolucao_while for igual ou menor que 20 eu entro no if
    while resolucao_while <= 20 {
        //aqui estarei usando operadores logicos
        //SE 10 for (menor ou igual - 10) E (menor ou igual - 15) eu entro no if
        if resolucao_while >= 10 && resolucao_while <= 15 {
            //entrando no if eu somo mais +1 e dou o continue
            resolucao_while += 1;
            continue;
        }

        //então não vai mostrar na tela [16, 17, 18, 19, 20]
        //irá ser parado o programa antes.
        if resolucao_while > 10 {
            break;
        }
        println!(
            "while não irá mostar no console [16, 17, 18, 19, 20] : {}",
            resolucao_while
        );

        resolucao_while += 1;
    }
}

/* //3º exemplo - while como operadores logicos e condicional no if
fn main () {
  //varialvel mutavel (mut) ela pode sofrer alterações.
  let mut resolucao_while: i32 = 1;

  //(while - enquanto) esta variavel resolucao_while for igual ou menor que 20 eu entro no if
  while resolucao_while <= 20 {

  //aqui estarei usando operadores logicos
  //SE 10 for (menor ou igual - 10) E (menor ou igual - 15) eu entro no if
    if resolucao_while >= 10 && resolucao_while <= 15 {
   //entrando no if eu somo mais +1 e dou o continue
      resolucao_while += 1;
      continue;
    }


    println!("while não irá mostar não irá mostrar no console [10, 11, 12, 13 , 14, 15] : {}", resolucao_while);

    resolucao_while += 1;
  }
} */

//#endregion
