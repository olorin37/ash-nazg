extern crate serde_yaml;
extern crate yaml_rust;

#[macro_use] extern crate maplit;
#[macro_use] extern crate serde_derive;

use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

use std::fs::File;

pub const FLAG_CFG_YAML: &str = "
---
global:
  \"0\": \"0x1089\"
dependent:
  \"branch\":
    \"v1.x\":
      \"01\":
        comment: Makes possible to use debug mode
        value: \"-1\"
  \"target\":
    \"3310r\":
      \"01\":
        comment: Makes possible to use debug mode
        value: \"10\"
    \"5511\":
      \"01\":
        comment: Makes possible to use debug mode
        value: \"11\"
";

pub const FLAG_CFG_YAML2: &str = "
---
global:
  \"0\": \"0x1037\"
dependent:
  \"branch\":
    \"v1.x\":
      \"01\":
        comment: Makes possible to use debug mode
        value: \"371\"
  \"target\":
    \"3310r\":
      \"01\":
        comment: Makes possible to use debug mode
        value: \"3710\"
    \"5511\":
      \"01\":
        comment: Makes possible to use debug mode
        value: \"3711\"
";

/// Main data structure which represents Flags Config
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct FlagsConfig {
    global: HashMap<String, Flag>,
    dependent: HashMap<String, HashMap<String, HashMap<String, Flag>>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum Flag {
    Value(String),
    Spec{
        value: String,
        comment: String,
    },
}

impl FlagsConfig {
    /// Loads Flags Config from string slice.
    fn new(from: &str) -> FlagsConfig {
        let out = match serde_yaml::from_str(from) {
            Ok(flags_conf) => flags_conf,
            Err(msg) => panic!["String cannot be parsed to FlagsConfig. {}", msg],
        };
        eprintln!("new FlagsConfig: {:?}", out); //debug

        out
    }

    /// Loads Flags Config from file
    fn from_reader(file: &File) -> FlagsConfig {
        let out = match serde_yaml::from_reader(file) {
            Ok(flags_conf) => flags_conf,
            Err(msg) => panic!["String cannot be parsed to FlagsConfig. {}", msg],
        };
        eprintln!("new FlagsConfig: {:?}", out); //debug

        out
    }

   // fn merge_to(&mut self: FlagsConfig, other: FlagsConfig, assignments: Vec<(String, String)>) -> String {
   //     // TODO: we could use just regural extend here and it should be done separately for both
   //     // parts of structure.
   //     let actual_flags: HashMap<String, Flag> = merge_hashmap(
   //         merge_hashmap(self.global, other.global),
   //         merge_hashmap(
   //             resolve_dependent(self.dependent, &assignments),
   //             resolve_dependent(other.dependent, &assignments)
   //         ),
   //     );
   // }
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
    constraints: &Vec<(P, Q)>
) -> HashMap<T, Flag>
    where
    P: Eq + Hash + Clone + Debug,
    Q: Eq + Hash + Clone + Debug,
    T: Eq + Hash + Clone + Debug,
{
    let empty_hash: HashMap<T, Flag> = HashMap::new();
    let mut out: HashMap<T, Flag> = HashMap::new();
    for constr in constraints {
        let flags_map = input.get(&constr.0).and_then(
            |h| h.get(&constr.1)
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

fn compose(x: FlagsConfig, y: FlagsConfig, assignments: Vec<(String, String)>) -> String {
    let actual_flags: HashMap<String, Flag> = merge_hashmap(
        merge_hashmap(x.global, y.global),
        merge_hashmap(
            resolve_dependent(x.dependent, &assignments),
            resolve_dependent(y.dependent, &assignments)
        )
    );

    let output = to_flags_string(actual_flags, "=");
    eprintln!("Our flagconfig:\n{}", output);

    output
}

pub fn compose_from_str(
    yml1: &str,
    yml2: &str,
    assignments: Vec<(String, String)>
    ) -> String {

    let x = FlagsConfig::new(yml1);
    let y = FlagsConfig::new(yml2);

    compose(x, y, assignments)
}

pub fn compose_from_file(
    file1: &File,
    file2: &File,
    assignments: Vec<(String, String)>
    ) -> String {

    let x = FlagsConfig::from_reader(file1);
    let y = FlagsConfig::from_reader(file2);

    compose(x, y, assignments)
}

pub fn go_compose() -> String {
    let assignments: Vec<(String, String)> = vec![
        ("branch".to_string(), "v1.x".to_string()),
        ("target".to_string(), "A".to_string()),
    ];

    compose_from_str(
        FLAG_CFG_YAML,
        FLAG_CFG_YAML2,
        assignments
    )
}

pub fn go() {
    let f = File::open("example/flag_conf_v1.x.yaml").unwrap();
    let flag_conf_gen = FlagsConfig::from_reader(&f);
    eprintln!("Loaded: {:?}", flag_conf_gen);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compose_from_yaml_str_checking() {
        let assignments: Vec<(String, String)> = vec![
            ("branch".to_string(), "v1.x".to_string()),
            ("target".to_string(), "A".to_string()),
        ];

        let expected = "01=371
0=0x1037
";
        let output = compose_from_str(
            FLAG_CFG_YAML,
            FLAG_CFG_YAML2,
            assignments
        );

        assert_eq!(output, expected)
    }

    #[test]
    fn compose_checking() {
        let assignments: Vec<(String, String)> = vec![
            ("branch".to_string(), "v1.x".to_string()),
            ("target".to_string(), "A".to_string()),
        ];

        let des = FlagsConfig::new(&FLAG_CFG_YAML);
        let des2 = FlagsConfig::new(&FLAG_CFG_YAML2);
        let expected = "01=371
0=0x1037
";
        let output = compose(des, des2, assignments);
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
        let des = FlagsConfig::new(&FLAG_CFG_YAML);
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
        let des = FlagsConfig::new(&FLAG_CFG_YAML);
        println!("{:?}", des);
    }
}
