# IBGE to JSON

Este projeto converte dados de um arquivo Excel fornecido pelo IBGE em arquivos JSON separados por Unidade Federativa (UF).

## Descrição

O script em Rust lê um arquivo Excel (`RELATORIO_DTB_BRASIL_MUNICIPIO.xls`) que contém informações sobre municípios brasileiros. Ele agrupa os municípios por UF e salva cada grupo em um arquivo JSON separado, nomeado com a sigla da UF.

## Como Funciona

1. **Leitura do Arquivo Excel**: O script utiliza a biblioteca `calamine` para abrir e ler o arquivo Excel.
2. **Processamento dos Dados**: Os dados são processados e agrupados por UF utilizando um `HashMap`.
3. **Criação dos Arquivos JSON**: Para cada UF, é criado um arquivo JSON contendo os municípios pertencentes àquela UF. Os arquivos são salvos no diretório `jsons`.

## Estrutura do Código

- **Estrutura Municipio**: Define a estrutura dos dados de um município.
- **Função main**: Realiza a leitura do arquivo Excel, processamento dos dados e criação dos arquivos JSON.

## Como Executar

1. **Pré-requisitos**: Certifique-se de ter o Rust instalado em sua máquina.
2. **Clone o Repositório**:
    ```sh
    git clone https://github.com/seu-usuario/ibge-to-json.git
    cd ibge-to-json
    ```
3. **Adicione o Arquivo Excel**: Coloque o arquivo `RELATORIO_DTB_BRASIL_MUNICIPIO.xls` no diretório raiz do projeto.
4. **Compile e Execute**:
    ```sh
    cargo run
    ```
5. **Verifique os Arquivos JSON**: Os arquivos JSON serão gerados no diretório `jsons`.

## Exemplo de Uso

Após executar o script, você encontrará arquivos JSON no diretório `jsons`, como `RO.json`, `AC.json`, etc. Cada arquivo conterá uma lista de municípios pertencentes àquela UF.

## Contribuição

Contribuições são bem-vindas! Sinta-se à vontade para abrir issues e pull requests.

## Licença

Este projeto está licenciado sob a Licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.