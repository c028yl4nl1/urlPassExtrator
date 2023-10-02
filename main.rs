extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use serde_json::{json, Value};
use std::process::exit;

// Função para adicionar uma chave e um objeto a ela
fn adicionar_chave_e_objeto(json: &mut Value, chave: &str, objeto: Value) {
    if json.get_mut(chave).is_none() {
        json[chave] = json!([objeto]);
    } else if let Some(chave_existente) = json.get_mut(chave) {
        // Verifique se o objeto já existe no array antes de adicioná-lo
        if !chave_existente.as_array_mut().unwrap().contains(&objeto) {
            chave_existente.as_array_mut().unwrap().push(objeto);
        }
    }
}

#[derive(Debug)]
struct JsonBuffer {
    chave: String,
    user: String,
    pass: String,
    caminho: String,
}

impl JsonBuffer {
    pub fn new(chave: String, user: String, pass: String , caminho: String) -> JsonBuffer {
        JsonBuffer { chave, user, pass , caminho}
    }
}

fn main() {
    let mut json_str = r#"{
        "teste":[
            {
                "user": "lanby",
                "pass": "lanbyteste",
                "caminho":"/"
            }]
       
    }"#;
    // Analisa a string JSON em um valor JSON
    let mut json: Value = serde_json::from_str(json_str).unwrap();

    let file_path = "saida.json";
    let file = File::create(file_path).expect("Erro ao criar o arquivo de saída");
    let mut writer = BufWriter::new(file);

    let mut files_to_read:Vec<String> = Vec::new();

    let nome_principal = "logs"; // Sem o .txt
    let range = 12;
    for arquivos in 0..range{
        let arquivo = format!("{}{}.txt",nome_principal.to_string(),arquivos.to_string());
        files_to_read.push(arquivo);
    }
 
    for file_name in &files_to_read {
        println!("Estou no arquivo: {}", file_name); 
        match File::open(file_name) {
            Ok(file) => {
                let reader = BufReader::new(file);
                for linha in reader.lines() {
                    if let Some(valor) = FuncaoparaSeparaOsdados(linha.unwrap().as_str()) {
                        let chave = &valor.chave;
                        let login_add = json!({
                            "user": &valor.user,
                            "pass": &valor.pass,
                            "caminho": &valor.caminho
                        });
                        {

                        
                        adicionar_chave_e_objeto(&mut json, chave, login_add);
                      

                        }
                    }
                }
            }
            Err(_) => {
                println!("Arquivo {} não encontrado, continuando...", file_name);
            }
        }
    }

    serde_json::to_writer_pretty(&mut writer, &json).expect("Erro ao escrever JSON no arquivo");
    println!("JSON salvo em '{}'", file_path);
}

pub fn Arquivo(path: &str) -> BufReader<File> {
    let file = File::open(path).expect("Erro ao abrir o arquivo");
    BufReader::new(file)
}

fn FuncaoparaSeparaOsdados(linha: &str) -> Option<JsonBuffer> {
    let a: Vec<&str> = linha.split(":").collect();
    let tam = a.len();
    if a[0].to_uppercase().contains("HTTPS") || a[0].to_uppercase().contains("HTTP"){
        // faça nada
    }
    else {
        return None;
    }

    //println!("{:?}", a);
    if tam < 1 {
        return None;
    } else if tam >= 4 && tam < 5 {
        let url = a[1];
        //println!("{}", url);
        let host_Apenas:  Vec<&str>  = url.split("/").collect();
        //println!("{:?}", host_Apenas);
        if host_Apenas.len() < 2 {
            return None;
        }
      {}
        let caminho = {
            let a = host_Apenas.get(3..);
            if let Some(valor) = a {
            format!("/{}",valor[0..].join("/")) // Acessando o primeiro elemento do slice
            } else {
                "/".to_string()
            }
        };
        //println!("{}", caminho);
        let url = host_Apenas.get(2).unwrap();
      
        let user = a.get(2).unwrap();
        let pass = a.get(3).unwrap();
       

        return Some(JsonBuffer {
            chave: url.to_string(),
            user: user.to_string(),
            pass: pass.to_string(),
            caminho: caminho,
        });
    } else {
        return None;
    }
}
