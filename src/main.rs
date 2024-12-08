mod combinacao_resultados;
mod divisao_lista;
mod gerenciamento_threads;
mod persistencia;

// use std::env;
// use std::process;
use std::sync::{Arc, Mutex};
// use std::thread;

use combinacao_resultados::combinar_resultados;
use divisao_lista::dividir_lista;
use gerenciamento_threads::gerenciar_threads;
use persistencia::{carregar_estado, salvar_estado, Estado};

use num_cpus;

fn main() {
    println!("\nPrograma de Soma Paralela de Numeros\n");

    // Exemplo de lista de numero grandes
    let numeros: Vec<i32> = (1..=10000).collect();

    // Determinar o numero de threads com base no numero de nucleos disponiveis
    let numero_threads = num_cpus::get();
    println!("Numero de nucleos disponiveis: {}", numero_threads);

    // Caminho para salvar o estado
    let caminho_estado = "estado.json";

    // Tentar carregar o estado existente
    let estado = match carregar_estado(caminho_estado) {
        Ok(est) => {
            println!("Estado carregado. Continuando a soma a partir de onde parou.");
            est
        }
        Err(_) => {
            println!("Nenhum estado salvo encontrado. Iniciando nova soma.");
            Estado::novo(numero_threads)
        }
    };

    // Dividir a lista em sublistas com base no numero de threads
    let sublistas = dividir_lista(&numeros, numero_threads);

    // Envolver o estado em Arc e Mutex para compartilhamento seguro entre threads
    let estado_arc = Arc::new(Mutex::new(estado));

    // Gerenciar as threads, passando o estado compartilhado
    let handles = gerenciar_threads(sublistas, estado_arc.clone(), caminho_estado.to_string());

    println!("Aguardando as threads completarem...");

    // Combinar os resultados das threads
    let soma_total = combinar_resultados(handles);

    println!("\nA soma total e: {}", soma_total);
}
