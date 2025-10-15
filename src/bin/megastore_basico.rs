use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

#[derive(Debug, Clone)]
struct Produto {
    codigo: String,
    nome: String,
}

impl Produto {
    fn new(codigo: &str, nome: &str) -> Self {
        Produto {
            codigo: codigo.to_string(),
            nome: nome.to_string(),
        }
    }
}

// 🔹 Carrega produtos de um arquivo TXT (formato: codigo;nome)
fn carregar_produtos(caminho: &str) -> Vec<Produto> {
    let mut produtos = Vec::new();

    if let Ok(file) = File::open(caminho) {
        let reader = io::BufReader::new(file);

        for linha in reader.lines() {
            if let Ok(linha) = linha {
                let partes: Vec<&str> = linha.splitn(2, ';').collect();
                if partes.len() == 2 {
                    produtos.push(Produto::new(partes[0].trim(), partes[1].trim()));
                }
            }
        }
    } else {
        println!("⚠️ Não foi possível abrir '{}'", caminho);
    }

    produtos
}

fn main() {
    let caminho = "produtos.txt";
    let mut produtos = carregar_produtos(caminho);

    if produtos.is_empty() {
        println!("Erro: nenhum produto carregado de '{}'.", caminho);
        return;
    }

    loop {
        println!("\n=== Menu MegaStore ({} produtos) ===", produtos.len());
        println!("1. Buscar produto por nome (parcial)");
        println!("2. Buscar produto por código");
        println!("3. Listar produtos por ordem alfabética");
        println!("4. Listar produtos por letra inicial");
        println!("5. Listar produtos por código");
        println!("6. Sair");

        let mut escolha = String::new();
        io::stdin()
            .read_line(&mut escolha)
            .expect("Erro ao ler entrada");
        let escolha = escolha.trim();

        match escolha {
            "1" => buscar_por_nome(&produtos),
            "2" => buscar_por_codigo(&produtos),
            "3" => listar_por_nome(&mut produtos.clone()),
            "4" => listar_por_letra_inicial(&produtos),
            "5" => listar_por_codigo(&mut produtos.clone()),
            "6" => {
                println!("Saindo...");
                break;
            }
            _ => println!("⚠️ Opção inválida. Escolha entre 1 e 6."),
        }
    }
}

// 🔹 Busca linear por nome (substring)
fn buscar_por_nome(produtos: &[Produto]) {
    println!("Digite parte do nome do produto:");
    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler entrada");
    let termo = entrada.trim().to_lowercase();

    if termo.is_empty() {
        println!("⚠️ Nenhum termo digitado.");
        return;
    }

    let inicio = Instant::now();
    let encontrados: Vec<&Produto> = produtos
        .iter()
        .filter(|p| p.nome.to_lowercase().contains(&termo))
        .collect();

    let duracao = inicio.elapsed().as_secs_f64();

    if encontrados.is_empty() {
        println!("Nenhum produto encontrado contendo '{}'.", termo);
    } else {
        println!("Produtos encontrados contendo '{}':", termo);
        for p in &encontrados {
            println!("[{}] {}", p.codigo, p.nome);
        }
    }

    println!(
        "✅ {} itens encontrados. Operação concluída em {:.6} segundos.",
        encontrados.len(),
        duracao
    );
}

// 🔹 Busca linear por código
fn buscar_por_codigo(produtos: &[Produto]) {
    println!("Digite o código (ex: 0472):");
    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler entrada");
    let codigo = entrada.trim();

    let inicio = Instant::now();
    let encontrados: Vec<&Produto> = produtos.iter().filter(|p| p.codigo == codigo).collect();
    let duracao = inicio.elapsed().as_secs_f64();

    if encontrados.is_empty() {
        println!("Nenhum produto encontrado com o código '{}'.", codigo);
    } else {
        for p in &encontrados {
            println!("Encontrado: [{}] {}", p.codigo, p.nome);
        }
    }

    println!(
        "✅ {} itens encontrados. Operação concluída em {:.6} segundos.",
        encontrados.len(),
        duracao
    );
}

// 🔹 Ordenação por nome
fn listar_por_nome(produtos: &mut Vec<Produto>) {
    let inicio = Instant::now();
    produtos.sort_by(|a, b| a.nome.to_lowercase().cmp(&b.nome.to_lowercase()));

    for (i, p) in produtos.iter().enumerate() {
        println!("{}. [{}] {}", i + 1, p.codigo, p.nome);
    }

    println!(
        "✅ {} itens listados. Operação concluída em {:.6} segundos.",
        produtos.len(),
        inicio.elapsed().as_secs_f64()
    );
}

// 🔹 Ordenação por código
fn listar_por_codigo(produtos: &mut Vec<Produto>) {
    let inicio = Instant::now();
    produtos.sort_by(|a, b| a.codigo.cmp(&b.codigo));

    for (i, p) in produtos.iter().enumerate() {
        println!("{}. [{}] {}", i + 1, p.codigo, p.nome);
    }

    println!(
        "✅ {} itens listados. Operação concluída em {:.6} segundos.",
        produtos.len(),
        inicio.elapsed().as_secs_f64()
    );
}

// 🔹 Listar produtos por letra inicial
fn listar_por_letra_inicial(produtos: &[Produto]) {
    println!("Digite a letra inicial:");
    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler entrada");
    let letra = entrada.trim().to_lowercase();

    if letra.len() != 1 {
        println!("⚠️ Digite apenas uma letra.");
        return;
    }

    let inicio = Instant::now();
    let encontrados: Vec<&Produto> = produtos
        .iter()
        .filter(|p| p.nome.to_lowercase().starts_with(&letra))
        .collect();
    let duracao = inicio.elapsed().as_secs_f64();

    if encontrados.is_empty() {
        println!("Nenhum produto encontrado começando com '{}'.", letra);
    } else {
        println!("Produtos que começam com '{}':", letra);
        for (i, p) in encontrados.iter().enumerate() {
            println!("{}. [{}] {}", i + 1, p.codigo, p.nome);
        }
    }

    println!(
        "✅ {} itens encontrados. Operação concluída em {:.6} segundos.",
        encontrados.len(),
        duracao
    );
}
