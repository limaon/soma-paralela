// Divide uma lista de numeros em sublistas com base no numero de threads.
pub fn dividir_lista(numeros: &[i32], numero_threads: usize) -> Vec<Vec<i32>> {
    let tamanho = numeros.len();
    let mut sublistas = Vec::new();
    let tamanho_sublista = tamanho / numero_threads;

    for i in 0..numero_threads {
        let inicio = i * tamanho_sublista;
        let fim = if i == numero_threads - 1 {
            tamanho
        } else {
            inicio + tamanho_sublista
        };
        sublistas.push(numeros[inicio..fim].to_vec());
    }

    sublistas
}
