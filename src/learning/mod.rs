// Las funciones o structs de los módulos por defecto son privadas.
#[allow(dead_code)]
fn private_func() -> () {
    println!("I'm a private function from module");
}

pub fn hello() -> () {
    println!("Hello, World! By {name}", name="BCode");
    private_func();
    comments();
}

#[allow(dead_code)]
pub fn comments() -> () {
    println!("// One line comments"); // Un comentario de una sola línea
    /* Un comentario de 
     varias líneas */
    println!("/* Line block comments */");
    /// COmentario para documentar el código
    println!("/// Generate library docs for the following item");
    // /! Genera una librería de documentación para rodear un elemento
    println!("//! Generate library docs for the enclosing item");
}
