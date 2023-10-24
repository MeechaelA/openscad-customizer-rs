
use serde::{Serialize, Deserialize};
use serde_json::{Value, Map};
use std::vec::Vec;

// Std 
// use std::env;
use std::fs::File;
use std::io::{Read, BufReader};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParameterSets{
    parameter_sets: Map<String, Value>,
    file_format_version: String
}

impl ParameterSets{
    pub fn new(sets: Vec<ParameterSet>, version: String) -> ParameterSets{
        let mut map_tranlsation = Map::new();
        for set in sets{
            map_tranlsation.insert(set.name, serde_json::Value::Object(set.key_values));
        }

        ParameterSets{
            parameter_sets: map_tranlsation,
            file_format_version: version
        }
    }

    pub fn from_file_path(path: String)->Result<ParameterSets, String>{
        let data = File::open(path);

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
                                    return Ok(values);
                                }
                                Err(_)=>{
                                    let error = String::from("Unable to Deserialize");
                                    return Err(error);
                                }
                            }
                        }
                        else{
                            let error = String::from("Unable to Deserialize");
                            return Err(error);
                        }                 
                    }
                    Err(error_contents)=>{
                        let error = error_contents.to_string();
                        return Err(error);
                    }
                }   
            }
            Err(stream_err)=>{
                return Err(stream_err.to_string());

            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ParameterSet{
    name: String,
    key_values: Map<String, Value>
}

#[cfg(test)]
mod test;