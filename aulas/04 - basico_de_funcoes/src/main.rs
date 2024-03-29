
/*
  Entendendo função !
  O ciclo de vida de dados em memoria e encerrado em cada fluxo de função.
  Se você não criar suas funções no modelo (to_snake_case) o (RUST) se não ele dar warning - alerta.
  Detalhe das funções via (RUST) quando você (não coloca ponto e virgula ;) ao fim de uma função significa que ela terá um retorno, sem (;) (não precisa colocar o return).
  Caso coloque (;) e você quer que ela de um return aí você precisará especificar com o (return);

  No (RUST) não temos uma função void como temos no C# que não tem retorno. No (RUST) sempre teremos algum return o (void) não existe no (RUST).
*/

//este é função sem retorno 
fn escrever_funcao_sem_retorno(){
   println!("woto - programador")
}

fn main () {
   escrever_funcao_sem_retorno();
   escrever_funcao_sem_retorno();
   escrever_funcao_sem_retorno();
   let x: i8 = escrever_com_retorno();

   println!("{}", x);

   let valor_concatenado: String = funcao_com_parametros(String::from("woto"),  46);
   //vou mostrar o valor concatenado na tela
   println!("{}", valor_concatenado);
}

/*
  Eu consigo fazer retorno no (RUST) de duas formas.
  1º falando o tipo de retorno que eu quero ter
*/
//função com retorno 
fn escrever_com_retorno() -> i8 { //estou mostrando o (tipo - i8 bits) de retorno.
   println!("retornando tipo 8 bits");
   /* 
   opção 1
     veja que eu não coloco (;) caso eu coloque (;) eu tenho que escrever o (return).
     caso não coloque (;) ele retorna naturalmente.
   */
   1

   //tambem tenho esta opção 2

   //return 1;
}

/**
 * neste exemplo vamos ver uma função com parametros.
 * A função com parametros serve para eu receber o tipo que eu quero.
 * Lembrando que função com parametro eu tenho que defenir os (tipos que preciso recebe como String, inteiro, float, double, slice)
 */
fn funcao_com_parametros(valor:String, numero: i32) -> String {
   format!("{} - {}", valor, numero)

 }

 