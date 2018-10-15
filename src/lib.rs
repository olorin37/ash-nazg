#[macro_use]
extern crate maplit;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate yaml_rust;

use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

use std::fs::File;

pub const FLAG_CFG_YAML: &str = "
---
global:
  \"0\":
    Value: \"0x1089\"
dependent:
  \"branch\":
    \"v1.x\":
      \"01\":
        Spec:
          comment: Makes possible to use debug mode
          value: \"-1\"
  \"target\":
    \"3310r\":
      \"01\":
        Spec:
          comment: Makes possible to use debug mode
          value: \"10\"
    \"5511\":
      \"01\":
        Spec:
          comment: Makes possible to use debug mode
          value: \"11\"
";

pub const FLAG_CFG_YAML2: &str = "
---
global:
  \"0\":
    Value: \"0x1037\"
dependent:
  \"branch\":
    \"v1.x\":
      \"01\":
        Spec:
          comment: Makes possible to use debug mode
          value: \"371\"
  \"target\":
    \"3310r\":
      \"01\":
        Spec:
          comment: Makes possible to use debug mode
          value: \"3710\"
    \"5511\":
      \"01\":
        Spec:
          comment: Makes possible to use debug mode
          value: \"3711\"
";

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct FlagsConfig {
    global: HashMap<String, Flag>,
    dependent: HashMap<String, HashMap<String, HashMap<String, Flag>>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
enum Flag {
    Value(String),
    Spec{
        value: String,
        comment: String,
    },
}

fn merge_hashmap<T, U>(fst: HashMap<T, U>, snd: HashMap<T, U>) -> HashMap<T, U>
    where
    T: Eq + Hash
{
   let mut output_map : HashMap<T, U> = HashMap::new();
   output_map.extend(fst);
   output_map.extend(snd);

   output_map
}

fn resolve_dependent<P, Q, T>(
    input: HashMap<P, HashMap<Q, HashMap<T, Flag>>>,
    assignments: &Vec<(P, Q)>
) -> HashMap<T, Flag>
    where
    P: Eq + Hash + Clone + Debug,
    Q: Eq + Hash + Clone + Debug,
    T: Eq + Hash + Clone + Debug,
{
    let empty_hash: HashMap<T, Flag> = HashMap::new();
    let mut out: HashMap<T, Flag> = HashMap::new();
    for a in assignments {
        let flags_map = input.get(&a.0).and_then(
            |h| h.get(&a.1)
        ).unwrap_or(&empty_hash);

        for (key, value) in flags_map.iter() {
            out.insert(key.to_owned(), value.clone());
        }
    };

    out
}

fn to_flags_string(flags_map: HashMap<String, Flag>, sep_out: &str) -> String {
    let mut out = String::new();
    for (flag_key, flag_value) in flags_map {
        out += &(flag_key + sep_out + &(
            match flag_value {
                Flag::Value(flag_str_value) => flag_str_value,
                Flag::Spec{ value: flag_str_value, comment: _} => flag_str_value,
            }) + "\n");
    }

    out
}

pub fn go() -> String {
    let assignments: Vec<(String, String)> = vec![
        ("branch".to_string(), "v1.x".to_string()),
        ("target".to_string(), "A".to_string()),
    ];

    let des: FlagsConfig = 
        match serde_yaml::from_str(&FLAG_CFG_YAML) {
            Ok(r) => r,
            Err(msg) => panic!["O kurde: {}", msg],
        };

    let des2: FlagsConfig = 
        match serde_yaml::from_str(&FLAG_CFG_YAML2) {
            Ok(r) => r,
            Err(msg) => panic!["O kurde: {}", msg],
        };

    let globs: HashMap<String, Flag> = merge_hashmap(des.global, des2.global);
    let deps1: HashMap<String, Flag> = resolve_dependent(des.dependent, &assignments);
    let deps2: HashMap<String, Flag> = resolve_dependent(des2.dependent, &assignments);
    let deps: HashMap<String, Flag> = merge_hashmap(deps1, deps2);

    let res: HashMap<String, Flag> = merge_hashmap(globs, deps);

    let output = to_flags_string(res, "=");
    println!("Our flagconfig:\n{}", output);

    output
}

pub fn gogo() {
    let f = File::open("example/flag_conf_gen1.yaml").unwrap();
    let flag_conf_gen: FlagsConfig = match serde_yaml::from_reader(&f) {
        Ok(r) => r,
        Err(msg) => panic!["O kurde: {}", msg],
    };
    println!("Loaded: {:?}", flag_conf_gen);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn go_checking() {
        let expected = "01=371
0=0x1037";
        let output = go();
        assert_eq!(output, expected)

    }

    #[test]
    fn conversion_to_flag_string() {
        let fst = hashmap!{
            String::from("10") => Flag::Spec{value: "13".to_string(), comment: String::from("Disables feature #66")},
            String::from("0x1201") => Flag::Value(String::from("07")),
        };

        let exp_part1: String = "10=13\n".to_string();
        let exp_part2: String = "0x1201=07\n".to_string();
        let done = to_flags_string(fst, "=");

        println!("str: {:?}", &done);
        println!("exp_part1: {:?}", &exp_part1);
        println!("exp_part2: {:?}", &exp_part2);

        //not deterministic order !!!
        //assert_eq!(done, exp);

        assert!(&done.as_str().contains(&exp_part1));
        assert!(&done.as_str().contains(&exp_part2));
    }

    #[test]
    fn resolving_dependent() {
        let des: FlagsConfig = 
            match serde_yaml::from_str(&FLAG_CFG_YAML) {
                Ok(r) => r,
                Err(msg) => panic!["O kurde: {}", msg],
            };
        println!("{:?}", des);
        let sels = vec![
            ("branch".to_string(), "v1.x".to_string()),
            ("target".to_string(), "3310r".to_string()),
        ];

        for sel in sels.iter()  {
            println!("Only dependent for {}={}: {:?}.",
                     sel.0,
                     sel.1,
                     &des.dependent.get(&sel.0).unwrap().get(&sel.1).unwrap());
        }
        let rdep = resolve_dependent(des.dependent, &sels);
        println!("This is resolved map: {:?}", rdep);
    }

    #[test]
    fn globals_merging() {
        let fst = hashmap!{
            String::from("10") => Flag::Spec{value: "13".to_string(), comment: String::from("Disables feature #66")},
            String::from("0x1201") => Flag::Value(String::from("07")),
        };
        let snd = hashmap!{
            String::from("10") => Flag::Spec{value: "17".to_string(), comment: String::from("Disables feature #66")},
            String::from("0x1241") => Flag::Value(String::from("77")),
        };

        let mrg;

        let exp = hashmap!{
            String::from("10") => Flag::Spec{value: "17".to_string(), comment: String::from("Disables feature #66")},
            String::from("0x1201") => Flag::Value(String::from("07")),
            String::from("0x1241") => Flag::Value(String::from("77")),
        };

        mrg = merge_hashmap(fst, snd);

        assert_eq!(mrg, exp)
    }

    #[test]
    fn flag_conf_deserialize() {
        let des: FlagsConfig = 
            match serde_yaml::from_str(&FLAG_CFG_YAML) {
                Ok(r) => r,
                Err(msg) => panic!["O kurde: {}", msg]
            };
        println!("{:?}", des);
    }
}
