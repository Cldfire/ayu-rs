use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

fn main() {
    let crates_io = include_str!("../../css/crates.io.css");
    let rustbyexample = include_str!("../../css/rustbyexample.css");
    let doc_rust_lang = include_str!("../../css/doc.rust-lang.org.css");
    let crates_io_docs = include_str!("../../css/docs.crates.io.css");
    let docs_rs = include_str!("../../css/docs.rs.css");
    let playground = include_str!("../../css/playground.css");
    let rustdoc = include_str!("../../css/rustdoc.css");
    let twir = include_str!("../../css/twir.css");

    let usercss_header = "\n
/* ==UserStyle==
@name          ayu-rs
@description   Gorgeous dark theme for various Rust websites
@namespace     https://github.com/Cldfire
@version       1.5.11
@homepageURL   https://github.com/Cldfire/ayu-rs
@author        Cldfire (https://cldfire.dev)
@license       MIT
==/UserStyle== */\n
";

    let output = "/* do not edit this file by hand, generate it with the combine-for-release tool */\n\n".to_string() +
                usercss_header +
                "@-moz-document domain(\"docs.rs\") {\n"
                + docs_rs +
                "\n}\n\n\
                @-moz-document domain(\"play.rust-lang.org\"), domain(\"play.integer32.com\") {\n"
                + playground +
                "\n}\n\n\
                @-moz-document domain(\"doc.rust-lang.org\"), domain(\"docs.rs\"), url-prefix(\"https://manishearth.github.io/rust-internals-docs/\") {\n"
                + rustdoc +
                "\n}\n\n\
                @-moz-document domain(\"doc.rust-lang.org\") {\n"
                + doc_rust_lang +
                "\n}\n\n\
                @-moz-document domain(\"crates.io\") {\n"
                + crates_io +
                "\n}\n\n\
                @-moz-document domain(\"doc.crates.io\") {\n"
                + crates_io_docs +
                "\n}\n\n\
                @-moz-document regexp('https://doc\\.rust-lang\\.org(/.*/|/)rust-by-example/.*') {\n"
                + rustbyexample +
                "\n}\n\n\
                @-moz-document domain(\"this-week-in-rust.org\") {\n"
                + twir +
                "\n}";

    let file = File::create("../ayu-rs.user.css").expect("file");
    let mut writer = BufWriter::new(file);
    writer.write_all(output.as_bytes()).expect("written bytes");
}
