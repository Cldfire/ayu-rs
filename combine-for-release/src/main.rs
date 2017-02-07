use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

fn main() {
    let docsrs = include_str!("../../css/docs.rs.css");
    let playground = include_str!("../../css/playground.css");
    let rustdoc = include_str!("../../css/rustdoc.css");

    let output = "@-moz-document domain(\"docs.rs\") {\n".to_owned() + docsrs +
                 "\n}\n\n@-moz-document domain(\"play.rust-lang.org\"), \
                  domain(\"play.integer32.com\") {\n" + playground +
                 "\n}\n\n@-moz-document domain(\"doc.rust-lang.org\"), \
                  domain(\"docs.rs\") {\n" + rustdoc + "\n}";

    let mut file = File::create("ayu-rs_release.css").expect("Could not create / open output file");
    let mut writer = BufWriter::new(file);
    writer.write_all(output.as_bytes()).expect("Could not write output to file");
}
