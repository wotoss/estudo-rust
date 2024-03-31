# Teremos todos os exemplos de códigos do estudo da linguagem Rust

### Link de estudos abaixo
https://github.com/wotoss/estudo-rust

### Link para montar e testar regex ou regular expression
https://rubular.com/

### O gerenciador de pacotes do RUST é o crates
https://crates.io/


### Dica 
Toda empresa deveria ter (Rust) em seus treinamento.

# Aula 29/03/2024
### Condicionais em Rust

Em Rust, as estruturas de controle condicionais são usadas para controlar o fluxo de execução do programa com base em condições lógicas. As condicionais mais comuns em Rust são `if`, `else if` e `else`.

## Estrutura Básica

## if
A estrutura `if` é usada para executar um bloco de código se uma condição especificada for verdadeira. Se a condição não for verdadeira, o bloco de código não será executado.

```rust
if condição {
    // código a ser executado se a condição for verdadeira
}
```

## else if
A estrutura `else if` é usada após um bloco `if` para testar uma condição adicional se a primeira condição não for verdadeira. Você pode ter quantos else `if` forem necessários para testar várias condições.
```rust

if condição1 {
    // código a ser executado se a condição1 for verdadeira
} else if condição2 {
    // código a ser executado se a condição1 for falsa e a condição2 for verdadeira
}

``` 


## else

```rust
if condição1 {
    // código a ser executado se a condição1 for verdadeira
} else if condição2 {
    // código a ser executado se a condição1 for falsa e a condição2 for verdadeira
} else {
    // código a ser executado se nenhuma das condições anteriores for verdadeira
}

``` 
