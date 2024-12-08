# Programação Paralela: Soma de Números Usando Múltiplas Threads

Este projeto implementa um programa em Rust para calcular a soma de uma lista grande de números utilizando múltiplas threads. O objetivo principal é dividir a lista em partes, calcular a soma parcial em cada thread e combinar os resultados de forma eficiente e segura.

## Grupo
- Nomes dos integrantes:
``` txt
- ALVARO LIMA
- CAIO MARQUES
- DANILO MARCANO
- ENZO ABENSUR
- JULLIAN BRITO
- PEDRO VITORIANO
```


## Roadmap do Projeto

1. **Divisão da Lista**:
   - A lista de números inteiros é dividida em partes iguais (ou quase iguais), com base no número de threads disponíveis.

2. **Soma Paralela**:
   - Cada thread calcula a soma parcial de sua parte da lista.

3. **Configuração Dinâmica de Threads**:
   - O número de threads é configurado dinamicamente com base no número de núcleos do CPU ou de forma personalizada.

4. **Combinação de Resultados**:
   - Após o cálculo, as somas parciais são combinadas utilizando mecanismos seguros de controle de concorrência.

5. **Persistência e Tolerância a Falhas**:
   - O estado parcial da soma é salvo para permitir a recuperação em caso de falha.

6. **Testes e Otimizações**:
   - Testar o desempenho com diferentes tamanhos de listas e configurações de threads.
   - Verificar e otimizar a segurança do controle de concorrência.

---

## Dependências

As seguintes dependências são utilizadas no projeto:

- **serde**: Utilizado para serialização e desserialização de dados.
  Versão: `1.0`
  Recursos habilitados: `derive`

- **serde_json**: Biblioteca para manipulação de JSON.
  Versão: `1.0`

- **num_cpus**: Biblioteca para determinar o número de CPUs disponíveis.
  Versão: `1.13.0`

---

### Instruções

1. Clone o repositório:

   ```bash
   git clone <url-do-repositorio>
   cd <nome-do-repositorio>
   ```

2. Compile o projeto:

   ```bash
   cargo build --release
   ```

3. Execute o programa:

   ```bash
   cargo run --release
   ```

