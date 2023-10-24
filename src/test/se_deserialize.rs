// Std 
// use std::env;
use std::fs::File;
use std::io::{Read, BufReader};

use serde::de::Error;

// Self
use crate::{ParameterSets};

// #[test]
// fn get_current_working_directory(){
    // let cwd = env::current_dir();
    // match cwd{
        // Ok(contents)=>{
            // println!("Contents: {:?}", contents);
            // assert_eq!(true, true);
        // }
        // Err(_)=>{
            // assert_eq!(false, true);
        // }
    // }
    // assert_eq!(false, true);
// }

#[test]
fn deserialize_format(){

    let data = File::open("data/format.json");

    match data{
        Ok(stream)=>{
            let mut buf_reader = BufReader::new(stream);
            let mut contents = String::new();
            let buf_reader_result = buf_reader.read_to_string(&mut contents);

            match buf_reader_result{
                Ok(contents_length)=>{
                    if contents_length > 0{
                        println!("Contents: {:?}", contents);
                        let deserialized: Result<ParameterSets, _> = serde_json::from_str(&contents);
                        match deserialized{
                            Ok(values)=>{
                                println!("Deserialized: {:?}", values);

                                let message = serde_json::to_string_pretty(&values);
                                match message{
                                    Ok(message_contents) => {
                                        let mut valid = false;
                                        if message_contents.contains("parameterSets") && message_contents.contains("fileFormatVersion"){
                                            println!("Serialized: {:?}", message_contents);
                                            valid = true;
                                        }
                                        else{
                                            valid = false;
                                        }
                                        assert_eq!(true, valid);        
                                    }
                                    Err(_) => {

                                    }
                                }
                            }
                            Err(_)=>{
                                println!("Unable to deserialize: {:?}", deserialized);
                                assert_eq!(false, true);        
                            }
                        }
                    }
                    else{
                        assert_eq!(false, true);        
                        println!("No Content.");
                    }                 
                }
                Err(error)=>{
                    println!("Could not find {}.", error);
                    assert_eq!(false, true);        
                }
            }

            println!("Contents: {}", contents);
            
        }
        Err(stream_err)=>{
            println!("Could not find {}.", stream_err);
            assert_eq!(false, true);
        }
    }
}
#[test]

fn serialize_format(){

    let data = File::open("data/format.json");

    match data{
        Ok(stream)=>{
            let mut buf_reader = BufReader::new(stream);
            let mut contents = String::new();
            let buf_reader_result = buf_reader.read_to_string(&mut contents);

            match buf_reader_result{
                Ok(contents_length)=>{
                    if contents_length > 0{
                        println!("Contents: {:?}", contents);
                        let deserialized: Result<ParameterSets, _> = serde_json::from_str(&contents);
                        match deserialized{
                            Ok(values)=>{
                                println!("Deserialized: {:?}", values);
                                assert_eq!(true, true);        
                            }
                            Err(_)=>{
                                println!("Unable to deserialize: {:?}", deserialized);
                                assert_eq!(false, true);        
                            }
                        }
                    }
                    else{
                        assert_eq!(false, true);        
                        println!("No Content.");
                    }                 
                }
                Err(error)=>{
                    println!("Could not find {}.", error);
                    assert_eq!(false, true);        
                }
            }

            println!("Contents: {}", contents);
            
        }
        Err(stream_err)=>{
            println!("Could not find {}.", stream_err);
            assert_eq!(false, true);
        }
    }
}