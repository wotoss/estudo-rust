/* 
  RUST não tem (switch case) como C#, java, JavaScript
  O exemplo mais proximo e logico é o (Padrão Match)
  O (go ou golange) tambem trabalha com o (Case) de forma diferente

  Resumo o match seria isto para colocarmos o caso que queremos seguir, apartir 
  da minha (variavel ou base declarara).
*/


fn vendo_como_bebuger_se_comporta(){
  let primeiro_passo: i32 = 1;
  let segundo_passo: i32 = 2;
  let resultado_passo: i32 = primeiro_passo + segundo_passo;

  println!("resultado - {}", resultado_passo)
}

fn main () {

  vendo_como_bebuger_se_comporta();

  let number: i8 = 1;
  //eu tenho uma declaração i8 bits e ai eu faço as verificações do que tem dentro do numero
  match number {
    1 => println!("Um"),
    2 => println!("Dois"),
    3 => println!("Três"),
  //quando você tem um retorno de um dado que você não quer (mapear você inseri o underline _)
    _ => println!("Outro número"),
  }
}
