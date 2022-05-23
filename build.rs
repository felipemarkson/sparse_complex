use flate2::read::GzDecoder;
use std::error::Error;
use std::fs::{remove_file, File};
use std::io::Cursor;
use std::path::Path;
use tar::Archive;

const URL: &str = "https://gitlab.com/libeigen/eigen/-/archive/3.4.0/eigen-3.4.0.tar.gz";
const EIGEN_PATH: &str = "./eigen-3.4.0/";
const EIGEN_TAR_NAME: &str = "eigen-3.4.0.tar.gz";

type GenericError = Box<dyn Error>;

async fn get_file_from_url<P>(url: &str, path: &P) -> Result<(), GenericError>
where
    P: AsRef<Path>,
{
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(path)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), GenericError> {

    let out_dir = std::env::var("OUT_DIR")?;
    let out_path = Path::new(&out_dir);
    let eingen_tar_path = out_path.join(EIGEN_TAR_NAME);

    let eigen_dir = out_path.join(EIGEN_PATH);
    if !eigen_dir.is_dir() {
        get_file_from_url(URL, &eingen_tar_path).await?;
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
        .compile("solver_cpp");

    Ok(())
}