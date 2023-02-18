use samuel::morse::translate;

fn main() {
    let translated_string = translate("Hello 123456789 Samuel");
    println!("{}", translated_string);
}

// Instalar cargo-modules para poder visualizar cómodamente los módulos en un árbol.
// cargo install cargo-modules
// cargo modules generate tree
// cargo modules generate tree --with-types
// Módulos en Rust no están directamente relacionados con archivos
