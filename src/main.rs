use std::{
    fs::{self, read_to_string},
    path::PathBuf,
};

use chrono::NaiveDate;
use pulldown_cmark::{html, Options, Parser};

struct Post {
    html: String,
    title: String,
    date: NaiveDate,
    file_name: String,
}

impl Post {
    fn new(file_path: PathBuf, options: Options) -> Self {
        let template = fs::read_to_string("static/post-template.html").unwrap();

        let markdown = read_to_string(file_path.clone()).unwrap();

        let mut events = Parser::new_ext(&markdown, options);

        let title: String = events
            .find_map(|event| {
                if let pulldown_cmark::Event::Text(text) = event {
                    Some(text)
                } else {
                    None
                }
            })
            .unwrap()
            .to_string();

        let date_string: String = events
            .find_map(|event| {
                if let pulldown_cmark::Event::Text(text) = event {
                    Some(text)
                } else {
                    None
                }
            })
            .unwrap()
            .to_string();

        let date_parts: Vec<&str> = date_string.split('.').collect();

        let (month, day, year) = (
            date_parts[0].parse().unwrap(),
            date_parts[1].parse().unwrap(),
            format!("20{}", date_parts[2]).parse().unwrap(),
        );

        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();

        let template = template.replace("{title}", &title);

        let mut template_pieces = template.split("{body}");

        let mut html_string = String::new();

        html_string.push_str(template_pieces.next().unwrap());

        let parser = Parser::new_ext(&markdown, options);

        html::push_html(&mut html_string, parser);

        html_string.push_str(template_pieces.next().unwrap());

        let file_path = file_path
            .strip_prefix("markdown/")
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        Post {
            html: html_string,
            title,
            date,
            file_name: file_path[0..file_path.len() - 3].to_owned(),
        }
    }
}

fn main() {
    let markdown_files = fs::read_dir("markdown").unwrap();

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let mut posts = vec![];

    fs::remove_dir_all("posts").unwrap();
    fs::create_dir("posts").unwrap();

    for file in markdown_files {
        let file = file.unwrap();

        let post = Post::new(file.path(), options);

        let file_name = file.file_name().to_str().unwrap().to_owned();
        let output_file_path = format!(
            "posts/{}.html",
            file_name[0..file_name.len() - 3].to_owned()
        );

        fs::write(output_file_path, post.html.clone()).unwrap();

        posts.push(post);
    }

    posts.sort_by_key(|post| post.date);
    posts.reverse();

    let template = fs::read_to_string("static/index-template.html").unwrap();

    let mut template_pieces = template.split("{body}");

    let mut html_string = String::new();

    html_string.push_str(template_pieces.next().unwrap());

    for post in posts {
        html_string.push_str(&format!(
            "<div class=\"card\"><a href=\"posts/{}.html\">{}</a>",
            post.file_name, post.title
        ));
        html_string.push_str(&format!(
            "<p class=\"date\">{}</p></div>",
            post.date.format("%-m.%-d.%y")
        ));
    }

    html_string.push_str(template_pieces.next().unwrap());

    fs::write("index.html", html_string).unwrap();
}
