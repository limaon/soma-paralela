mod combinacao_resultado;
mod divisao_lista;
mod gerenciamento_thread;
mod persistencia;
mod soma_parcial;

use combinacao_resultado::combinar_resultados;
use divisao_lista::dividir_lista;
use gerenciamento_thread::criar_e_gerenciar_threads;
// use persistencia::{EstadoParcial, EstadoParcial};
use soma_parcial::calcular_soma_parcial;
use std::sync::{Arc, Mutex};

fn main() {
    println!("Projeto 'soma-parelela' iniciado");

    // Exemplo de lista de numeros
    let numeros: Vec<i32> = (1..=100).collect();

    // Determinar o numero de threads
    let num_threads = num_cpus::get();
    println!("Numero de threads a serem usadas: {}", num_threads);

    // Dividir a lista
    let lista_dividida = dividir_lista(&numeros, num_threads);
    println!("Lista dividida em {} partes.", num_threads);

    // Armazenar as somas parciais de forma compartilhada
    let somas_parciais = Arc::new(Mutex::new(Vec::new()));
    let clone_somas_parciais = Arc::clone(&somas_parciais);

    // Criar e iniciar as threads
    let manipuladores = criar_e_gerenciar_threads(num_threads, move |i| {
        let sublista = &lista_dividida[i];
        let soma = calcular_soma_parcial(sublista);
        let mut somas = clone_somas_parciais.lock().unwrap();
        somas.push(soma);
    });

    for manipulador in manipuladores {
        manipulador.join().unwrap();
    }

    let somas = somas_parciais.lock().unwrap();
    let soma_total = combinar_resultados(&somas);
    println!("Soma total: {}", soma_total);
}
