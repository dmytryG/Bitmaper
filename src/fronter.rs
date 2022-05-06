pub fn generate_HTML(layers: Vec<(String, String)>, filename: String) -> String{
    let mut res = format!("<html>\n<head>\n<link href=\"./{}.css\" rel=\"stylesheet\">\n</head>\n<body>\n<div id=\"pic\">\n", filename);
    for layer in layers {
        res = res + format!("\t<section class=\"{}\"></section>\n", layer.0.clone().split('.').next().unwrap().to_string()).as_str();
    }
    res = res + format!(
        "</div>\n</body>\n</html>"
    ).as_str();
    res
}

pub fn generate_CSS(layers: Vec<(String, String)>) -> String{
    let mut res = std::fs::read_to_string("cssBase").unwrap();
    res = res + "\n";
    for layer in layers {
        res = res + layer.1.as_str() + "\n";
    }
    res
}