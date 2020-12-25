use std::fs::File;
use crate::Result;
use std::io::{Read, Seek, SeekFrom};
use byteorder::{ReadBytesExt, LittleEndian};
use bytes::BytesMut;

/// Represents a valid SAH header.
const HEADER_MAGIC_VALUE: &str = "SAH";

/// A `workspace` is a collection of the files contained within a folder or archive file.
pub struct Workspace {
    root: SFolder,
    data: File
}

/// Represents a virtual folder in the workspace. A folder can contain multiple subfolders, and files.
#[derive(Debug)]
pub struct SFolder {
    pub name: String,
    pub files: Vec<SFile>,
    pub folders: Vec<SFolder>
}

/// Represents a file that contains data in the workspace.
#[derive(Debug)]
pub struct SFile {
    pub name: String,
    pub offset: usize,
    pub length: usize
}

impl Workspace {

    /// Opens a workspace from a header and data file.
    ///
    /// # Arguments
    /// * `header_file_path`    - The path to the Shaiya Archive Header (usually "data.sah")
    /// * `data_file_path`      - The path to the Shaiya Archive File which contains the file data (usually "data.saf")
    pub fn from_archive(header_file_path: &str, data_file_path: &str) -> Result<Self> {
        let mut header_file = File::open(header_file_path)?;
        let data_file = File::open(data_file_path)?;

        let mut root_folder = SFolder { name: "data".to_owned(), files: vec![], folders: vec![] };
        parse_header(&mut header_file, &mut root_folder)?;

        Ok(Workspace { root: root_folder, data: data_file })
    }

    /// Gets a file at a specified path.
    ///
    /// # Arguments
    /// * `path`    - The path to the file.
    pub fn file(&self, path: &str) -> Result<&SFile> {
        let paths = path.split('/');    // Split based on a path seperator
        let mut folder = &self.root;

        for subpath in paths {
            let candidate_file = folder.files.iter().find(|f| f.name.eq_ignore_ascii_case(subpath));
            if let Some(file) = candidate_file  {
                return Ok(file)
            }

            let candiate_folder = folder.folders.iter().find(|f| f.name.eq_ignore_ascii_case(subpath));
            if let Some(subfolder) = candiate_folder {
                folder = subfolder;
            }
        }

        Err(format!("Unable to find file with path: {}", path)).unwrap()
    }

    /// Gets a folder at a specified path.
    ///
    /// # Arguments
    /// * `path`    - The path to the folder.
    pub fn folder(&self, path: &str) -> Result<&SFolder> {
        if path == "/" {
            return Ok(&self.root)
        }

        let paths = path.split('/');    // Split based on a path seperator
        let mut folder = &self.root;
        let mut last = "";

        for subpath in paths {
            let candiate_folder = folder.folders.iter().find(|f| f.name.eq_ignore_ascii_case(subpath));
            if let Some(subfolder) = candiate_folder {
                folder = subfolder;
            }

            last = subpath
        }

        if folder.name.eq_ignore_ascii_case(last) {
            return Ok(folder)
        }

        Err(format!("Unable to find folder with path: {}", path)).unwrap()
    }

    /// Reads the data for a file.
    ///
    /// # Arguments
    /// * `file`    - The file to read.
    pub fn data(&self, file: &SFile) -> Result<BytesMut> {
        let mut data = &self.data;
        let required_data = file.offset + file.length;
        let available_data = data.metadata()?.len() as usize;

        if required_data > available_data {
            Err(format!("Required file length exceeds the data available (required: {}, available: {})", required_data, available_data)).unwrap()
        }

        let mut file_buf: Vec<u8> = vec![0; file.length as usize];
        data.seek(SeekFrom::Start(file.offset as u64))?;
        data.read_exact(&mut file_buf)?;
        Ok(BytesMut::from(file_buf.as_slice()))
    }
}

/// Parses a Shaiya header file.
///
/// # Arguments
/// * `header_file` - The Shaiya Archive header file.
/// * `folder`      - The root workspace folder.
fn parse_header(header_file: &mut File, folder: &mut SFolder) -> Result<()> {
    // Read the file header to ensure that it is an SAH file.
    let mut header: [u8; 3] = [0; 3];
    header_file.read_exact(&mut header)?;
    let header = std::str::from_utf8(&header)?;
    if header != HEADER_MAGIC_VALUE {
        Err(format!("Invalid SAH header: {}", header)).unwrap()
    }

    // Skip the next 4 bytes, read the total file count, and then skip another 45 bytes
    header_file.seek(SeekFrom::Current(4))?;
    let _total_file_count = header_file.read_i32::<LittleEndian>()? as usize;
    header_file.seek(SeekFrom::Current(45))?;

    // Parse the root folder
    parse_folder(&mut *header_file, &mut *folder)?;
    Ok(())
}

/// Parses a folder present in the SAH file.
///
/// # Arguments
/// * `header_file` - The SAH file handle.
/// * `folder`      - The current folder that is being read.
fn parse_folder(header_file: &mut File, folder: &mut SFolder) -> Result<()> {
    let files = &mut folder.files;
    let subfolders = &mut folder.folders;
    let file_qty = header_file.read_i32::<LittleEndian>()? as usize;
    for _ in 0..file_qty {
        // Read the name of the file
        let name_len = header_file.read_i32::<LittleEndian>()? as usize;
        let mut name_data: Vec<u8> = vec![0; name_len as usize];
        header_file.read_exact(name_data.as_mut_slice())?;
        let name = String::from_utf8_lossy(&name_data).trim_end_matches(char::from(0)).to_owned();

        // Read the offset and length of the file's data.
        // For some reason, using ReadBytesExt::read_i64 produces incorrect values here, even when the bytes are the same.
        let mut data: [u8; 8] = [0; 8];
        header_file.read_exact(&mut data)?;
        let offset = u64::from_le_bytes(data);

        let mut data: [u8; 4] = [0; 4];
        header_file.read_exact(&mut data)?;
        let length = u32::from_le_bytes(data);

        // Skip the next 4 bytes
        header_file.seek(SeekFrom::Current(4))?;
        files.push(SFile { name, offset: offset as usize, length: length as usize});
    }

    let folder_qty = header_file.read_i32::<LittleEndian>()?;
    for _ in 0..folder_qty {
        // Read the name of the folder
        let name_len = header_file.read_i32::<LittleEndian>()? as usize;
        let mut name_data: Vec<u8> = vec![0; name_len as usize];
        header_file.read_exact(name_data.as_mut_slice())?;
        let name = String::from_utf8_lossy(&name_data).trim_end_matches(char::from(0)).to_owned();

        // Create the folder and recursively parse it's contents
        let mut subfolder = SFolder { name, files: vec![], folders: vec![] };
        parse_folder(&mut *header_file, &mut subfolder)?;
        subfolders.push(subfolder);
    }

    Ok(())
}