/// Modulo responsavel por combinar as somas parciais.
pub fn combinar_resultados(somas_parciais: &[i32]) -> i32 {
    somas_parciais.iter().sum()
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn testar_combinar_resultados() {
        let somas_parciais = vec![10, 20, 30];
        let total = combinar_resultados(&somas_parciais);
        assert_eq!(total, 60);
    }
}
