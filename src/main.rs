use std::{
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process,
};

use pechakucha_gen::generate_presentation;

/// Ce programme a pour but de générer un pechakucha à partir de 20 photos
/// Ces 20 photos seront triées par ordre lexicographique
/// Elles sont collées en une page web de slides qui défilent automatiquement
/// toutes les 20 secondes. Bon courage !
fn main() -> Result<(), std::io::Error> {
    // Obtenir le chemin vers le dossier
    let path_folder = get_folder_path();

    // Créer un html contenant notre page web
    let (output, paths) = generate_presentation(&path_folder)?;
    write_presentation(&output, paths)?;

    Ok(())
}

fn write_presentation(output: &str, paths: Vec<PathBuf>) -> Result<(), std::io::Error> {
    let create_dir = fs::create_dir("output");
    if let Err(e) = create_dir {
        match e.kind() {
            std::io::ErrorKind::AlreadyExists => {
                fs::remove_dir_all("output")?;
                fs::create_dir("output")?;
            }
            _ => {
                return Err(e);
            }
        }
    }
    let mut presentation_file = File::create("output/pechakucha.html")?;
    presentation_file.write(output.as_bytes())?;

    // Copy all images in folder
    for image_path in &paths {
        let file_name = image_path
            .file_name()
            .expect("Ne peut pas fonctionner sans filename")
            .to_str()
            .unwrap();
        fs::copy(image_path, format!("output/{}", file_name))?;
    }
    println!("Created presentation pechakucha.html");
    Ok(())
}

fn get_folder_path() -> PathBuf {
    let mut args = env::args();
    args.next();
    // Skip path
    let path_folder = args.next().unwrap_or_else(|| {
        eprintln!("Missing argument: needs a path to a folder.");
        process::exit(1);
    });
    let path_folder = PathBuf::from(path_folder);
    path_folder
}
