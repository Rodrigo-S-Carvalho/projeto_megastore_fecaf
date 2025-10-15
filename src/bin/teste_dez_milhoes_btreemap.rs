use std::collections::BTreeMap;
use std::io;
use std::time::Instant;

fn main() {
    println!("Iniciar a busca de todos os dez milhões de produtos? (S/N)");

    let mut resposta = String::new();
    io::stdin().read_line(&mut resposta).expect("Erro na leitura");

    let resposta = resposta.trim().to_lowercase();
    if resposta != "s" {
        println!("Operação cancelada pelo usuário.");
        return;
    }

    println!("Gerando e listando produtos...");

    let inicio = Instant::now();
    let mut produtos: BTreeMap<u32, String> = BTreeMap::new();

    // Gera 10 milhões de produtos simulados
    for i in 1..=10_000_000 {
        let nome = format!("Produto {} | Marca XYZ | R$ {:.2} | Estoque {}", i, i as f64 * 0.37, i % 500);
        produtos.insert(i, nome);
    }

    // Exibe os 10 primeiros só para visualização (evita flood no terminal)
    for (id, descricao) in produtos.iter().take(10) {
        println!("{}: {}", id, descricao);
    }

    let duracao = inicio.elapsed().as_secs_f64();

    println!(
        "{} itens encontrados. Busca realizada em {:.3} segundos.",
        produtos.len(),
        duracao
    );
}
