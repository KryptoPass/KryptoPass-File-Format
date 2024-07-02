use super::central_directory::{CentralDirectory, FileMetadata};
pub use super::file_record::FileRecord;
pub use super::header::Header;
use super::utils::{calculate_crc, calculate_hmac};
use std::{
    fs::File,
    io::{BufWriter, Seek, SeekFrom, Write},
};

pub struct KPDBWriter {
    file: BufWriter<File>,
    filepath: String,
    header: Header,
    central_directory: CentralDirectory,
    current_offset: u64,
}

impl KPDBWriter {
    pub fn new(filepath: &str) -> std::io::Result<Self> {
        let temp_filepath = format!("{}.tmp", filepath);
        let file = BufWriter::new(File::create(&temp_filepath)?);

        Ok(KPDBWriter {
            file,
            filepath: String::from(filepath),
            header: Header::new(),
            central_directory: CentralDirectory::new(),
            current_offset: 0,
        })
    }

    pub fn write_header(&mut self) -> std::io::Result<()> {
        let serialized_header = self.header.serialize();
        self.file.write_all(&serialized_header)?;
        self.current_offset += serialized_header.len() as u64;
        self.write_padding(self.header.padding_size as usize)
    }

    fn write_padding(&mut self, size: usize) -> std::io::Result<()> {
        let padding = vec![0u8; size];
        self.file.write_all(&padding)?;
        self.current_offset += size as u64;
        Ok(())
    }

    pub fn write_file_record(&mut self, record: FileRecord) -> std::io::Result<()> {
        let serialized_metadata = record.metadata.serialize();
        let metadata_offset = self.current_offset;
        let metadata_size = serialized_metadata.len() as u64;
        self.file.write_all(&serialized_metadata)?;
        self.current_offset += metadata_size;

        let data_offset = self.current_offset;
        let data_size = record.data.len() as u64;
        self.file.write_all(&record.data)?;
        self.current_offset += data_size;

        let (preview_offset, preview_size) = if let Some(preview) = record.preview {
            let serialized_preview = bincode::serialize(&preview).unwrap();
            let preview_offset = self.current_offset;
            let preview_size = serialized_preview.len() as u64;
            self.file.write_all(&serialized_preview)?;
            self.current_offset += preview_size;
            (Some(preview_offset), Some(preview_size))
        } else {
            (None, None)
        };

        let file_metadata = FileMetadata {
            file_name: format!("file_{}", self.central_directory.files.len()),
            file_offset: data_offset,
            file_size: data_size,
            crc: calculate_crc(&record.data),
            mac: calculate_hmac(&record.data),
            metadata_offset,
            metadata_size,
            preview_offset,
            preview_size,
        };

        self.central_directory.files.push(file_metadata);
        Ok(())
    }

    pub fn finalize(&mut self) -> std::io::Result<()> {
        // Calcular el offset del directorio central
        let central_directory_offset = self.current_offset;

        // Serializar el directorio central
        let serialized_central_directory = self.central_directory.serialize();
        self.file.write_all(&serialized_central_directory)?;
        let central_directory_size = serialized_central_directory.len() as u64;

        // Actualizar el header con la información del directorio central
        self.header.central_directory_offset = central_directory_offset;
        self.header.central_directory_size = central_directory_size;

        // Reescribir el header actualizado al inicio del archivo
        self.file.seek(SeekFrom::Start(0))?;
        let serialized_header = self.header.serialize();
        self.file.write_all(&serialized_header)?;

        // No escribir el padding aquí para no sobrescribir los datos

        // Renombrar archivo temporal al archivo final
        let temp_filepath = format!("{}.tmp", self.filepath);

        std::fs::rename(temp_filepath, self.filepath.clone())?;

        Ok(())
    }
}
