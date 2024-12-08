extern crate csv;
extern crate serde;

use csv::{ReaderBuilder, WriterBuilder};
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Record {
    idvenda: u32,
    idvendedor: u32,
    idcliente: u32,
    data: String,
    total: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Caminho para o arquivo CSV de entrada e saída
    let input_file_path = "vendas.csv";
    let output_file_path = "vendas_filtradas_ordenadas.csv";

    // Abre o arquivo CSV para leitura
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';') // Define o delimitador como ponto e vírgula
        .has_headers(true) // Assume que o arquivo CSV tem cabeçalhos
        .from_path(input_file_path)?;

    // Coleta os registros que atendem à condição (vendas > 10500)
    let mut filtered_records: Vec<Record> = Vec::new();

    for result in rdr.deserialize::<Record>() {
        match result {
            Ok(record) => {
                // Filtra as vendas com total superior a 10500
                if record.total > 10500.0 {
                    filtered_records.push(record);
                }
            }
            Err(e) => {
                eprintln!("Erro ao deserializar a linha: {}", e);
            }
        }
    }

    // Ordena os registros filtrados por idvendedor de forma decrescente
    filtered_records.sort_by(|a, b| b.idvendedor.cmp(&a.idvendedor));

    // Imprime os registros filtrados e ordenados na tela
    println!("Vendas maiores que R$ 10.500 (ordenadas por idvendedor):");
    for record in &filtered_records {
        println!(
            "IDVenda: {}, IDVendedor: {}, IDCliente: {}, Data: {}, Total: R$ {:.2}",
            record.idvenda, record.idvendedor, record.idcliente, record.data, record.total
        );
    }
    // Abre o arquivo CSV de saída para escrita
    let mut wtr = WriterBuilder::new()
        .delimiter(b';') // Mantém o delimitador como ponto e vírgula
        .from_path(output_file_path)?;

    // Escreve os cabeçalhos no arquivo de saída
    wtr.write_record(&["idvenda", "idvendedor", "idcliente", "data", "total"])?;

    // Escreve os registros filtrados e ordenados no arquivo de saída
    for record in filtered_records {
        wtr.write_record(&[
            record.idvenda.to_string(),
            record.idvendedor.to_string(),
            record.idcliente.to_string(),
            record.data,
            record.total.to_string(),
        ])?;
    }

    println!(
        "Processamento concluído! Arquivo salvo em {}",
        output_file_path
    );
    Ok(())
}
