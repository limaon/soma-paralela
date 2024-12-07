/// Modulo responsavel pelo calculo da soma parcial.
pub fn calcular_soma_parcial(numeros: &[i32]) -> i32 {
    numeros.iter().sum()
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn testar_calcular_soma_parcial() {
        let numeros = vec![1, 2, 3, 4];
        let soma = calcular_soma_parcial(&numeros);
        assert_eq!(soma, 10);
    }
}
