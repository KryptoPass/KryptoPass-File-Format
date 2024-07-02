mod header;
mod metadata;
mod file_record;
mod central_directory;
mod utils;
mod kpdb_writer;
mod kpdb_reader;

pub use file_record::FileRecord;
pub use metadata::Metadata;
pub use kpdb_writer::KPDBWriter;
pub use kpdb_reader::KPDBReader;
use central_directory::FileMetadata;
use serde::{Serialize, Deserialize};
use std::io;

// Trait Serializable
pub trait Serializable {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8]) -> Self where Self: Sized;
}

// Implementación del trait Serializable para tipos nativos que implementan Serialize y Deserialize
impl<T> Serializable for T where T: Serialize + for<'de> Deserialize<'de> {
    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    fn deserialize(data: &[u8]) -> Self {
        bincode::deserialize(data).unwrap()
    }
}

// API de alto nivel para la escritura y lectura
pub struct KPDB {
    writer: Option<KPDBWriter>,
    reader: Option<KPDBReader>,
}

impl KPDB {
    // Crear un nuevo archivo para escribir
    pub fn new(filepath: &str) -> io::Result<Self> {
        let mut writer = KPDBWriter::new(filepath)?;
        writer.write_header()?;
        Ok(KPDB {
            writer: Some(writer),
            reader: None,
        })
    }

    // Abrir un archivo existente para leer
    pub fn open(filepath: &str) -> io::Result<Self> {
        let reader = KPDBReader::open(filepath)?;
        Ok(KPDB {
            writer: None,
            reader: Some(reader),
        })
    }

    // Escribir datos en el archivo
    pub fn write<T: Serializable>(&mut self, data: T) -> io::Result<()> {
        if let Some(writer) = &mut self.writer {
            let serialized_data = data.serialize();
            let metadata = Metadata::new();
            let file_record = FileRecord::new(serialized_data, metadata, None);
            writer.write_file_record(file_record)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "File is not opened for writing"))
        }
    }

    // Finalizar la escritura del archivo
    pub fn finalize(&mut self) -> io::Result<()> {
        if let Some(writer) = &mut self.writer {
            writer.finalize()
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "File is not opened for writing"))
        }
    }

    // Leer datos desde el archivo
    pub fn read<T: Serializable>(&mut self, index: usize) -> io::Result<T> {
        if let Some(reader) = &mut self.reader {
            let file_record = reader.read_file_record(index)?;
            let data = T::deserialize(&file_record.data);
            Ok(data)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "File is not opened for reading"))
        }
    }

    // Listar archivos almacenados
    pub fn list_files(&self) -> io::Result<Vec<FileMetadata>> {
        if let Some(reader) = &self.reader {
            Ok(reader.central_directory.files.clone())
        } else if let Some(writer) = &self.writer {
            Ok(writer.central_directory.files.clone())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "File is not opened"))
        }
    }
}
