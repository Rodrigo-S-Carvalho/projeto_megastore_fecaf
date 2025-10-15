#!/usr/bin/env python3
"""
Simulação de listagem de 10 milhões de produtos.
Mais eficiente possível usando apenas bibliotecas nativas Python.
"""

import time

def main():
    resposta = input("Iniciar a busca de todos os dez milhões de produtos? (S/N): ").strip().lower()
    if resposta != "s":
        print("Operação cancelada pelo usuário.")
        return

    print("Gerando e listando produtos...")
    inicio = time.perf_counter()

    # Usa list comprehension em vez de append em loop
    produtos = [
        f"Produto {i} | Marca XYZ | R$ {i * 0.37:.2f} | Estoque {i % 500}"
        for i in range(1, 10_000_001)
    ]

    # Exibe apenas os 10 primeiros produtos (como no Rust)
    for i, produto in enumerate(produtos[:10], start=1):
        print(f"{i}: {produto}")

    duracao = time.perf_counter() - inicio
    print(f"{len(produtos):,} itens encontrados. Busca realizada em {duracao:.3f} segundos.")


if __name__ == "__main__":
    main()
