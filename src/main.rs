
use fanotify::{low_level::*,  high_level::*};
use hasher::get_file_hash;
use std::{fmt::LowerHex, fs::metadata, path::{Path, PathBuf}, thread};

mod hasher;

fn main() {
/* ###### PERMISSÃO ##### para o binario ter privilegio similiar de root: sudo setcap cap_sys_admin=eip rhunter

###### Leitura dos eventos ################
CloseNowrite -> abriu e não escreveu( posso ignorar)
Quando abre o arquivo cria arquivo.txt.swp, e nele aparece modify e depois deleted.
            


    
 */ 
    let primary_path = PathBuf::from("/home/fabio/teste_av");
    let ft = Fanotify::new_nonblocking(FanotifyMode::NOTIF).expect("Error regitering fanotify listener");
    let _real_path = ft.add_path(
        FAN_ACCESS | FAN_CLOSE | FAN_EVENT_ON_CHILD | FAN_MODIFY | FAN_ONDIR,
        &primary_path,
    ).unwrap();

    let mut diretorios:Vec<PathBuf> = vec![primary_path];
    

    loop {
        let eventos = ft.read_event();
        for events in eventos {
            let write = FanEvent::CloseWrite;    
            let file_path = PathBuf::from(&events.path);
            if events.events.contains(&write) && !events.path.contains("swp") && file_path.is_file()
            {

                println!("Arquivo alterado: {:?}", events.path );
               
                   let hsh =  match get_file_hash(&file_path)
                   {
                        Ok(hex) => println!("hash: {hex}"),
                        Err(e) => println!("Couldnt hash the file")
                    };
                

            } else if file_path.is_dir() && !diretorios.contains(&file_path){ // Need to be a dir and not inside the vector.
                    
                println!("Novo diretorio adicionado ao monitoramento: {:?}", file_path);
                let new_path = PathBuf::from(&events.path);
                diretorios.push(new_path.clone());
                ft.add_path(
                    FAN_ACCESS | FAN_CLOSE | FAN_EVENT_ON_CHILD | FAN_MODIFY | FAN_ONDIR,
                    &file_path,
                ).unwrap();
                
                
            } else {
                println!("ignorado: {:?}", events)
            }
            
        }

    }

    
    }
