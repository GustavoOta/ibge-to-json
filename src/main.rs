use calamine::{open_workbook, Reader, Xls}; // Importa as bibliotecas necessárias para ler arquivos Excel
use serde::Serialize; // Importa a biblioteca para serialização de dados
use std::collections::HashMap; // Importa a biblioteca para usar HashMap
use std::fs::File; // Importa a biblioteca para manipulação de arquivos
use std::io::Write; // Importa a biblioteca para escrita em arquivos

#[derive(Serialize)]
struct Municipio {
    uf: String,        // Unidade Federativa (Estado)
    codigo: String,    // Código do município
    municipio: String, // Nome do município
}

fn main() {
    // Abre o arquivo Excel
    let mut excel: Xls<_> = open_workbook("RELATORIO_DTB_BRASIL_MUNICIPIO.xls").unwrap();
    // Obtém a planilha específica dentro do arquivo Excel
    let range = excel.worksheet_range("DTB_2022_Municipio").unwrap();

    // Índices das colunas A, L e M (começando do zero)
    let coluna_a_index = 0; // Índice da coluna A (UF)
    let codigo_index = 11; // Índice da coluna L (Código do município)
    let nome_index = 12; // Índice da coluna M (Nome do município)

    // Cria um HashMap para agrupar os municípios por UF
    let mut municipios_por_uf: HashMap<String, Vec<Municipio>> = HashMap::new();

    // Itera sobre as linhas da planilha, pulando as primeiras 7 linhas
    for row in range.rows().skip(7) {
        // Obtém o código da UF da coluna A
        let uf_code = row[coluna_a_index].to_string();
        // Converte o código da UF para a sigla correspondente
        let uf = match uf_code.as_str() {
            "11" => "RO".to_string(),
            "12" => "AC".to_string(),
            "13" => "AM".to_string(),
            "14" => "RR".to_string(),
            "15" => "PA".to_string(),
            "16" => "AP".to_string(),
            "17" => "TO".to_string(),
            "21" => "MA".to_string(),
            "22" => "PI".to_string(),
            "23" => "CE".to_string(),
            "24" => "RN".to_string(),
            "25" => "PB".to_string(),
            "26" => "PE".to_string(),
            "27" => "AL".to_string(),
            "28" => "SE".to_string(),
            "29" => "BA".to_string(),
            "31" => "MG".to_string(),
            "32" => "ES".to_string(),
            "33" => "RJ".to_string(),
            "35" => "SP".to_string(),
            "41" => "PR".to_string(),
            "42" => "SC".to_string(),
            "43" => "RS".to_string(),
            "50" => "MS".to_string(),
            "51" => "MT".to_string(),
            "52" => "GO".to_string(),
            "53" => "DF".to_string(),
            _ => uf_code, // Caso não encontre a UF, usa o código original
        };
        // Obtém o código do município da coluna L
        let codigo = row[codigo_index].to_string();
        // Obtém o nome do município da coluna M
        let municipio = row[nome_index].to_string();

        // Cria uma instância de Municipio
        let municipio = Municipio {
            uf: uf.clone(),
            codigo,
            municipio,
        };

        // Adiciona o município ao HashMap, agrupando por UF
        municipios_por_uf
            .entry(uf)
            .or_insert_with(Vec::new)
            .push(municipio);
    }

    // Itera sobre o HashMap para salvar os municípios em arquivos JSON separados por UF
    for (uf, municipios) in municipios_por_uf {
        // Converte a lista de municípios para JSON
        let json = serde_json::to_string_pretty(&municipios).unwrap();
        // Verifica se o diretório "jsons" existe, se não, cria o diretório
        let is_path_exists = std::path::Path::new("./jsons").exists();
        if !is_path_exists {
            std::fs::create_dir("./jsons").unwrap();
        }
        // Cria um arquivo JSON com o nome da UF
        let mut file = File::create(format!("./jsons/{}.json", uf)).unwrap();
        // Escreve o JSON no arquivo
        file.write_all(json.as_bytes()).unwrap();
    }
}
