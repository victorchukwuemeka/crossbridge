use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::error::Error;

pub struct BurnTracker {
    file_path: PathBuf,
}

impl BurnTracker{
    pub fn new()-> Self{
        let mut path = std::env::current_exe()
            .unwrap_or_else(|_| PathBuf::from("."))
            .parent()
            .unwrap_or(&PathBuf::from("."))
            .to_path_buf();
        path.push("processed_burns.txt");

        Self { file_path: path }
    } 

    pub fn processed(&self, tx_hash:&str) -> bool{
        if let Ok(file) = File::open(&self.file_path){
            let reader = BufReader::new(file);
            for line in reader.lines(){
                if let Ok(hash) = line{
                    if hash.trim() == tx_hash {
                        return true ;
                    }
                }
            }
        }
       return false;
    }

    pub fn mark_processed(&self, tx_hash:&str)->Result<bool, Box<dyn Error>>{
        let mut file = match OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path){
                Ok(file)=>{
                    println!("file processed");
                    file
                }
                Err(e)=>{
                    println!("file not processed");
                    return Err(e.into())
                }
            };
        match writeln!(file, "{}", tx_hash){
            Ok(write)=>{
                println!("writing the transaction hash into the file");
                write
            }
            Err(e)=>{
                println!("could not write into the file");
                return Err(e.into());
            }
        };
        Ok(true)
    }

    pub fn can_process(&self, tx_hash:&str)->Result<bool, Box<dyn Error>>{
        if self.processed(tx_hash){
            println!("Burn :{} has already been processed", tx_hash);
            return Ok(false);
        }
        

        self.mark_processed(tx_hash);
        Ok(true)
    }
}

impl Default for BurnTracker {
    fn default() -> Self {
        Self::new()
    }
}