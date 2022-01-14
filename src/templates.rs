pub const HEADER: &str = r#"<!DOCTYPE html>
<html lang="en" data-theme="dark">

  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="stylesheet" href="https://unpkg.com/@picocss/pico@latest/css/pico.min.css">
  </head>

"#;

pub fn render_body(body: &str) -> String {
  format!(
    r#" <body>
            <main class="container">
              <nav>
              <ul>
                <li><h1><a data-tooltip="Click to go to /" class="contrast" href="/">MorphTail</a></h1></li>
              </ul>
              <ul>
              <li>🌎</li>
              <li>📨</li>
              <li>🐦</li>
              <li>📷</li>
              <li>💻</li>
              <li>💼</li>
              </ul>
              </nav>             
              <article>
                {}      
                <article>hello</article>
                </article>
                </main>
        </body>"#,
    body
  )
}

pub const FOOTER: &str = r#"

</html>
"#;

//TODO: separate template