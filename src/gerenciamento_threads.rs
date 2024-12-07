use std::thread;

// Gerencia a criacao e execucao das threads para calcular a soma parcial.
pub fn gerenciar_threads(sublistas: Vec<Vec<i32>>) -> Vec<thread::JoinHandle<i32>> {
    let mut handles = Vec::new();

    for sublista in sublistas {
        let handle = thread::spawn(move || calcular_soma_parcial(&sublista));
        handles.push(handle);
    }

    handles
}

// Calcula a soma de uma sublista de numeros.
fn calcular_soma_parcial(sublista: &[i32]) -> i32 {
    sublista.iter().sum()
}
