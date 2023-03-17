use getopts::{Matches, Options};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum GetOptType {
    OPT,
    FLAG,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct GetOptOption {
    description: String,
    long_name: String,
    short_name: String,
    get_opt_type: GetOptType,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct GetOptConfig {
    options: BTreeMap<String, GetOptOption>,
}

fn getopt_load_config(config_path: &String) -> GetOptConfig {
    let f = std::fs::File::open(config_path).expect("Could not open file");
    let get_opt_config: GetOptConfig = serde_yaml::from_reader(f).expect("Could not read values.");
    return get_opt_config;
}

pub fn getopt_setup(program: &String, args: &Vec<String>) -> Matches {
    if args.is_empty() {
        panic!("getopt setup has empty program arguments");
    }

    if args[0] == String::from("--getoptconfig") {
        let config_path = &args[1];
        let get_opt_config = getopt_load_config(&config_path);
        let get_opt_options = get_opt_config.options;

        let mut opts = Options::new();
        // Define standard help flag
        opts.optflag("h", "help", "Print help menu.");

        for (_, get_opt_option) in &get_opt_options {
            if get_opt_option.get_opt_type == GetOptType::FLAG {
                opts.optflag(
                    &get_opt_option.short_name,
                    &get_opt_option.long_name,
                    &get_opt_option.description,
                );
            } else if get_opt_option.get_opt_type == GetOptType::OPT {
                opts.optopt(
                    &get_opt_option.short_name,
                    &get_opt_option.long_name,
                    &get_opt_option.description,
                    "",
                );
            } else {
                panic!(
                    "unrecognised get opt type {:?}",
                    get_opt_option.get_opt_type
                );
            }
        }

        let matches = match opts.parse(&args[2..]) {
            Ok(m) => m,
            Err(e) => {
                panic!(
                    "Program {:?} has invalid program arguments {:?}",
                    program, e
                )
            }
        };

        // Handle default help flag in program arguments
        if matches.opt_present("h") {
            let brief = format!("Usage: {} [options]", program);
            print!("{}", opts.usage(&brief))
        }

        return matches;
    } else {
        panic!("--getoptconfig is not the first option from the program arguments");
    }
}

#[cfg(test)]
mod tests {
    use crate::getopt_load_config;

    use super::*;

    #[test]
    fn load_config() {
        let get_opt_config = get_config();

        assert_eq!(get_opt_config.options.len(), 2);
        assert!(get_opt_config.options.contains_key(&String::from("port")));
        assert!(get_opt_config
            .options
            .contains_key(&String::from("verbose")));
    }

    #[test]
    #[should_panic(expected = "getopt setup has empty program arguments")]
    fn setup_empty_args() {
        let program = String::from("yagetopts_test");
        let args: Vec<String> = vec![];

        getopt_setup(&program, &args);
    }

    #[test]
    #[should_panic(expected = "--getoptconfig is not the first option from the program arguments")]
    fn setup_panic_when_getoptconfig_option_is_not_first_in_args() {
        let program = String::from("yagetopts_test");
        let args = vec![
            String::from("-h"),
            String::from("--getoptconfig"),
            String::from("example_getopt_config.yml"),
        ];

        getopt_setup(&program, &args);
    }

    #[test]
    fn help_flag_after_setup() {
        let program = String::from("yagetopts_test");
        let args = vec![
            String::from("--getoptconfig"),
            String::from("example_getopt_config.yml"),
            String::from("-h"),
        ];

        let matches = getopt_setup(&program, &args);
        if matches.opt_present("h") {
            assert!(true);
        }
    }

    fn get_config() -> GetOptConfig {
        let config_path = String::from("example_getopt_config.yml");
        return getopt_load_config(&config_path);
    }
}
