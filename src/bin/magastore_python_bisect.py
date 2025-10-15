#!/usr/bin/env python3
"""
lista_produtos.py
Versão Python equivalente ao sistema Rust que usava BTreeMap.
Funcionalidades:
1 - Buscar produto por nome (prefixo parcial, eficiente via bisect em lista ordenada)
2 - Buscar produto por código (hash map/dict)
3 - Listar produtos por ordem alfabética
4 - Listar produtos por letra inicial
5 - Listar produtos por código
6 - Sair
"""

from dataclasses import dataclass
from typing import List, Tuple, Dict
import time
import bisect
import sys
import os

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
                if not linha or linha.startswith("#"):
                    continue
                partes = linha.split(";", 1)
                if len(partes) != 2:
                    continue
                codigo = partes[0].strip()
                nome = partes[1].strip()
                produtos.append(Produto(codigo=codigo, nome=nome))
    except Exception as e:
        print(f"Erro ao ler '{caminho}': {e}")
    return produtos


def construir_indices(produtos: List[Produto]):
    # índice por código (dict)
    indice_por_codigo: Dict[str, Produto] = {p.codigo: p for p in produtos}

    # índice por nome: lista ordenada de (nome_lower, Produto)
    lista_nome_ordenada: List[Tuple[str, Produto]] = sorted(
        ((p.nome.lower(), p) for p in produtos), key=lambda x: x[0]
    )

    # para buscar por código ordenado (lista de produtos ordenada por código)
    ordenado_por_codigo = sorted(produtos, key=lambda p: p.codigo)

    # para listar por nome (apenas lista de Produto ordenada por nome)
    ordenado_por_nome = [p for (_n, p) in lista_nome_ordenada]

    return indice_por_codigo, lista_nome_ordenada, ordenado_por_nome, ordenado_por_codigo


def buscar_por_nome_prefixo(lista_nome_ordenada: List[Tuple[str, Produto]]):
    termo = input("Digite parte do nome do produto (prefixo): ").strip().lower()
    if termo == "":
        print("⚠️ Nenhum termo digitado.")
        return

    inicio = time.perf_counter()

    # intervalo [termo, termo + high_char)
    # usamos um caractere alto para delimitar o fim do prefixo
    termo_fim = termo + chr(0x10FFFF)

    # extrai apenas as chaves (nomes) para bisect
    nomes = [n for (n, _p) in lista_nome_ordenada]

    left = bisect.bisect_left(nomes, termo)
    right = bisect.bisect_right(nomes, termo_fim)

    encontrados = [lista_nome_ordenada[i][1] for i in range(left, right)]

    if not encontrados:
        print(f"Nenhum produto encontrado contendo prefixo '{termo}'")
    else:
        print(f"Produtos encontrados contendo prefixo '{termo}':")
        for p in encontrados:
            print(f"[{p.codigo}] {p.nome}")

    print(f"Operação concluída em {time.perf_counter() - inicio:.6f} segundos")


def buscar_por_codigo(indice_por_codigo: Dict[str, Produto]):
    chave = input("Digite o código (ex: 0472): ").strip()
    inicio = time.perf_counter()
    produto = indice_por_codigo.get(chave)
    if produto:
        print(f"Encontrado: [{produto.codigo}] {produto.nome}")
    else:
        print("Nenhum produto encontrado com este código.")
    print(f"Operação concluída em {time.perf_counter() - inicio:.6f} segundos")


def listar(ordenado_por_nome: List[Produto], modo: str):
    inicio = time.perf_counter()
    print(f"Produtos em ordem {modo}:")
    for i, p in enumerate(ordenado_por_nome, start=1):
        print(f"{i}. [{p.codigo}] {p.nome}")
    print(
        f"Operação concluída em {time.perf_counter() - inicio:.6f} segundos ({len(ordenado_por_nome)} itens)"
    )


def listar_por_letra_inicial(ordenado_por_nome: List[Produto]):
    letra = input("Digite a letra inicial: ").strip().lower()
    if len(letra) != 1:
        print("⚠️ Digite apenas uma letra.")
        return
    inicio = time.perf_counter()
    encontrados = [p for p in ordenado_por_nome if p.nome.lower().startswith(letra)]
    if not encontrados:
        print(f"Nenhum produto encontrado com '{letra}'")
    else:
        print(f"Produtos que começam com '{letra}':")
        for i, p in enumerate(encontrados, start=1):
            print(f"{i}. [{p.codigo}] {p.nome}")
    print(f"Operação concluída em {time.perf_counter() - inicio:.6f} segundos")


def listar_por_codigo(ordenado_por_codigo: List[Produto]):
    inicio = time.perf_counter()
    print("Produtos por código:")
    for i, p in enumerate(ordenado_por_codigo, start=1):
        print(f"{i}. [{p.codigo}] {p.nome}")
    print(
        f"Operação concluída em {time.perf_counter() - inicio:.6f} segundos ({len(ordenado_por_codigo)} itens)"
    )


def main():
    caminho = PRODUTOS_FILE
    produtos = carregar_produtos(caminho)
    if not produtos:
        print(f"Erro: nenhum produto carregado de '{caminho}'.")
        return

    (
        indice_por_codigo,
        lista_nome_ordenada,
        ordenado_por_nome,
        ordenado_por_codigo,
    ) = construir_indices(produtos)

    while True:
        print(f"\n=== Menu MegaStore ({len(produtos)} produtos) ===")
        print("1. Buscar produto por nome (prefixo)")
        print("2. Buscar produto por código")
        print("3. Listar produtos por ordem alfabética")
        print("4. Listar produtos por letra inicial")
        print("5. Listar produtos por código")
        print("6. Sair")

        escolha = input("Escolha: ").strip()
        if escolha == "1":
            buscar_por_nome_prefixo(lista_nome_ordenada)
        elif escolha == "2":
            buscar_por_codigo(indice_por_codigo)
        elif escolha == "3":
            listar(ordenado_por_nome, "alfabética")
        elif escolha == "4":
            listar_por_letra_inicial(ordenado_por_nome)
        elif escolha == "5":
            listar_por_codigo(ordenado_por_codigo)
        elif escolha == "6":
            print("Saindo...")
            break
        else:
            print("⚠️ Opção inválida. Escolha entre 1 e 6.")


if __name__ == "__main__":
    main()
