// Std 
// use std::env;
use std::fs::File;
use std::io::{Read, BufReader};

// Self
use crate::ParameterSets;

#[test]
fn change_map(){

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
                            Ok(mut values)=>{
                                println!("Deserialized: {:?}", values);

                                let parameter_set = values.get_parameter_set("set-name_0".to_string());
                                match parameter_set{
                                    Ok(parameters)=>{
                                        let params_name = parameters.get_name();
                                        let params_kvs = parameters.get_key_values();
                                        println!("Name: {:?}", params_name);
                                        let prev_values = values.clone();
                                        values.set_parameter_set(params_name.clone(), params_kvs.clone());
                                        
                                        if values != prev_values{
                                            assert_eq!(true, true);
                                        }
                                        else{
                                            assert_eq!(false, true);
                                        }
                                        
                                        // values.set_parameter_set(key, values)

                                    }
                                    Err(_)=>{

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
