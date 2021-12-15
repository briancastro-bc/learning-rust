mod learning;
mod sort;

fn main() {
    println!("{:?}", sort::shell_sort());
    self::give_adult(None);
}

fn give_adult(drink: Option<&str>) {
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary"),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No drink? Oh well")
    }
}

#[allow(dead_code)]
fn test() {}
