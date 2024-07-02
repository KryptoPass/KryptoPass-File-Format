use super::central_directory::CentralDirectory;
pub use super::file_record::FileRecord;
pub use super::header::Header;
pub use super::metadata::Metadata;
use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
};

#[derive(Debug)]
pub struct KPDBReader {
    file: BufReader<File>,
    header: Header,
    central_directory: CentralDirectory,
}

impl KPDBReader {
    pub fn open(filepath: &str) -> std::io::Result<Self> {
        let file = BufReader::new(File::open(filepath)?);

        let mut reader = KPDBReader {
            file,
            header: Header::new(), // Inicializamos con valores por defecto
            central_directory: CentralDirectory::new(),
        };

        // Leer y deserializar el header
        reader.read_header()?;

        // Leer y deserializar el directorio central
        reader.read_central_directory()?;

        Ok(reader)
    }

    fn read_header(&mut self) -> std::io::Result<()> {
        let mut header_data = vec![0u8; 39]; // Tamaño del header
        self.file.read_exact(&mut header_data)?;
        self.header = Header::deserialize(&header_data);
        Ok(())
    }

    fn read_central_directory(&mut self) -> std::io::Result<()> {
        self.file
            .seek(SeekFrom::Start(self.header.central_directory_offset))?;
        let mut central_directory_data = vec![0u8; self.header.central_directory_size as usize];
        self.file.read_exact(&mut central_directory_data)?;
        self.central_directory = CentralDirectory::deserialize(&central_directory_data);
        Ok(())
    }

    pub fn read_file_record(&mut self, index: usize) -> std::io::Result<FileRecord> {
        let file_metadata = &self.central_directory.files[index];

        // Leer metadata
        self.file
            .seek(SeekFrom::Start(file_metadata.metadata_offset))?;
        let mut metadata_data = vec![0u8; file_metadata.metadata_size as usize];
        self.file.read_exact(&mut metadata_data)?;
        let metadata = Metadata::deserialize(&metadata_data);

        // Leer data
        self.file.seek(SeekFrom::Start(file_metadata.file_offset))?;
        let mut file_data = vec![0u8; file_metadata.file_size as usize];
        self.file.read_exact(&mut file_data)?;

        // Leer preview (si existe)
        let preview = if let (Some(preview_offset), Some(preview_size)) =
            (file_metadata.preview_offset, file_metadata.preview_size)
        {
            self.file.seek(SeekFrom::Start(preview_offset))?;
            let mut preview_data = vec![0u8; preview_size as usize];
            self.file.read_exact(&mut preview_data)?;
            Some(bincode::deserialize(&preview_data).unwrap())
        } else {
            None
        };

        Ok(FileRecord::new(file_data, metadata, preview))
    }
}
