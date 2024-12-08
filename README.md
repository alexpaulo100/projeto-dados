# Projeto Dados - Processamento de Vendas

Este projeto em Rust realiza o processamento de dados de vendas, filtrando e ordenando informações de vendas, com foco em dados como ID da venda, ID do vendedor, ID do cliente, data da venda e o total da venda. O objetivo é identificar vendas acima de um determinado valor e gerar relatórios filtrados e ordenados.

## Funcionalidades

- Ler um arquivo CSV contendo informações sobre vendas.
- Filtrar vendas que tenham um total superior a R$ 10.500,00.
- Ordenar os vendedores por total de vendas, de forma decrescente.
- Gerar um novo arquivo CSV com as vendas filtradas e ordenadas.
- Excluir arquivos de vendas do repositório com o auxílio de um `.gitignore`.

## Estrutura do Projeto

- **src/**: Contém o código-fonte principal do projeto.
- **vendas.csv**: Arquivo CSV de entrada contendo as informações de vendas.
- **vendas_filtradas_ordenadas.csv**: Arquivo CSV de saída com as vendas filtradas e ordenadas.
- **Cargo.toml**: Arquivo de configuração do Cargo (gerenciador de pacotes e build do Rust).
- **Cargo.lock**: Arquivo gerado automaticamente que bloqueia as versões de dependências.
- **.gitignore**: Arquivo de configuração para ignorar arquivos específicos, como CSVs.

## Dependências

Este projeto utiliza as seguintes dependências:

- **csv**: Biblioteca para leitura e escrita de arquivos CSV.
- **serde**: Biblioteca para deserialização e serialização de dados, usada para processar os registros CSV.

As dependências estão configuradas no arquivo `Cargo.toml`.

## Instalação

Para rodar o projeto, siga os passos abaixo:

1. **Clonar o Repositório**:

   Se você ainda não tem o repositório, clone-o para o seu computador:

   ```bash
   git clone https://github.com/alexpaulo100/projeto-dados.git
   cd projeto-dados
