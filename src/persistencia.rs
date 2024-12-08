// src/persistencia.rs

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

// Estrutura que representa o estado do programa.
#[derive(Serialize, Deserialize, Debug)]
pub struct Estado {
    // Soma total acumulada ate o momento.
    pub soma_total: i32,
    // Somas parciais calculadas por cada thread. `None` indica que a sublista ainda nao foi processada.
    pub somas_parciais: Vec<Option<i32>>,
}

impl Estado {
    // Cria um novo estado inicial.
    pub fn novo(numero_threads: usize) -> Self {
        Estado {
            soma_total: 0,
            somas_parciais: vec![None; numero_threads],
        }
    }
}

// Salva o estado parcial em um arquivo JSON.
pub fn salvar_estado(estado: &Estado, caminho: &str) -> io::Result<()> {
    let arquivo = File::create(caminho)?;
    serde_json::to_writer_pretty(arquivo, estado)?;
    Ok(())
}

// Carrega o estado parcial de um arquivo JSON, se existir.
pub fn carregar_estado(caminho: &str) -> io::Result<Estado> {
    if !Path::new(caminho).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Arquivo de estado nao encontrado",
        ));
    }

    let arquivo = File::open(caminho)?;
    let estado = serde_json::from_reader(arquivo)?;
    Ok(estado)
}
