

// Las funciones o structs de los módulos por defecto son privadas.
#[allow(dead_code)]
fn private_func() -> () {
    println!("I'm a private function from module");
}

pub fn hello() -> () {
    println!("Hello, World! By {name}", name="BCode");
    private_func();
    comments();
    print_formats();
}

#[allow(dead_code)]
fn comments() -> () {
    println!("// One line comments"); // Un comentario de una sola línea
    /* Un comentario de 
     varias líneas */
    println!("/* Line block comments */");
    /// Comentario para documentar el código
    println!("/// Generate library docs for the following item");
    // /! Genera una librería de documentación para rodear un elemento
    println!("//! Generate library docs for the enclosing item");
}

#[allow(dead_code)]
fn print_formats() -> () {
    let name: &str = "Brian";
    format!("Hello {}", name); //Escribe texto formateado a string
    print!("Hello {}", name); //Similar a :format! pero el texto es impreso a la consola.
    println!("Hello {}", name); //Similar a :print! pero agrega una nueva línea.
    eprint!("Hello {}", name); //Similar a :format! pero el texto es impreso a un error estándar. 
    eprintln!("Hello {}", name); //Similar a :eprint! pero agrega una nueva línea.
}

#[allow(dead_code)]
fn primitive_types() -> () {
    /* Tipo entero */
    // i8, i16, i32(default), i64, i128 -> enteros con signo
    let an_integer_i8: i8 = 127; //Max value for i8
    let an_integer_i16: i16 = 32767; //Max value for i16
    let an_integer_i32: i32 = 2147483647; //Max value for i32
    let an_integer_i64: i64 = 9223372036854775807; //Max value for i64
    let an_integer_i128: i128 = i128::max_value();
    
    // u8, u16, u32, u64, u128 -> enteros sin signo
    let an_integer_u8: u8 = (127 * 2) + 1; //Max value for u8
    let an_integer_i16: u16 = (32767 * 2) + 1; //Max value for u16
    let an_integer_u32: u32 = (2147483647 * 2) + 1; //Max value for u32
    let an_integer_u64: u64 = (9223372036854775807 * 2) + 1; //Max value for u64
    let an_integer_u128: u128 = u128::max_value(); //Max value for u128

    /* Tipo flotante */
    // f32, f64(default)
    let a_float_f32: f32 = 2.0;
    let a_float_f64: f64 = 0.9999999999999999999999;

    /* Tipo boolean */
    // true or false
    let is_bool: bool = false || true;

    /* Tipo char */
    // Valores de un solo caracter unicode
    let a_char: char = 'x';

    /* Arreglos y tuplas */
    // [T;N] <T> for type and <N> for elements quantity
    // By default, arrays are inmutable
    let an_array: [i8; 5] = [1, 2, 3, 4, 5];
    // an_array[0] = 6; // Error

    let mut a_mutable_array: [i32; 6] = [456, 678, 10005, 2318, 9430, 25];
    a_mutable_array[0] = 1000;
    a_mutable_array[1] = 2000;


}
