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

    // Pré-aloca memória para 10 milhões de produtos (melhor performance)
    let mut produtos: Vec<String> = Vec::with_capacity(10_000_000);

    // Gera produtos simulados
    for i in 1..=10_000_000 {
        produtos.push(format!(
            "Produto {} | Marca XYZ | R$ {:.2} | Estoque {}",
            i,
            i as f64 * 0.37,
            i % 500
        ));
    }

    // Exibe os 10 primeiros apenas (para não travar o terminal)
    for (i, produto) in produtos.iter().take(10).enumerate() {
        println!("{}: {}", i + 1, produto);
    }

    let duracao = inicio.elapsed().as_secs_f64();

    println!(
        "{} itens encontrados. Busca realizada em {:.3} segundos.",
        produtos.len(),
        duracao
    );
}
