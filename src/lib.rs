#[macro_use]
extern crate maplit;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate yaml_rust;

use yaml_rust::*;
use std::collections::HashMap;

pub const FST_YAML_STR: &str = "
    dict:
      A: jedna
      B: druga
      C: trzej
      D: czterej
      E: pięciu
    root:
      foo:
        - jeden
        - dwa
        - trzy
      bar:
        - 1
";

pub const FLAG_CFG_YAML: &str = "
---
global:
  \"0\":
    Value: \"0x1089\"
dependent:
  \"branch\":
    \"v1.0\":
      \"01\":
        Spec:
          comment: Makes possible to use debug mode
          value: \"-1\"
";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct FlagsConfig {
    global: HashMap<String, Flag>,
    dependent: HashMap<String, HashMap<String, HashMap<String, Flag>>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Flag {
    Value(String),
    Spec{value: String, comment: String},
}

#[derive(Debug)]
enum Tree {
    Branch(Box<Tree>),
    Leaf,
}

pub fn yamel(s: &str) -> (String, i64) {
    let docs = YamlLoader::load_from_str(s).unwrap();
    let dict = &docs[0];

    println!("Mapa: {:?}", dict);

    (
        String::from(dict["root"]["foo"][0].as_str().unwrap()),
        match dict["root"]["bar"][0].as_i64() {
            Some(val) => val,
            None => 0,
        },
    )
}

fn merge_glob(fst: HashMap<String, Flag>, snd: HashMap<String, Flag>) -> HashMap<String, Flag> {
   let mut output_map : HashMap<String, Flag> = HashMap::new();

   output_map.extend(fst);
   output_map.extend(snd);

   output_map
}

pub fn merge<'a>(fst: &HashMap<&'a str, &'a str>, snd: &HashMap<&'a str, &'a str>) -> HashMap<&'a str, &'a str> {
   let mut output_map : HashMap<&'a str, &'a str> = HashMap::new();

   output_map.extend(fst);
   output_map.extend(snd);

   output_map
}

pub fn abc() -> String {
    String::from("abc")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkit() {
        assert_eq!(2+2, 4)
    }

    #[test]
    fn checkabc() {
        assert_eq!("abc".to_string(), abc())
    }

    #[test]
    fn correct_value_from_yaml() {
        assert_eq!(yamel(FST_YAML_STR), ("jeden".to_string(), 1))
    }

    #[test]
    fn merging_hashes() {
        let fst = hashmap!{
            "A" => "jedna",
            "B" => "druga",
            "C" => "trzecia",
        };
        let snd = hashmap!{
            "C" => "trzej",
            "D" => "czterej",
            "E" => "pięciu",
        };

        let mrg = merge(&fst, &snd);

        let exp = hashmap!{
            "A" => "jedna",
            "B" => "druga",
            "C" => "trzej",
            "D" => "czterej",
            "E" => "pięciu",
        };

        assert_eq!(&mrg, &exp)
    }

    //#[test]
    //fn merging_hashes_of_hashes() {
    //    let fst = hashmap!{
    //        "A" => hashmap!{
    //            "1" => "jedna",
    //            "2" => "druga",
    //        },
    //        "B" => hashmap!{
    //            "3" => "trzecia",
    //            "4" => "czwarta",
    //        },
    //    };
    //    let snd = hashmap!{
    //        "A" => hashmap!{
    //            "1" => "jedna",
    //            "2" => "druga",
    //        },
    //        "B" => hashmap!{
    //            "3" => "trzecia",
    //            "4" => "czwarta",
    //        },
    //    };

    //    let mrg = merge(&fst, &snd);

    //    let exp = hashmap!{
    //        "A" => hashmap!{
    //            "1" => "jedna",
    //            "2" => "druga",
    //        },
    //        "B" => hashmap!{
    //            "3" => "trzecia",
    //            "4" => "czwarta",
    //        },
    //    };

    //    assert_eq!(&mrg, &exp)
    //}

    //#[test]
    //fn flag_conf_serialize() {
    //    let flag_conf = FlagsConfig {
    //        global: hashmap!{
    //            "10".to_string() => Flag::Spec{
    //                value: "13".to_string(),
    //                comment: "This is default value".to_string(),
    //            },
    //            "17".to_string() => Flag::Value("0xA1".to_string()),
    //        },
    //        dependent: hashmap!{
    //            "A".to_string() => hashmap!{
    //                "10".to_string() => Flag::Value("20".to_string()),
    //            },
    //            "B".to_string() => hashmap!{
    //                "10".to_string() => Flag::Value("21".to_string()),
    //            },
    //        },
    //    };

    //    let out = serde_yaml::to_string(&flag_conf).unwrap();

    //    println!("[Config]\n{}", &out);
    //    // non deterministic
    //    assert_eq!(
    //        &out,
    //        "---\nglobal:\n  \"10\":\n    Spec:\n      value: \"13\"\n      comment: This is default value\n  \"17\":\n    Value: 0xA1\ndependent:\n  A:\n    \"10\":\n      Value: \"20\"\n  B:\n    \"10\":\n      Value: \"21\""
    //    )
    //}

    #[test]
    fn flag_conf_deserialize() {
        let des: FlagsConfig = 
            match serde_yaml::from_str(&FLAG_CFG_YAML) {
                Ok(r) => r,
                Err(msg) => panic!["O kurde: {}", msg]
            };
        println!("{:?}", des)
    }
}
