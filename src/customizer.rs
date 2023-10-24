use serde::{Serialize, Deserialize};
use serde_json::{Value, Map, Number};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ParameterSets{
    sets: Map<String, Value>,
    version: FileFormatVersion
}


#[derive(Serialize, Deserialize, Debug, Default)]
struct ParameterSet{
    name: String,
    key_values: Map<String, Value>
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct FileFormatVersion{
    version: String
}

enum ParameterType{

}