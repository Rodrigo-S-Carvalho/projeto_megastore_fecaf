#!/usr/bin/env python3
"""
lista_produtos_linear.py
Versão *intencionalmente ineficiente* do sistema MegaStore.

Todas as operações são lineares (O(n)):
- Nenhum índice ou estrutura ordenada.
- Nenhum uso de dict, bisect, set ou sort prévio.
- Todas as buscas percorrem toda a lista de produtos a cada operação.
"""

import os
import time
from dataclasses import dataclass
from typing import List

PRODUTOS_FILE = "produtos.txt"  # arquivo no mesmo diretório


@dataclass
class Produto:
    codigo: str
    nome: str


def carregar_produtos(caminho: str) -> List[Produto]:
    produtos: List[Produto] = []
    if not os.path.isfile(caminho):
        print(f"⚠️ Não foi possível abrir '{caminho}' (arquivo não encontrado).")
        return produtos

    try:
        with open(caminho, "r", encoding="utf-8") as f:
            for linha in f:
                linha = linha.strip()
                if not linha or ";" not in linha:
                    continue
                partes = linha.split(";", 1)
                codigo = partes[0].strip()
                nome = partes[1].strip()
                produtos.append(Produto(codigo, nome))
    except Exception as e:
        print(f"Erro ao ler '{caminho}': {e}")
    return produtos


def buscar_por_nome(produtos: List[Produto]):
    termo = input("Digite parte do nome do produto: ").strip().lower()
    if termo == "":
        print("⚠️ Nenhum termo digitado.")
        return

    inicio = time.perf_counter()
    encontrados = []

    # busca linear: percorre todos os produtos
    for p in produtos:
        if termo in p.nome.lower():
            encontrados.append(p)

    if not encontrados:
        print(f"Nenhum produto encontrado contendo '{termo}'")
    else:
        print(f"Produtos encontrados contendo '{termo}':")
        for p in encontrados:
            print(f"[{p.codigo}] {p.nome}")

    print(f"Operação concluída em {time.perf_counter() - inicio:.6f} segundos")


def buscar_por_codigo(produtos: List[Produto]):
    codigo = input("Digite o código (ex: 0472): ").strip()
    inicio = time.perf_counter()
    encontrado = None

    # busca linear simples
    for p in produtos:
        if p.codigo == codigo:
            encontrado = p
            break

    if encontrado:
        print(f"Encontrado: [{encontrado.codigo}] {encontrado.nome}")
    else:
        print("Nenhum produto encontrado com este código.")
    print(f"Operação concluída em {time.perf_counter() - inicio:.6f} segundos")


def listar_por_nome(produtos: List[Produto]):
    inicio = time.perf_counter()

    # ordenação ineficiente (cria uma cópia e usa sorted manualmente)
    ordenado = sorted(produtos, key=lambda p: p.nome.lower())

    print("Produtos em ordem alfabética:")
    for i, p in enumerate(ordenado, start=1):
        print(f"{i}. [{p.codigo}] {p.nome}")

    print(f"Operação concluída em {time.perf_counter() - inicio:.6f} segundos ({len(ordenado)} itens)")


def listar_por_codigo(produtos: List[Produto]):
    inicio = time.perf_counter()

    ordenado = sorted(produtos, key=lambda p: p.codigo)

    print("Produtos em ordem de código:")
    for i, p in enumerate(ordenado, start=1):
        print(f"{i}. [{p.codigo}] {p.nome}")

    print(f"Operação concluída em {time.perf_counter() - inicio:.6f} segundos ({len(ordenado)} itens)")


def listar_por_letra_inicial(produtos: List[Produto]):
    letra = input("Digite a letra inicial: ").strip().lower()
    if len(letra) != 1:
        print("⚠️ Digite apenas uma letra.")
        return

    inicio = time.perf_counter()
    encontrados = []

    # loop linear para checar cada item
    for p in produtos:
        if p.nome.lower().startswith(letra):
            encontrados.append(p)

    if not encontrados:
        print(f"Nenhum produto encontrado com '{letra}'")
    else:
        print(f"Produtos que começam com '{letra}':")
        for i, p in enumerate(encontrados, start=1):
            print(f"{i}. [{p.codigo}] {p.nome}")

    print(f"Operação concluída em {time.perf_counter() - inicio:.6f} segundos")


def main():
    caminho = PRODUTOS_FILE
    produtos = carregar_produtos(caminho)

    if not produtos:
        print(f"Erro: nenhum produto carregado de '{caminho}'.")
        return

    while True:
        print(f"\n=== Menu MegaStore ({len(produtos)} produtos) ===")
        print("1. Buscar produto por nome (parcial, lenta)")
        print("2. Buscar produto por código (lenta)")
        print("3. Listar produtos por ordem alfabética (lenta)")
        print("4. Listar produtos por letra inicial (lenta)")
        print("5. Listar produtos por código (lenta)")
        print("6. Sair")

        escolha = input("Escolha: ").strip()
        if escolha == "1":
            buscar_por_nome(produtos)
        elif escolha == "2":
            buscar_por_codigo(produtos)
        elif escolha == "3":
            listar_por_nome(produtos)
        elif escolha == "4":
            listar_por_letra_inicial(produtos)
        elif escolha == "5":
            listar_por_codigo(produtos)
        elif escolha == "6":
            print("Saindo...")
            break
        else:
            print("⚠️ Opção inválida. Escolha entre 1 e 6.")


if __name__ == "__main__":
    main()
