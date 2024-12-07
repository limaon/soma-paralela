mod combinacao_resultados;
mod divisao_lista;
mod gerenciamento_threads;
mod persistencia;
mod soma_parcial;

// use std::env;
use std::process;

use combinacao_resultados::combinar_resultados;
use divisao_lista::dividir_lista;
use gerenciamento_threads::gerenciar_threads;

fn main() {
    println!("Programa de Soma Paralela de Numeros");

    let numeros = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let numero_threads = 2;

    println!("Dividindo a lista em {} threads...", numero_threads);
    let sublistas = dividir_lista(&numeros, numero_threads);

    println!("Criando e executando as threads...");
    let handles = gerenciar_threads(sublistas);

    println!("Combinando os resultados...");
    let soma_total = combinar_resultados(handles); // 15+40

    println!("A soma total é: {}", soma_total);

    process::exit(0);
}
