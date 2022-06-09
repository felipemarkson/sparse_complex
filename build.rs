use flate2::read::GzDecoder;
use std::error::Error;
use std::fs::{self, remove_file, File};
use std::io;
use std::path::Path;
use tar::Archive;

const URL: &str = "https://gitlab.com/libeigen/eigen/-/archive/3.4.0/eigen-3.4.0.tar.gz";
const EIGEN_PATH: &str = "./eigen-3.4.0/";
const EIGEN_TAR_NAME: &str = "eigen-3.4.0.tar.gz";

type GenericError = Box<dyn Error>;

fn get_file_from_url<P>(url: &str, path: &P) -> Result<(), GenericError>
where
    P: AsRef<Path>,
{
    use curl::easy::Easy;
    use std::io::Write;

    let f = fs::File::create(path)?;
    let mut writer = io::BufWriter::new(f);
    let mut easy = Easy::new();
    easy.url(url)?;
    easy.write_function(move |data| Ok(writer.write(data).unwrap()))?;
    easy.perform()?;
    let response_code = easy.response_code()?;

    if response_code == 200 {
        Ok(())
    } else {
        Err(format!("Unexpected response code {} for {}", response_code, url).into())
    }
    // Ok(())
}

fn main() -> Result<(), GenericError> {
    if !std::env::var("DOCS_RS").is_ok() {
        // ... your code here ...

        let out_dir = std::env::var("OUT_DIR")?;
        let out_path = Path::new(&out_dir);
        let eingen_tar_path = out_path.join(EIGEN_TAR_NAME);

        let eigen_dir = out_path.join(EIGEN_PATH);
        if !eigen_dir.is_dir() {
            get_file_from_url(URL, &eingen_tar_path)?;
            let tar_gz = File::open(&eingen_tar_path)?;
            let tar = GzDecoder::new(tar_gz);
            let mut archive = Archive::new(tar);
            archive.unpack(out_path)?;
            remove_file(eingen_tar_path)?;
        }

        cc::Build::new()
            .cpp(true)
            .include(eigen_dir)
            .file("src/solver.cpp")
            .flag_if_supported("-std=c++1y")
            .compile("solver_cpp");
    }
    Ok(())
}
