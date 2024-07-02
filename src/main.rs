mod kpdb;

use kpdb::{KPDB, Serializable};

#[derive(Debug)]
struct ComplexData {
    field1: String,
    field2: i32,
}

impl Serializable for ComplexData {
    fn serialize(&self) -> Vec<u8> {
        // Implementación personalizada de serialización
        let mut data = Vec::new();
        data.extend(self.field1.len().to_le_bytes());
        data.extend(self.field1.as_bytes());
        data.extend(self.field2.to_le_bytes());
        data
    }

    fn deserialize(data: &[u8]) -> Self {
        // Implementación personalizada de deserialización
        let field1_len = usize::from_le_bytes(data[0..8].try_into().unwrap());
        let field1 = String::from_utf8(data[8..8+field1_len].to_vec()).unwrap();
        let field2 = i32::from_le_bytes(data[8+field1_len..].try_into().unwrap());
        ComplexData { field1, field2 }
    }
}

fn main() {
    // Ejemplo de uso de la API de alto nivel para escritura y lectura
    {
        let mut kpdb = KPDB::new("example.kpdb").expect("Failed to create writer");

        // Datos simples
        let data = vec![1, 2, 3, 4, 5];
        kpdb.write(data).expect("Failed to write data");

        let complex_data = ComplexData {
            field1: "Hello".to_string(),
            field2: 42,
        };
        kpdb.write(complex_data).expect("Failed to write complex data");

        kpdb.finalize().expect("Failed to finalize file");
    }

    // Ejemplo de uso de la API de alto nivel para lectura
    {
        let mut kpdb = KPDB::open("example.kpdb").expect("Failed to open file");

        // Leer datos simples
        let data: Vec<u8> = kpdb.read(0).expect("Failed to read data");
        println!("Read data: {:?}", data);


        let complex_data: ComplexData = kpdb.read(1).expect("Failed to read complex data");
        println!("Read complex data: {:?}", complex_data);
    }
}
