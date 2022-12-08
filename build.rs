use flate2::write::GzEncoder;
use flate2::Compression;
use lol_html::element;
use lol_html::html_content::ContentType;
use lol_html::html_content::TextChunk;
use lol_html::text;
use lol_html::HtmlRewriter;
use lol_html::Settings;
use maud::html;
use maud::PreEscaped;
use maud::DOCTYPE;
use pulldown_cmark::html;
use pulldown_cmark::Options as CmarkOpts;
use pulldown_cmark::Parser;
use regex::Regex;
use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use syntect::highlighting::ThemeSet;
use syntect::html::ClassStyle;
use syntect::html::ClassedHTMLGenerator;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let temp_dir = PathBuf::from("temp");
    let css = fs::read_to_string("styles/missing.min.css")?;

    fs::create_dir_all(&temp_dir)?;

    // Preproccess Mermaid Diagrams
    for post in fs::read_dir(PathBuf::from("posts"))? {
        let post = post?;
        if post.file_type()?.is_dir() {
            continue;
        }
        let temp_path = temp_dir.join(post.file_name());
        fs::copy(post.path(), &temp_path)?;
        Command::new("npx")
            .arg("-p")
            .arg("@mermaid-js/mermaid-cli")
            .arg("mmdc")
            .arg("-i")
            .arg(&temp_path)
            .arg("-o")
            .arg(&temp_path)
            .status()?;
    }

    for post in fs::read_dir(&temp_dir)? {
        let post = post?;
        if post.file_type()?.is_dir() {
            fs::copy(post.path(), out_dir.join(post.path().file_name().unwrap()))?;
            continue;
        }

        // Just move the file if it's an svg
        if let Some("svg") = post.path().extension().and_then(|s| s.to_str()) {
            fs::copy(post.path(), out_dir.join(post.path().file_name().unwrap()))?;
            continue;
        }

        let post_contents = fs::read_to_string(post.path())?;
        let title = &post_contents.lines().next().unwrap().to_string()[2..];
        if title.is_empty() {
            panic!("No title")
        }

        let options = CmarkOpts::all();
        let ss = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();

        let mut body = String::new();
        html::push_html(&mut body, Parser::new_ext(&post_contents, options));

        let mut output = vec![];

        let quote_regex = Regex::new(r"&quot;")?;
        let lt_regex = Regex::new(r"&lt;")?;
        let gt_regex = Regex::new(r"&gt;")?;
        let amp_regex = Regex::new(r"&amp;")?;
        let lang_highlight = move |txt: &mut TextChunk<'_>, lang: &str| {
            let txt_str = txt.as_str();
            let txt_str = quote_regex.replace_all(&txt_str, "\"");
            let txt_str = lt_regex.replace_all(&txt_str, "<");
            let txt_str = gt_regex.replace_all(&txt_str, ">");
            let txt_str = amp_regex.replace_all(&txt_str, "&");
            let syntax = ss.find_syntax_by_name(lang).unwrap();
            let mut html_generator = ClassedHTMLGenerator::new_with_class_style(
                syntax,
                &ss,
                ClassStyle::SpacedPrefixed { prefix: "syntect" },
            );
            for line in LinesWithEndings::from(&txt_str) {
                html_generator
                    .parse_html_for_line_which_includes_newline(line)
                    .unwrap();
            }
            let output_html = html_generator.finalize();
            txt.replace(&output_html, ContentType::Html);
            Ok(())
        };

        let mut rewriter = HtmlRewriter::new(
            Settings {
                element_content_handlers: vec![
                    element!("pre", |el| {
                        el.set_attribute("class", "syntectcode")?;
                        Ok(())
                    }),
                    text!("code.language-rust", |txt| lang_highlight(txt, "Rust")),
                    text!("code.language-python", |txt| lang_highlight(txt, "Python")),
                    // Hack until I can get more language highlighting
                    // definitions in the pipeline somehow
                    text!("code.language-toml", |txt| lang_highlight(txt, "Rust")),
                    text!("code.language-bash", |txt| lang_highlight(
                        txt,
                        "Shell-Unix-Generic"
                    )),
                    element!("code", |el| {
                        el.remove_attribute("class");
                        el.remove_and_keep_content();
                        Ok(())
                    }),
                ],
                ..Settings::default()
            },
            |c: &[u8]| output.extend_from_slice(c),
        );

        rewriter.write(body.as_bytes())?;
        rewriter.end()?;
        let body = String::from_utf8(output)?;

        let lang_css = syntect::html::css_for_theme_with_class_style(
            &ts.themes["base16-ocean.dark"],
            ClassStyle::SpacedPrefixed { prefix: "syntect" },
        )?;
        let contents = html! {
          (DOCTYPE)
          meta charset="utf-8";
          meta name="theme-color" content="#ffffff";
          meta name="viewport" content="width=device-width, initial-scale=1.0";
          meta name="description" content=(title);
          title { (title) }
          style {
            (PreEscaped(&css))
            (PreEscaped(&lang_css))
          }
          body {
            header."navbar" {
              p { "RATSM" }
              nav."contents" aria-label="Site Sections" {
                ul role="list" {
                  li {
                    a href="/" { "Home" };
                  }
                  li {
                    a href="/about" { "About" };
                  }
                  li {
                    a href="/contact" { "Contact" };
                  }
                }
              }
            }
            main {
              (PreEscaped(body))
            }
          }
          footer {
            "Made with ☕ by Michael Gattozzi"
          }
        }
        .into_string();
        // Get the name of the post and remove the `.md` prefix
        let file_name = post.file_name().to_str().unwrap().to_owned();
        let size = file_name.len() - 3;
        let name = &file_name[0..size];

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(contents.as_bytes())?;
        let contents = encoder.finish()?;

        fs::write(&out_dir.join(name), contents).unwrap();
    }

    fs::remove_dir_all(temp_dir)?;

    Ok(())
}
