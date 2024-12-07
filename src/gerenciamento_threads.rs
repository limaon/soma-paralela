use std::thread;

/// Módulo responsável por criar e gerenciar threads.
pub fn criar_e_gerenciar_threads<F, T>(
    numero_de_threads: usize,
    tarefa: F,
) -> Vec<thread::JoinHandle<T>>
where
    F: Fn(usize) -> T + Send + Sync + 'static,
    T: Send + 'static,
{
    let mut manipuladores = Vec::with_capacity(numero_de_threads);
    let tarefa = std::sync::Arc::new(tarefa);

    for i in 0..numero_de_threads {
        let tarefa_clonada = std::sync::Arc::clone(&tarefa);
        let manipulador = thread::spawn(move || tarefa_clonada(i));
        manipuladores.push(manipulador);
    }

    manipuladores
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn testar_criar_e_gerenciar_threads() {
        let numero_de_threads = 4;
        let manipuladores = criar_e_gerenciar_threads(numero_de_threads, |i| i * 2);

        let resultados: Vec<_> = manipuladores
            .into_iter()
            .map(|h| h.join().unwrap())
            .collect();
        assert_eq!(resultados, vec![0, 2, 4, 6]);
    }
}
