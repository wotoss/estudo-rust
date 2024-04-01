
// 1º exemplo - usando looop em RUST
// fn main (){
//   //fazendo um contador das vezes do loop
//   let mut contador: i32 = 0;
//   loop {
//     //roda initamente
//     println!("loop até 10 - {}", contador);
//     //se eu quero parar eu coloco um break
//     //compila uma vez e para

//     contador += 1;
//     if contador > 10 {
//       break
//     }
    
//   }
// }




//2º exemplo - neste exemplo de loop vamos pular um numro no caso o (4) do loop até 10

 fn main (){
   //fazendo um contador das vezes do loop
   let mut contador_pulando_4: i32 = 0;
   loop {
    
    contador_pulando_4 += 1;

     if contador_pulando_4 == 4{
       //quando eu continuo eu pulo esta (vez - do loop)
       continue;
     }
       println!("loop até 10, pulando 4 - {}", contador_pulando_4);
     if contador_pulando_4 > 10 {
        break
       }
    
     }
 }


