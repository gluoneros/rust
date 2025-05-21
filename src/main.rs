// Ejemplo de variables en Rust
fn variables() {
    let x = 5; // Variable inmutable
    let mut y = 10; // Variable mutable
    y += 5; // Modificando la variable mutable
    println!("x: {}, y: {}", x, y);
}

// Ejemplo de estructuras en Rust
struct GrupoPestañas {
    nombre: String,
    pestañas: Vec<String>, // Lista de URLs o títulos
}

impl GrupoPestañas {
    fn new(nombre: &str) -> Self {
        GrupoPestañas {
            nombre: nombre.to_string(),
            pestañas: Vec::new(),
        }
    }

    fn agregar_pestaña(&mut self, url: &str) {
        self.pestañas.push(url.to_string());
    }
}

fn main() {
    let mut grupo = GrupoPestañas::new("Redes Sociales");
    grupo.agregar_pestaña("https://twitter.com");
    println!("Grupo: {:?}", grupo.pestañas);
}