// Modulo responsavel por dividir a lista de numeros em partes menores.

pub fn dividir_lista(numeros: &[i32], partes: usize) -> Vec<&[i32]> {
    let tamanho_do_pedaco = numeros.len() / partes;
    let mut dividida = Vec::with_capacity(partes);

    for i in 0..partes {
        let inicio = i * tamanho_do_pedaco;
        let fim = if i == partes - 1 {
            numeros.len()
        } else {
            inicio + tamanho_do_pedaco
        };
        dividida.push(&numeros[inicio..fim]);
    }

    dividida
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn teste_dividir_lista() {
        let numeros = vec![1, 2, 3, 4, 5, 6];
        let partes = 2;
        let dividida = dividir_lista(&numeros, partes);
        assert_eq!(dividida.len(), 2);
        assert_eq!(dividida[0], &[1, 2, 3]);
        assert_eq!(dividida[1], &[4, 5, 6]);
    }
}
