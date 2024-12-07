// Combina as somas parciais calculadas pelas threads para obter a soma total.
pub fn combinar_resultados(handles: Vec<std::thread::JoinHandle<i32>>) -> i32 {
    let mut soma_total = 0;

    for handle in handles {
        match handle.join() {
            Ok(soma_parcial) => soma_total += soma_parcial,
            Err(_) => {
                println!("Uma thread falhou ao calcular a soma parcial.");
                // Aqui você pode implementar lógica adicional para tratar falhas
            }
        }
    }

    soma_total
}
