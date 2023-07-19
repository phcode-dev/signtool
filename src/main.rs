use std::env;
use std::process::{Command, exit};
use std::fs::File;
use std::io::{BufReader};
use serde_json::{Value};

fn read_secrets() -> Value {
    let binding = env::current_dir().unwrap();
    let path = binding.to_str().unwrap();
    let file_full_path = path.to_owned() + "\\..\\secrets.json";
    let file = File::open(file_full_path).unwrap();
    let reader = BufReader::new(file);
    let u = serde_json::from_reader(reader).unwrap();
    return u;
}


fn sign(file_name: &str, time_stamp_url: &str) {
    println!("started signing");

    let secrets = read_secrets();

    let cert_name = secrets["AZURE_CERT_NAME"].to_string().replace("\"", "");
    let client_id = secrets["AZURE_CLIENT_ID"].to_string().replace("\"", "");

    let client_secret = secrets["AZURE_CLIENT_SECRET"].to_string().replace("\"", "");

    let key_vault_uri = secrets["AZURE_KEY_VAULT_URI"].to_string().replace("\"", "");

    let tenant_id = secrets["AZURE_TENANT_ID"].to_string().replace("\"", "");
    let company_name = secrets["AZURE_COMPANY_NAME"].to_string().replace("\"", "");

    let output = Command::new("AzureSignTool")
        .arg("sign")
        .arg("-kvu")
        .arg(key_vault_uri)
        .arg("-kvi")
        .arg(client_id)
        .arg("-kvt")
        .arg(tenant_id)
        .arg("-kvs")
        .arg(client_secret)
        .arg("-kvc")
        .arg(cert_name)
        .arg("-tr")
        .arg(time_stamp_url)
        .arg("-v")
        .arg(file_name)
        .arg("-d")
        .arg(company_name)
        .output()
        .expect("failed to execute process");
    if !output.stderr.is_empty() {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
    }
    if !output.stdout.is_empty() {
        eprint!("{}", String::from_utf8_lossy(&output.stdout));
    }
    println!("completed signing");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("length {}", args.len());
    if args.len() <= 10 {
        exit(1)
    }
    let time_stamp_url = &args[9];
    let file_name_to_sign = &args[10];
    println!("signing  {}", file_name_to_sign);
    sign(file_name_to_sign, time_stamp_url);
}