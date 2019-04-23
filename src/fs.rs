use alloc::vec::Vec;
use alloc::string::String;
use log::info;
use uefi::{
    prelude::{
        BootServices,
        ResultExt
    },

    proto::media::{
        file::{
            RegularFile,
            File, 
            FileAttribute, 
            FileInfo,
            FileMode
        },
        fs::SimpleFileSystem,
    }
};

#[derive(Clone, Copy, Debug)]
pub enum FsError {

}

pub struct Filesystem<'a> {
    fs: &'a mut SimpleFileSystem
}

impl <'a> Filesystem<'a> {
    pub fn new(boot_services: &BootServices) -> Result<Self, FsError> {
        let fs = boot_services.locate_protocol::<SimpleFileSystem>()
            .expect_success("file system protocol");
        let fs = unsafe { &mut *fs.get() };

        Ok(Filesystem {
            fs
        })
    }

    pub fn load(&mut self, path: &str) -> Result<Vec<u8>, FsError> {
        info!("Opening file {}...", path);

        let mut dir = self.fs.open_volume().expect_success("open volume");
        info!("  -> Opened volume");

        let mut handle = dir.open(
            path,
            FileMode::Read, 
            FileAttribute::READ_ONLY
        ).expect_success("open file");
        info!("  -> Volume");

        let mut info_buffer = Vec::new();
        info_buffer.resize(100, 0);
        let info = handle.get_info::<FileInfo>(&mut info_buffer).expect_success("file info");

        let file_size = info.file_size() as usize;
        info!("  -> File size is {} bytes", file_size);

        let mut file = unsafe { RegularFile::new(handle) };
        info!("  -> Reading file");

        let mut read_buffer = Vec::new();
        read_buffer.resize(file_size, 0);
        let bytes_read = file.read(&mut read_buffer).expect_success("file read");
        info!("  -> Done reading {} bytes from file", bytes_read);

        Ok(read_buffer)
    }
}