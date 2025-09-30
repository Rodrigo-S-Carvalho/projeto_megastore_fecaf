use std::io;
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

fn main() {
    // Lista fixa de produtos
    let produtos = vec![
        Produto::new("0001", "Martelo"),
        Produto::new("0002", "Chave de Fenda"),
        Produto::new("0003", "Alicate"),
        Produto::new("0004", "Parafuso"),
        Produto::new("0005", "Prego"),
        Produto::new("0006", "Serrote"),
        Produto::new("0007", "Trena"),
        Produto::new("0008", "Nível de Bolha"),
        Produto::new("0009", "Broca"),
        Produto::new("0010", "Cola de Madeira"),
    ];

    loop {
        println!("\n=== Menu Ferragem ===");
        println!("1. Buscar produto por nome");
        println!("2. Buscar produto por código (ex: 1 para 0001)");
        println!("3. Listar produtos por ordem alfabética");
        println!("4. Listar produtos por código");
        println!("5. Sair");

        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).expect("Erro ao ler entrada");
        let escolha = escolha.trim();

        match escolha {
            "1" => buscar_por_nome(&produtos),
            "2" => buscar_por_codigo(&produtos),
            "3" => listar_por_nome(&produtos),
            "4" => listar_por_codigo(&produtos),
            "5" => {
                println!("Saindo...");
                break;
            }
            _ => println!("Opção inválida!"),
        }
    }
}

//pesquisar produto por nome (ignorar caixa e espaços)
fn buscar_por_nome(produtos: &[Produto]) {
    println!("Digite o nome do produto:");
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro ao ler");

    let inicio = Instant::now(); // inicia o cronômetro

    let entrada = entrada.trim().to_lowercase(); // remove espaços extras e ignora caixa
    let mut encontrados = vec![];

    for p in produtos {
        if p.nome.to_lowercase().contains(&entrada) {
            encontrados.push(p);
        }
    }

    if encontrados.is_empty() {
        println!("Nenhum produto encontrado!");
    } else {
        println!("Produtos encontrados:");
        for p in encontrados {
            println!("[{}] {}", p.codigo, p.nome);
        }
    }

    let duracao = inicio.elapsed();
    println!("Operação realizada em {:.6} segundos", duracao.as_secs_f64());
}

//pesquisar produto por código (1 ao valor máximo de produtos)
fn buscar_por_codigo(produtos: &[Produto]) {
    println!("Digite o número do produto (1 a 10):");
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro ao ler");

    let inicio = Instant::now(); // inicia o cronômetro

    if let Ok(num) = entrada.trim().parse::<usize>() {
        if num >= 1 && num <= produtos.len() {
            let p = &produtos[num - 1];
            println!("Encontrado: [{}] {}", p.codigo, p.nome);
        } else {
            println!("Código inválido!");
        }
    } else {
        println!("Entrada inválida!");
    }

    let duracao = inicio.elapsed();
    println!("Operação realizada em {:.6} segundos", duracao.as_secs_f64());
}

//listar todos os produtos por nome, em ordem alfabética
fn listar_por_nome(produtos: &[Produto]) {
    let inicio = Instant::now(); // inicia o cronômetro

    let mut lista = produtos.to_vec();
    lista.sort_by(|a, b| a.nome.to_lowercase().cmp(&b.nome.to_lowercase()));

    println!("Produtos em ordem alfabética:");
    for p in lista {
        println!("[{}] {}", p.codigo, p.nome);
    }

    let duracao = inicio.elapsed();
    println!("Operação realizada em {:.6} segundos", duracao.as_secs_f64());
}

//listar todos os produtos por código, em ordem crescente
fn listar_por_codigo(produtos: &[Produto]) {
    let inicio = Instant::now(); // inicia o cronômetro

    let mut lista = produtos.to_vec();
    lista.sort_by(|a, b| a.codigo.cmp(&b.codigo));

    println!("Produtos por código:");
    for p in lista {
        println!("[{}] {}", p.codigo, p.nome);
    }

    let duracao = inicio.elapsed();
    println!("Operação realizada em {:.6} segundos", duracao.as_secs_f64());
}
