use std::collections::BTreeMap;
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

// 🔹 Carrega produtos do arquivo produtos.txt (formato: "codigo;nome")
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
    let produtos = carregar_produtos(caminho);

    if produtos.is_empty() {
        println!("Erro: nenhum produto carregado de '{}'.", caminho);
        return;
    }

    // Índice de busca por código usando BTreeMap
    let mut indice_por_codigo: BTreeMap<String, Produto> = BTreeMap::new();
    for p in &produtos {
        indice_por_codigo.insert(p.codigo.clone(), p.clone());
    }

    // Índice de busca por nome (prefixos) usando BTreeMap
    let mut indice_por_nome: BTreeMap<String, Produto> = BTreeMap::new();
    for p in &produtos {
        indice_por_nome.insert(p.nome.to_lowercase(), p.clone());
    }

    // Vetores ordenados
    let mut ordenado_por_nome = produtos.clone();
    ordenado_por_nome.sort_by(|a, b| a.nome.to_lowercase().cmp(&b.nome.to_lowercase()));

    let mut ordenado_por_codigo = produtos.clone();
    ordenado_por_codigo.sort_by(|a, b| a.codigo.cmp(&b.codigo));

    // Menu
    loop {
        println!("\n=== Menu MegaStore ({} produtos) ===", produtos.len());
        println!("1. Buscar produto por nome (parcial)");
        println!("2. Buscar produto por código");
        println!("3. Listar produtos por ordem alfabética");
        println!("4. Listar produtos por letra inicial");
        println!("5. Listar produtos por código");
        println!("6. Sair");

        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).expect("Erro ao ler entrada");
        let escolha = escolha.trim();

        match escolha {
            "1" => buscar_por_nome_prefixo(&indice_por_nome),
            "2" => buscar_por_codigo(&indice_por_codigo),
            "3" => listar(&ordenado_por_nome, "alfabética"),
            "4" => listar_por_letra_inicial(&ordenado_por_nome),
            "5" => listar(&ordenado_por_codigo, "código"),
            "6" => {
                println!("Saindo...");
                break;
            }
            _ => println!("⚠️ Opção inválida. Escolha entre 1 e 6."),
        }
    }
}

// 🔹 Busca otimizada por nome (prefixo parcial) usando BTreeMap
fn buscar_por_nome_prefixo(indice: &BTreeMap<String, Produto>) {
    println!("Digite parte do nome do produto (prefixo):");
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro ao ler entrada");
    let chave = entrada.trim().to_lowercase();

    if chave.is_empty() {
        println!("⚠️ Nenhum termo digitado.");
        return;
    }

    let inicio = Instant::now();

    let prefixo_fim = format!("{}~", chave);
    let mut encontrados = Vec::new();

    for (_, produto) in indice.range(chave.clone()..prefixo_fim) {
        encontrados.push(produto);
    }

    let duracao = inicio.elapsed().as_secs_f64();

    if encontrados.is_empty() {
        println!("Nenhum produto encontrado contendo '{}'", chave);
    } else {
        println!("Produtos encontrados contendo '{}':", chave);
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

// 🔹 Busca exata por código usando BTreeMap
fn buscar_por_codigo(indice: &BTreeMap<String, Produto>) {
    println!("Digite o código (ex: 0472):");
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro ao ler entrada");
    let chave = entrada.trim();

    let inicio = Instant::now();
    let mut encontrados = Vec::new();

    if let Some(produto) = indice.get(chave) {
        encontrados.push(produto);
    }

    let duracao = inicio.elapsed().as_secs_f64();

    if encontrados.is_empty() {
        println!("Nenhum produto encontrado com este código.");
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

// 🔹 Listagem completa por nome ou código
fn listar(lista: &[Produto], modo: &str) {
    let inicio = Instant::now();
    println!("Produtos em ordem {}:", modo);

    for (i, p) in lista.iter().enumerate() {
        println!("{}. [{}] {}", i + 1, p.codigo, p.nome);
    }

    let duracao = inicio.elapsed().as_secs_f64();

    println!(
        "✅ {} itens listados. Operação concluída em {:.6} segundos.",
        lista.len(),
        duracao
    );
}

// 🔹 Listar produtos por letra inicial
fn listar_por_letra_inicial(lista: &[Produto]) {
    println!("Digite a letra inicial:");
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro ao ler entrada");
    let letra = entrada.trim().to_lowercase();

    if letra.len() != 1 {
        println!("⚠️ Digite apenas uma letra.");
        return;
    }

    let inicio = Instant::now();
    let encontrados: Vec<&Produto> = lista
        .iter()
        .filter(|p| p.nome.to_lowercase().starts_with(&letra))
        .collect();

    let duracao = inicio.elapsed().as_secs_f64();

    if encontrados.is_empty() {
        println!("Nenhum produto encontrado com '{}'", letra);
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
