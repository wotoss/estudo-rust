fn main() {
    println!("===[Ele faz o for ou (looop até 4) neste range]====");
    println!("========================================");

    for number in 1..=4 {
        //quando ele for (igual a 4) ele sai do for...
        println!("Número: {}", number);
    }
    println!("==================================");

    println!("===[ Ele faz o for até (menor que 4) neste range]==");
    println!("=======================================");

    for number in 1..4 {
        //neste caso ela vai até (menor do que 4)
        println!("Número: {}", number);
    }
    println!("=======================================");
}
