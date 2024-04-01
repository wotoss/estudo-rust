
//Não tem ternário. Este é o exemplo logico mais proximo. 
fn main () {
  //declarei um let de tipo i8 inteiro de 8 bits
  let numero: i8 = 3;
  /*
    seria um pouco parecido com o ternário, mas ao invés de termos (? - para o SE) e a (: - ELSE)
    neste caso e estrutura logica é diferente
    SE numero for (> menor doque 3 o resultado é: 4) SENão o (resultado é: 6)
    neste caso o resultado é: 6

    Continuando: A ultima operação é a que esta sendo retornada o (else)
   */
  let resultado: i8 = if numero > 3 { 4 } else { 6 };

  println!("O valor é: {}", resultado);
}