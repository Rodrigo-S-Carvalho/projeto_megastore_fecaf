# projeto_megastore_fecaf
Sistema de catalogo de produtos da loja fictícia MegaStore
A versão pronta para uso está na pasta App MegaStore Versão de Produção
Basta abrir o arquivo executável para Windows

Na primeira linha da interface da aplicação, além do nome do estabelecimento, MegaStore, você tem, entre parênteses, a quantidade atual de produtos em estoque:

=== Menu MegaStore (500 produtos) ===

Em seguita, você terá seis opções de menu, sendo elas:
1. Buscar produto por nome
2. Buscar produto por código
3. Listar produtos por ordem alfabética
4. Listar produtos por letra inicial
5. Listar produtos por código
6. Sair

Na opção 1, você pode digitar o nome exato e completo de um produto ou apenas parte do nome, em busca parcial. Detalhe: se você pesuisar, por exemplo, apenas por "or", todos os produtos que tiverem a combinação "or" em qualquer parte dos seus nomes serão listados. Essa opção não diferencia maiúsculas e minúsculas.

Exemplo da pesquisa "serr":

1    
Digite parte do nome do produto:
serr
Produtos encontrados contendo 'serr':
[0359] Serra Circular 5"
[0360] Serra Circular 6"
[0129] Serra Circular 7.1/4"
[0357] Serra Fita 3"
[0358] Serra Fita 4"
[0128] Serra Mármore 110mm
[0130] Serra Tico-Tico
[0006] Serrote de Madeira
[0086] Serrote Poda 12"
[0472] Serrote Poda 14"
Operação em 0.004701 segundos

"Desculpe, mas nenhum produto contendo 'zwzw' foi encontrado" é um exemplo da mensagem que você recebe, caso se a busca que você fizer pelo nome completo ou parte do nome de um produto não encontrar nenhum nome ou parte de nome compatível.

Note como tanto nesta opção quanto nas demais, exceto na opção 6, você tem o tempo em segundos que o comando levou para ser executado:

"Operação em 0.004701 segundos" foi o tempo de execução do comando do exemplo anterior.

Esse tempo ajuda você a detectar possíveis atrasos e pode ajudar no diagnóstico de possíveis problemas de desempenho do sistema.

Na opção 2, buscar produto por código, você precisa digitar exatamente o código do produto. Em vez de digitar apenas "3" para encontrar  o código "0003", por exemplo, você deve pesquisar exatamente por "0003". Isso evita possíveis confusões resultados "0003", "0030", "0300" ou "3000", onde todos poderiam acabar sendo listados. A pesquisa por código é ideal para quando você já sabe o código exato de um produto. Para pesquisas parciais, prefira a opção 1.

A opção 3, listar produtos por ordem alfabética, não é uma opção de pesquisa, mas sim a listagem de todos os produtos, em ordem alfabética. Dependendo da quantidade de produtos, ela pode levar um pouco mais de tempo para retornar todos os itens. Prefira utilizar esta opção apenas para revisões manuais de todo o estoque atual. Para busca de apenas alguns itens específicos, prefira as opções 1 e 2.

A opção 4, listar produtos por letra inicial, permite ao usuário listar todos os produtos que comecem com a letra do alfabeto informada. Exemplo de pesquisa com a letra f (não diferencia maiúsculas e minúsculas):

Digite a letra inicial:
f 
Produtos que começam com 'f':
1. [0169] Faqueiro Inox 24 Peças
2. [0460] Fechadura Porta Externa
3. [0074] Fechadura Porta Interna
4. [0283] Filtro Antena TV
5. [0302] Filtro de Linha 10 Tomadas
6. [0102] Filtro de Linha 5 Tomadas
7. [0330] Filtro Tela 1/2
8. [0331] Filtro Tela 3/4
9. [0154] Filtro Torneira Universal
10. [0027] Fita Crepe 18mm
etc.

Observação. Use apenas essa opção para pesquisar itens por letra inicial. Apenas uma letra é permitida na busca. Tentar usar a opção 1 para o mesmo fim retorna quaisquer itens contendo a letra informada, em qualquer parte dos seus nomes. Exemplo:

1
Digite parte do nome do produto:
F
Produtos encontrados contendo 'f':
[0278] Adaptador HDMI Macho/Fêmea
[0227] Afiador Facas Manual
[0338] Aspersor Fixo
[0241] Bloco Anotações 100fl
[0277] Cabo Força Tripolar
[0239] Caderno Universitário 200fl
[0184] Cafeteira Alumínio Italiana 6x
[0276] Caixa Fósforo 10un
etc.

A opção 5, listar produtos por código, lista os produtos por seus códigos numéricos, em ordem crescente, conforme os códigos com os quais foram catalogados no sistema. Exemplo:

Produtos em ordem código:
1. [0001] Martelo
2. [0002] Chave de Fenda
3. [0003] Alicate Universal
4. [0004] Parafuso Aço Zincado
5. [0005] Prego Comum
6. [0006] Serrote de Madeira
7. [0007] Trena 3m
8. [0008] Nível de Bolha 30cm
9. [0009] Broca Aço Rápido 6mm
10. [0010] Cola de Madeira
11. [0011] Chave Inglesa
12. [0012] Chave Allen Jogo
etc.

Observação. O código do produto é a numeração de quatro dígitos como "[0001]". A numeração sequencial à esquerda é apenas uma forma de visualização mais organizada do próprio aplicativo.

Por fim, a opção 6 é aquela com a qual você encerra o funcionamento da aplicação, sem confirmação.

Exceto pela opção 6, todas as opções anteriores retornam ao menu principal depois de o comando anterior ter sido concluído. Exemplo:

=== Menu MegaStore (500 produtos) ===
1. Buscar produto por nome (parcial)
2. Buscar produto por código
3. Listar produtos por ordem alfabética
4. Listar produtos por letra inicial
5. Listar produtos por código
6. Sair
2
Digite o código (ex: 0472):
0005
Encontrado: [0005] Prego Comum
Operação em 0.000293 segundos

=== Menu MegaStore (500 produtos) ===
1. Buscar produto por nome (parcial)
2. Buscar produto por código
3. Listar produtos por ordem alfabética
4. Listar produtos por letra inicial
5. Listar produtos por código
6. Sair

Note como a opção 2 foi escolhida no exemplo acima, o código 0005 foi pesquisado, seu resultado exibido e, em seguida, o menu está pronto para receber um novo comando.
