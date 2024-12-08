use std::thread;

// Combina as somas parciais calculadas pelas threads para obter a soma total.
pub fn combinar_resultados(handles: Vec<thread::JoinHandle<()>>) -> i32 {
    // Aguardamos todas as threads completarem
    for handle in handles {
        if let Err(e) = handle.join() {
            eprintln!("Uma thread falhou: {:?}", e);
        }
    }

    // Apos todas as threads completarem, recuperamos o estado salvo
    match crate::persistencia::carregar_estado("estado.json") {
        Ok(estado) => estado.soma_total,
        Err(e) => {
            eprintln!("Erro ao carregar o estado final: {}", e);
            0
        }
    }
}
