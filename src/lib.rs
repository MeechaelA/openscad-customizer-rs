
use serde::{Serialize, Deserialize};
use serde_json::{Value, Map};
use std::vec::Vec;

// Std 
// use std::env;
use std::fs::File;
use std::io::{Read, BufReader};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
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
    pub fn get_parameter_sets(&self)->Result<&Map<String, Value>, String>{
        if self.parameter_sets.len() > 0{
            return Ok(&self.parameter_sets);
        }
        return Err(String::from("No Parameter Sets"));
    }
    pub fn set_parameter_sets(&mut self, sets: Vec<ParameterSet>){
        let mut map_tranlsation = Map::new();
        for set in sets{
            map_tranlsation.insert(set.name, serde_json::Value::Object(set.key_values));
        }
        self.parameter_sets = map_tranlsation;
    }
    
    pub fn get_parameter_set(&self, key: String)->Result<&ParameterSet, String>{
        let key_values = self.parameter_sets.get(&key);
        let mut error = Err(String::from("Error..."));
        match key_values{
            Some(map) => {
                let value = map.as_object();
                match value{
                    Some(map_value)=>{
                        let mut sets = vec![];
                        for value in map_value{
                            let key_values = value.1.as_object();
                            match key_values{
                                Some(key_value_map)=>{
                                    let set = ParameterSet::new(value.0.to_string(), key_value_map.clone()); 
                                    sets.push(set);
                                }
                                None=>{
                                    error = Err(String::from("Errors with parameter map."))
                                }
                            }
                        }
                    }
                    None=>{
                        error = Err(String::from("Parameter set has no values."));
                    }
                }
            }
            None=>{
                error = Err(String::from("Key Value Not Found"))
            }
        }
        return error;
    }
    pub fn set_parameter_set(&mut self, key: String, values: Map<String, Value>){
        let sets = self.parameter_sets.get_mut(&key);
        match sets{
            Some(set_values)=>{
                let set_values_as_object = set_values.as_object_mut();
                match set_values_as_object{
                    Some(values_object)=>{
                        values_object.insert(key, Value::Object(values));
                    }
                    None=>{
                        println!("Couldn't set...");
                    }
                }
            }
            None=>{
                println!("Couldn't set...");
            }
        }

    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct ParameterSet{
    name: String,
    key_values: Map<String, Value>
}
impl ParameterSet{
    pub fn new(name: String, key_values: Map<String, Value>)->ParameterSet{
        ParameterSet { 
            name: name, 
            key_values: key_values 
        }
    }
    pub fn get_name(&self)->&String{
        return &self.name;
    }
    pub fn get_key_values(&self)->&Map<String, Value>{
        return &self.key_values;
    }

}

#[cfg(test)]
mod test;