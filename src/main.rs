use std::{
    fs::{self, read, read_to_string},
    path::PathBuf,
};

use pulldown_cmark::{html, Options, Parser};

fn main() {
    let markdown_files = fs::read_dir("markdown").unwrap();

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    for file in markdown_files {
        let file = file.unwrap();

        let html = create_post(file.path(), options);
        let file_name = file.file_name().to_str().unwrap().to_owned();
        let output_file_path = format!("posts/{}.html", file_name[0..file_name.len() - 3].to_owned());
        fs::write(output_file_path, html).unwrap();
    }

    let index_html = create_index();
    fs::write("index.html", index_html).unwrap();
}

fn create_index() -> String {
    let template = fs::read_to_string("static/index-template.html").unwrap();

    let mut template_pieces = template.split("{body}");

    let mut html_string = String::new();
    
    html_string.push_str(template_pieces.next().unwrap());

    let posts = fs::read_dir("posts").unwrap();

    for post in posts {
        let file_name = post.unwrap().file_name().to_str().unwrap().to_owned();
        html_string.push_str(&format!("<a href=\"posts/{}\">{}</a>", file_name, &file_name[0..file_name.len() - 5].to_owned()));
    }

    html_string.push_str(template_pieces.next().unwrap());

    return html_string;
}

fn create_post(file_path: PathBuf, options: Options) -> String {
    let template = fs::read_to_string("static/post-template.html").unwrap();

    let mut template_pieces = template.split("{body}");

    let mut html_string = String::new();
    
    html_string.push_str(template_pieces.next().unwrap());

    let markdown = read_to_string(file_path).unwrap();
    let parser = Parser::new_ext(&markdown, options);

    html::push_html(&mut html_string, parser);

    html_string.push_str(template_pieces.next().unwrap());

    return html_string;
}
