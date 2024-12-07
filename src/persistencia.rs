use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

/// Estrutura para armazenar o estado parcial.
#[derive(Serialize, Deserialize, Debug)]
pub struct EstadoParcial {
    pub somas_parciais: Vec<i32>,
}

impl EstadoParcial {
    pub fn novo() -> Self {
        EstadoParcial {
            somas_parciais: Vec::new(),
        }
    }

    /// Salva o estado parcial em um arquivo.
    pub fn salvar(&self, nome_do_arquivo: &str) -> io::Result<()> {
        let serializado = serde_json::to_string(&self)?;
        let mut arquivo = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(nome_do_arquivo)?;
        arquivo.write_all(serializado.as_bytes())
    }

    /// Carrega o estado parcial de um arquivo.
    pub fn carregar(nome_do_arquivo: &str) -> io::Result<Self> {
        let mut arquivo = File::open(nome_do_arquivo)?;
        let mut conteudo = String::new();
        arquivo.read_to_string(&mut conteudo)?;
        let estado = serde_json::from_str(&conteudo)?;
        Ok(estado)
    }
}

#[cfg(test)]
mod testes {
    use super::*;
    use std::fs;

    #[test]
    fn testar_salvar_e_carregar() {
        let estado = EstadoParcial {
            somas_parciais: vec![10, 20, 30],
        };
        let nome_do_arquivo = "test_state.json";
        estado.salvar(nome_do_arquivo).unwrap();

        let estado_carregado = EstadoParcial::carregar(nome_do_arquivo).unwrap();
        assert_eq!(estado_carregado.somas_parciais, vec![10, 20, 30]);

        // Limpeza após o teste
        fs::remove_file(nome_do_arquivo).unwrap();
    }
}
