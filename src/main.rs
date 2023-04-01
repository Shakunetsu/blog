use std::{
    fs::{self, read, read_to_string},
    path::PathBuf,
};

use pulldown_cmark::{html, Options, Parser};

struct Post {
    html: String,
    title: String,
    file_name: String,
}

impl Default for Post {
    fn default() -> Self {
        Post {
            html: String::new(),
            title: String::new(),
            file_name: String::new(),
        }
    }
}

type Posts = Vec<Post>;

fn main() {
    let markdown_files = fs::read_dir("markdown").unwrap();

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let mut posts = vec![];

    for file in markdown_files {
        let file = file.unwrap();

        let post = create_post(file.path(), options);

        let file_name = file.file_name().to_str().unwrap().to_owned();
        let output_file_path = format!("posts/{}.html", file_name[0..file_name.len() - 3].to_owned());

        fs::write(output_file_path, post.html.clone()).unwrap();

        posts.push(post);
    }

    let index_html = create_index(posts);
    fs::write("index.html", index_html).unwrap();
}

fn create_index(posts: Posts) -> String {
    let template = fs::read_to_string("static/index-template.html").unwrap();

    let mut template_pieces = template.split("{body}");

    let mut html_string = String::new();
    
    html_string.push_str(template_pieces.next().unwrap());

    for post in posts {
        html_string.push_str(&format!("<div class=\"card\"><a href=\"posts/{}.html\">{}</a></div>", post.file_name, post.title));
    }

    html_string.push_str(template_pieces.next().unwrap());

    return html_string;
}

fn create_post(file_path: PathBuf, options: Options) -> Post {
    let template = fs::read_to_string("static/post-template.html").unwrap();

    let mut template_pieces = template.split("{body}");

    let mut html_string = String::new();
    
    html_string.push_str(template_pieces.next().unwrap());

    let markdown = read_to_string(file_path.clone()).unwrap();
    let parser = Parser::new_ext(&markdown, options);

    let mut title = String::new();

    for event in  Parser::new_ext(&markdown, options) {
        match event {
            pulldown_cmark::Event::Text(text) => {
                title = text.to_string();
                break;
            }
            _ => {}
        }
    }

    html::push_html(&mut html_string, parser);

    html_string.push_str(template_pieces.next().unwrap());

    let file_path = file_path.strip_prefix("markdown/").unwrap().to_str().unwrap().to_owned();

    Post {
        html: html_string,
        title,
        file_name: file_path[0..file_path.len() - 3].to_owned(),
    }
}
