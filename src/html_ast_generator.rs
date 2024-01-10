#[derive(Debug)]
pub enum HtmlElement {
    Node(String, Vec<HtmlElement>),
    Leaf(String),
}

impl HtmlElement {
    pub fn generate_html(&self) -> String {
        match self {
            HtmlElement::Node(balise, children) => {
                let collect: Vec<String> =
                    children.iter().map(|child| child.generate_html()).collect();

                let html = format!("<{balise}>{}</{balise}>", collect.join(""));
                html
            }
            HtmlElement::Leaf(content) => content.to_string(),
        }
    }

    /// document.querySelector("body").setAttribute("style", "background-color: black;")
    pub fn create_script_autonext(images_path: Vec<String>) -> HtmlElement {
        let script_content = String::from(
            r#"
    let images =  ##IMAGES##
    function set_background(url) {
    document.querySelector("body").setAttribute("style", `background-image: url("${url}"); background-repeat: no-repeat; background-position: center;`)
    }
    set_background(images[0]);
    let index = 1
    function autoPlay() {
    setTimeout(() => {
        set_background(images[index])
        index = (index + 1)
        if (index < images.length) {
        autoPlay()
        }
    }, ##TEMPS_TRANSITION## * 1000)
    }

    autoPlay()
      "#,
        );
        let list_images_javascript = format!(
            "[{}]",
            images_path
                .iter()
                .map(|i| format!("\"{}\"", i).to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
        let script_content = script_content.replace("##IMAGES##", &list_images_javascript);
        let script_content = script_content.replace("##TEMPS_TRANSITION##", "20");
        HtmlElement::Node(
            String::from("script"),
            vec![HtmlElement::Leaf(script_content)],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn affiche_un_élément_simple() {
        let leaf = HtmlElement::Leaf(String::from("Mon joli texte"));
        let generate = leaf.generate_html();
        assert_eq!(generate, String::from("Mon joli texte"));
    }

    #[test]
    fn affiche_une_node_html() {
        let leaf = HtmlElement::Leaf(String::from("Mon joli texte"));
        let node = HtmlElement::Node(String::from("balise"), vec![leaf]);
        assert_eq!(
            node.generate_html(),
            String::from("<balise>Mon joli texte</balise>")
        )
    }

    #[test]
    fn affiche_un_arbre_avec_deux_enfants() {
        let leaf1 = HtmlElement::Leaf(String::from("Mon joli texte 1"));
        let leaf2 = HtmlElement::Leaf(String::from("Mon joli texte 2"));
        let node = HtmlElement::Node(String::from("bar"), vec![leaf1, leaf2]);
        assert_eq!(
            node.generate_html(),
            String::from("<bar>Mon joli texte 1Mon joli texte 2</bar>")
        )
    }

    #[test]
    fn affiche_un_arbre_composé_de_profondeur_deux() {
        let leaf1 = HtmlElement::Leaf(String::from("Mon joli texte 1"));
        let leaf2 = HtmlElement::Leaf(String::from("Mon joli texte 2"));
        let jolie_node = HtmlElement::Node(String::from("bar"), vec![leaf1, leaf2]);

        let leaf1_moche = HtmlElement::Leaf(String::from("Mon moche texte 1"));
        let leaf2_moche = HtmlElement::Leaf(String::from("Mon moche texte 2"));
        let moche_node = HtmlElement::Node(String::from("foo"), vec![leaf1_moche, leaf2_moche]);

        let node = HtmlElement::Node(String::from("parent"), vec![jolie_node, moche_node]);
        assert_eq!(node.generate_html(), String::from("<parent><bar>Mon joli texte 1Mon joli texte 2</bar><foo>Mon moche texte 1Mon moche texte 2</foo></parent>"))
    }

    #[test]
    fn doit_creer_script() {
        let create_script_autonext = HtmlElement::create_script_autonext(vec![
            String::from("image1"),
            String::from("image2"),
        ]);
        assert!(
            create_script_autonext
                .generate_html()
                .contains("[\"image1\",\"image2\"]"),
            "{:?}",
            create_script_autonext
        );
        assert!(
            create_script_autonext.generate_html().contains("20 * 1000"),
            "{:?}",
            create_script_autonext
        );
    }
}
