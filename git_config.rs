use std::io;
use std::io::prelude::*;
use std::process::Command;

fn main(){
    let mut stdin = io::stdin();

    let descobrir_usuario = {
        Command::new("powershell")
            .args(["/C", "whoami"])
            .output()
            .expect("failed to execute process")
    };

    let diretorio_usuario = format!("{:?}", String::from_utf8_lossy(&descobrir_usuario.stdout));

    let nome_usuario: Vec<_> = diretorio_usuario.split(r"\").map(str::to_string).collect();
    
    println!("Seja bem vindo {}!",nome_usuario[2]);

    let comando_ssh_keygen = format!(r###"ssh-keygen -t rsa -C "{}" -f C:\Users\"{}"\.ssh\{} -N '""'"###, nome_usuario[2].clone(), nome_usuario[2].clone(), nome_usuario[2].clone());

    let _cria_chave_ssh = {
        Command::new("powershell")
            .args(["/C", &comando_ssh_keygen])
            .output()
            .expect("failed to execute process")
    };

    let local_chave = format!(r###"cat C:\Users\"{}"\.ssh\{}.pub"###, nome_usuario[2].clone(), nome_usuario[2].clone());

    let pega_chave = {
        Command::new("powershell")
            .args(["/C", &local_chave])
            .output()
            .expect("failed to execute process")
    };

    let chave = format!("{:?}", String::from_utf8_lossy(&pega_chave.stdout));
    let mut chave_ssh = chave.replace(&['\"'][..], "");
    let mut contador:i32 = 0;
    loop{
        contador = contador + 1;
        // retirando os ultimos caracteres pois está vindo coisas a mais
        chave_ssh.pop();

        if contador == 4{
            break;
        }
    }
    
    println!("Pegue a chave abaixo copie e cole nas configurações da plataforma de controle de versão");
    println!("\n");
    println!("{}", chave_ssh);

    // recebimento de string qualquer para programa não fechar
    let _ = stdin.read(&mut [0u8]).unwrap();
}