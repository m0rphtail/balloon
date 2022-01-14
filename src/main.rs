use axum::{http::StatusCode, service, Router};
use clap::{arg, App, AppSettings};
use std::{convert::Infallible, fs, net::SocketAddr, path::Path, thread, time::Duration};
use tower_http::services::ServeDir;

mod templates;

const CONTENT_DIR: &str = "content";
const PUBLIC_DIR: &str = "public";

fn main() {
    let app = App::new("balloon")
        .bin_name("balloon")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            App::new("new")
                .about("Create a new project")
                .arg(arg!(<NAME> "Name of the project"))
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .subcommand(
            App::new("serve")
                .about("Start the server")
                .arg(arg!(<PORT> "Port to run server").default_value("8080")),
        );

    let matches = app.get_matches();
    match matches.subcommand() {
        Some(("serve", sub_matches)) => {
            println!(
                "Starting server on port {}",
                sub_matches.value_of("PORT").expect("no port specified")
            );
            start(
                sub_matches
                    .value_of("PORT")
                    .unwrap()
                    .parse::<u16>()
                    .unwrap(),
            )
            .unwrap();
        }
        Some(("new", sub_matches)) => {
            println!(
                "Creating new project: {}",
                sub_matches.value_of("NAME").expect("project name needed")
            );
        }
        _ => (),
    }
}

#[tokio::main]
pub async fn start(port: u16) -> Result<(), anyhow::Error> {
    rebuild_site(CONTENT_DIR, PUBLIC_DIR).expect("Rebuilding site");

    tokio::task::spawn_blocking(move || {
        println!("listenning for changes: {}", CONTENT_DIR);
        let mut hotwatch = hotwatch::Hotwatch::new().expect("hotwatch failed to initialize!");
        hotwatch
            .watch(CONTENT_DIR, |_| {
                println!("Rebuilding site");
                rebuild_site(CONTENT_DIR, PUBLIC_DIR).expect("Rebuilding site");
            })
            .expect("failed to watch content folder!");
        loop {
            thread::sleep(Duration::from_secs(1));
        }
    });

    let app = Router::new().nest(
        "/",
        service::get(ServeDir::new(PUBLIC_DIR)).handle_error(|error: std::io::Error| {
            Ok::<_, Infallible>((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            ))
        }),
    );

    let mut addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    addr.set_port(port);
    println!("serving site on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

pub fn rebuild_site(content_dir: &str, output_dir: &str) -> Result<(), anyhow::Error> {
    let _ = fs::remove_dir_all(output_dir);

    let markdown_files: Vec<String> = walkdir::WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().display().to_string().ends_with(".md"))
        .map(|e| e.path().display().to_string())
        .collect();
    let mut html_files = Vec::with_capacity(markdown_files.len());

    for file in &markdown_files {
        let mut html = templates::HEADER.to_owned();
        let markdown = fs::read_to_string(&file)?;
        let parser = pulldown_cmark::Parser::new_ext(&markdown, pulldown_cmark::Options::all()); //TODO: use a custom parser

        let mut body = String::new();
        pulldown_cmark::html::push_html(&mut body, parser);

        html.push_str(templates::render_body(&body).as_str());
        html.push_str(templates::FOOTER);

        let html_file = file
            .replace(content_dir, output_dir)
            .replace(".md", ".html");
        let folder = Path::new(&html_file).parent().unwrap();
        let _ = fs::create_dir_all(folder);
        fs::write(&html_file, html)?;

        html_files.push(html_file);
    }

    write_index(html_files, output_dir)?;
    Ok(())
}

fn write_index(files: Vec<String>, output_dir: &str) -> Result<(), anyhow::Error> {
    let mut html = templates::HEADER.to_owned();
    let body = files
        .into_iter()
        .map(|file| {
            let file = file.trim_start_matches(output_dir);
            let title = file.trim_start_matches("/").trim_end_matches(".html");
            format!(r#"<a href="{}">{}</a>"#, file, title)
        })
        .collect::<Vec<String>>()
        .join("<br />\n");

    html.push_str(templates::render_body(&body).as_str());
    html.push_str(templates::FOOTER);

    let index_path = Path::new(&output_dir).join("index.html");
    fs::write(index_path, html)?;
    Ok(())
}
