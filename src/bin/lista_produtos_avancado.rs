use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

// Estrutura para representar um produto, como uma classe de produto
#[derive(Debug, Clone)]
struct Produto {
    codigo: String,
    nome: String,
}

// construtor para criar um novo produto
impl Produto {
    fn new(codigo: &str, nome: &str) -> Self {
        Produto {
            codigo: codigo.to_string(),
            nome: nome.to_string(),
        }
    }
}

// Função para carregar os produtos de um arquivo
//(produtos.txt no formato "codigo;nome", salvo no nível superior do projeto)
fn carregar_produtos(caminho: &str) -> Vec<Produto> {
    let mut produtos = Vec::new();

    if let Ok(file) = File::open(caminho) {
        let reader = io::BufReader::new(file);

        for linha in reader.lines() {
            if let Ok(linha) = linha {
                let partes: Vec<&str> = linha.splitn(2, ';').collect();
                if partes.len() == 2 {
                    let codigo = partes[0].trim();
                    let nome = partes[1].trim();
                    produtos.push(Produto::new(codigo, nome));
                }
            }
        }
    } else {
        println!("⚠️ Desculpe, mas houve um problema ao encontrar o arquivo dos dados. Não foi possível abrir o arquivo '{}'", caminho);
    }

    produtos
}

// função principal com menu interativo
fn main() {
    let caminho = "produtos.txt";
    let produtos = carregar_produtos(caminho);

    if produtos.is_empty() {
        println!("Desculpe, mas houve um erro no carregamento dos dados. Nenhum produto carregado. Verifique '{}'.", caminho);
        return;
    }

    // Criar índices para busca eficiente
    let mut indice_por_nome: HashMap<String, Produto> = HashMap::new();
    let mut indice_por_codigo: HashMap<String, Produto> = HashMap::new();

    for p in &produtos {
        indice_por_nome.insert(p.nome.to_lowercase().trim().to_string(), p.clone());
        indice_por_codigo.insert(p.codigo.clone(), p.clone());
    }

    // Vetores já ordenados para simplificar e otimizar a busca pelo usuário
    let mut ordenado_por_nome = produtos.clone();
    ordenado_por_nome.sort_by(|a, b| a.nome.to_lowercase().cmp(&b.nome.to_lowercase()));

    let mut ordenado_por_codigo = produtos.clone();
    ordenado_por_codigo.sort_by(|a, b| a.codigo.cmp(&b.codigo));

    // Menu interativo
    loop {
        println!("\n=== Menu MegaStore ({} produtos) ===", produtos.len());
        println!("1. Buscar produto por nome");
        println!("2. Buscar produto por código");
        println!("3. Listar produtos por ordem alfabética");
        println!("4. Listar produtos por letra inicial");
        println!("5. Listar produtos por código");
        println!("6. Sair");
        
        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).expect("Desculpe, mas algo deu errado na listagem do item. Erro ao ler entrada de dados.");
        let escolha = escolha.trim();

        //====== Opções do menu - adicionar AQUI novas opções e no loop acima deste nível ^ ====
        match escolha {
            "1" => buscar_por_nome(&indice_por_nome),
            "2" => buscar_por_codigo(&indice_por_codigo),
            "3" => listar(&ordenado_por_nome, "alfabética"),
            "4" => listar_por_letra_inicial(&ordenado_por_nome),
            "5" => listar(&ordenado_por_codigo, "código"),
            "6" => {
                println!("Saindo...");
                break;
            },            
            _ => println!("Desculpe, mas sua opção é inválida. Por favor, escolha opções entre 1 e 6."),
        }
    }
}

//encontrar produto por nome informado (ignorar caixa e espaços)
fn buscar_por_nome(indice: &HashMap<String, Produto>) {
    println!("Digite o nome do produto:");
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Desculpe, mas houve um erro ao listar este item.");

    let inicio = Instant::now();

    let chave = entrada.trim().to_lowercase();
    if let Some(produto) = indice.get(&chave) {
        println!("Encontrado: [{}] {}", produto.codigo, produto.nome);
    } else {
        println!("Desculpe, mas nenhum item com esse nome foi encontrado");
    }

    let duracao = inicio.elapsed();
    println!("Operação realizada em {:.6} segundos", duracao.as_secs_f64());
}

//encontrar produto por código informado (0001 a valor máximo de produtos)
fn buscar_por_codigo(indice: &HashMap<String, Produto>) {
    println!("Digite o código do produto (ex: 0001):");
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Desculpe, mas ouve algum erro ao listar o código deste produto.");

    let inicio = Instant::now();

    let chave = entrada.trim();
    if let Some(produto) = indice.get(chave) {
        println!("Encontrado: [{}] {}", produto.codigo, produto.nome);
    } else {
        println!("Desculpe, mas nenhum produto com este código foi encontrado.");
    }

    let duracao = inicio.elapsed();
    println!("Operação realizada em {:.6} segundos", duracao.as_secs_f64());
}

//listar todos os produtos por nome ou código, em ordem crescente
fn listar(lista: &[Produto], modo: &str) {
    let inicio = Instant::now();

    println!("Produtos em ordem {}:", modo);
    for (i, p) in lista.iter().enumerate() {
        println!("{}. [{}] {}", i + 1, p.codigo, p.nome);
    }

    let duracao = inicio.elapsed();
    println!(
        "Operação realizada em {:.6} segundos ({} itens listados)",
        duracao.as_secs_f64(),
        lista.len()
    );
}

//listar produtos que comecem com a letra informada
fn listar_por_letra_inicial(lista: &[Produto]) {
    println!("Listar itens que comecem com a letra:");
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro ao ler");

    let letra = entrada.trim().to_lowercase();

    if letra.len() != 1 {
        println!("⚠️ Digite apenas uma letra.");
        return;
    }

    let inicio = Instant::now();
    let mut encontrados = Vec::new();

    //numerar os resultados para facilitar a visualização
    for (i, p) in lista.iter().enumerate() {
        if let Some(primeira) = p.nome.chars().next() {
            if primeira.to_lowercase().to_string() == letra {
                encontrados.push((i + 1, p));
            }
        }
    }

    // mostrar resultados ou mensagem de nenhum encontrado
    if encontrados.is_empty() {
        println!("Nenhum produto encontrado com a letra '{}'", letra);
    } else {
        println!("Produtos que começam com '{}':", letra);
        for (i, p) in encontrados {
            println!("{}. [{}] {}", i, p.codigo, p.nome);
        }
    }

    let duracao = inicio.elapsed();
    println!("Operação realizada em {:.6} segundos", duracao.as_secs_f64());
}
