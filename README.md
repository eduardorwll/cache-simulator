# cache-simulator

Simulador de cache em Rust para a disciplina AOCII (UFPEL). Suporta cache L1 separada
para instruções e dados, com uma L2 unificada opcional.

## Requisitos

É necessário ter o [Rust](https://www.rust-lang.org/) instalado. A forma recomendada
de instalação é pelo [rustup](https://rustup.rs/), o instalador oficial do Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Compilação

O Cargo tem dois modos de compilação:

- **debug** (padrão): compila rápido, mas o binário roda mais lento. Ideal durante o desenvolvimento.
  ```bash
  cargo build
  ```
- **release**: compila mais devagar, mas gera um binário otimizado. Recomendado para uso "de verdade".
  ```bash
  cargo build --release
  ```

Os binários ficam em `target/debug/cache-simulator` e `target/release/cache-simulator`,
respectivamente.

## Uso

A forma mais prática de rodar é com o `cargo run` (lembrando do `--` para separar os
argumentos do programa dos argumentos do próprio cargo):

```bash
cargo run -- <CONFIG_L1_INSTRUÇÕES> <CONFIG_L1_DADOS> [CONFIG_L2] <ARQUIVO>
```

- O **último argumento** é sempre o caminho para o arquivo de entrada (benchmark).
- Os argumentos anteriores são as configurações das caches:
  - **2 configs**: L1 de instruções + L1 de dados, sem L2;
  - **3 configs**: as duas L1s + L2 unificada.

### Configuração de cache

Cada cache é configurada no formato `<nsets>:<bsize>:<assoc>`:

| Campo | Significado                         | Exemplo |
| ----- | ----------------------------------- | ------- |
| nsets | número de conjuntos                 | 1024    |
| bsize | tamanho do bloco, em bytes          | 16      |
| assoc | associatividade (vias por conjunto) | 4       |

Ou seja, `1024:16:4` = 1024 conjuntos, blocos de 16 bytes, 4 vias.

## Exemplos

```bash
# Apenas L1 (instruções + dados), sem L2
cargo run -- 256:16:4 256:16:4 benchmark.bin

# L1 + L2 unificada
cargo run -- 256:16:4 256:16:4 4096:32:8 benchmark.bin

# Usando o binário compilado em modo release
cargo run --release -- 1024:32:8 1024:32:8 8192:64:16 benchmark.bin
```

## Arquivo de entrada

Arquivo binário contendo endereços de 32 bits, um a cada 4 bytes. O endereçamento é
feito a byte.
