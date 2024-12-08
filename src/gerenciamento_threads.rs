// use std::fs;
// use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::persistencia::{salvar_estado, Estado};

// Gerencia a criacao e execucao das threads para calcular a soma parcial.
// Atualiza o estado compartilhado após cada cálculo e salva o estado no arquivo.
pub fn gerenciar_threads(
    sublistas: Vec<Vec<i32>>,
    estado_arc: Arc<Mutex<Estado>>,
    caminho_estado: String,
) -> Vec<thread::JoinHandle<()>> {
    let mut handles = Vec::new();

    for (i, sublista) in sublistas.into_iter().enumerate() {
        let estado_clone = Arc::clone(&estado_arc);
        let caminho_clone = caminho_estado.clone();

        let handle = thread::spawn(move || {
            // Verificar se esta sublista ja foi processada
            {
                let estado = estado_clone.lock().unwrap();
                if estado.somas_parciais[i].is_some() {
                    println!("Sublista {} ja foi processada. Pulando...", i);
                    return;
                }
            }

            // Calcular a soma parcial
            let soma_parcial: i32 = sublista.iter().sum();
            println!("Sublista {} - Soma Parcial: {}", i, soma_parcial);

            // Atualizar o estado compartilhado
            {
                let mut estado = estado_clone.lock().unwrap();
                estado.somas_parciais[i] = Some(soma_parcial);
                estado.soma_total += soma_parcial;

                // Salvar o estado atualizado no arquivo
                if let Err(e) = salvar_estado(&estado, &caminho_clone) {
                    eprintln!("Erro ao salvar o estado: {}", e);
                }
            }
        });

        handles.push(handle);
    }

    handles
}
