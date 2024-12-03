use std::env;

pub fn get_env(name: String) -> String {
    let expect: String = format!("${} is not set!", name);
    let v = env::var(name).expect(&expect);

    v
}
