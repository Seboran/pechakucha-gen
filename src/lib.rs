use std::{
    io::{self, Error},
    path::PathBuf,
};

use html_ast_generator::HtmlElement;

mod html_ast_generator;

/// À partir d'un chemin vers un répertoire, retourne une &str contenant un html
pub fn generate_presentation(path_folder: &PathBuf) -> Result<(String, Vec<PathBuf>), Error> {
    // Add script tag

    let images_path_list = get_list_of_image_paths_sorted(path_folder)?;

    let create_script_autonext = HtmlElement::create_script_autonext(
        images_path_list
            .iter()
            .map(|i| i.file_name().unwrap().to_str().unwrap().to_string())
            .collect(),
    );
    let child_body = vec![create_script_autonext];

    let output: HtmlElement = HtmlElement::Node(String::from("body"), child_body);
    Ok((output.generate_html().to_string(), images_path_list))
}

/// Retourne la liste des fichiers contenus dans le répertoire envoyé en argument.
fn get_list_of_image_paths_sorted(path_folder: &PathBuf) -> Result<Vec<PathBuf>, Error> {
    let mut images_path_list = path_folder
        .read_dir()?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    images_path_list.sort();
    Ok(images_path_list)
}
