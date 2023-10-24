use std::collections::HashMap;

struct ParameterSets{
    sets: HashMap<String, ParameterSet>
}

struct ParameterSet<T>{
    name: String,
    key_values: HashMap<String, T>
}


struct Version{

}