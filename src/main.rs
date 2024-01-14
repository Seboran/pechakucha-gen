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
    let (path_folder, output_folder) = get_folder_path();

    // Créer un html contenant notre page web
    let (output, paths) = generate_presentation(&path_folder)?;
    write_presentation(&output, paths, output_folder)?;

    Ok(())
}

fn write_presentation(
    output: &str,
    paths: Vec<PathBuf>,
    output_folder: PathBuf,
) -> Result<(), std::io::Error> {
    let path = output_folder
        .to_str()
        .expect("Something went very wrong when unwrapping the output folder to a path.");
    let create_dir = fs::create_dir(path);
    if let Err(e) = create_dir {
        match e.kind() {
            std::io::ErrorKind::AlreadyExists => {
                fs::remove_dir_all(path)?;
                fs::create_dir(path)?;
            }
            _ => {
                return Err(e);
            }
        }
    }
    let mut presentation_file = File::create(format!("{path}/pechakucha.html"))?;
    presentation_file.write(output.as_bytes())?;

    // Copy all images in folder
    for image_path in &paths {
        let file_name = image_path
            .file_name()
            .expect("Ne peut pas fonctionner sans filename")
            .to_str()
            .unwrap();
        fs::copy(image_path, format!("{path}/{}", file_name))?;
    }
    println!("Created presentation pechakucha.html");
    Ok(())
}

fn get_folder_path() -> (PathBuf, PathBuf) {
    let mut args = env::args();
    args.next();
    // Skip path
    let path_folder = args.next().unwrap_or_else(|| {
        eprintln!("Missing argument: needs a path to a folder.");
        process::exit(1);
    });

    let path_folder = PathBuf::from(path_folder);
    let output_folder = args.next().unwrap_or(String::from("./output"));
    let output_folder = PathBuf::from(output_folder);
    (path_folder, output_folder)
}
