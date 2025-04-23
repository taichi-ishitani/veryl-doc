use miette::{GraphicalReportHandler, GraphicalTheme, Report, ThemeCharacters, ThemeStyles};
use semver::Version;
use std::collections::HashMap;
use std::path::PathBuf;
use veryl_analyzer::{Analyzer, namespace_table, symbol_table};
use veryl_emitter::Emitter;
use veryl_formatter::Formatter;
use veryl_metadata::{
    Build, BuildInfo, Doc, EnvVar, Format, Lint, Lockfile, Metadata, Project, Pubfile, Publish,
    Test,
};
use veryl_parser::{Parser, resource_table};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub struct Result {
    err: bool,
    content: String,
}

#[wasm_bindgen]
impl Result {
    #[wasm_bindgen]
    pub fn err(&self) -> bool {
        self.err
    }

    #[wasm_bindgen]
    pub fn content(&self) -> String {
        self.content.clone()
    }
}

fn render_err(err: Report) -> String {
    let mut out = String::new();
    GraphicalReportHandler::new_themed(GraphicalTheme {
        characters: ThemeCharacters::emoji(),
        styles: ThemeStyles::none(),
    })
    .with_width(80)
    .render_report(&mut out, err.as_ref())
    .unwrap();
    out
}

fn metadata() -> Metadata {
    Metadata {
        project: Project {
            name: "project".into(),
            version: Version::parse("0.0.0").unwrap(),
            authors: vec![],
            description: None,
            license: None,
            repository: None,
        },
        build: Build::default(),
        format: Format::default(),
        lint: Lint::default(),
        publish: Publish::default(),
        doc: Doc::default(),
        test: Test::default(),
        dependencies: HashMap::new(),
        metadata_path: "".into(),
        pubfile_path: "".into(),
        pubfile: Pubfile::default(),
        lockfile_path: "".into(),
        lockfile: Lockfile::default(),
        build_info: BuildInfo::default(),
        env_var: EnvVar::default(),
    }
}

#[wasm_bindgen]
pub fn set_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn build(source: &str) -> Result {
    let metadata = metadata();
    match Parser::parse(source, &"") {
        Ok(parser) => {
            if let Some(path) = resource_table::get_path_id(PathBuf::from("")) {
                symbol_table::drop(path);
                namespace_table::drop(path);
            }

            let analyzer = Analyzer::new(&metadata);
            let mut errors = Vec::new();
            errors.append(&mut analyzer.analyze_pass1("project", "", &parser.veryl));
            errors.append(&mut Analyzer::analyze_post_pass1());
            errors.append(&mut analyzer.analyze_pass2("project", "", &parser.veryl));
            let info = Analyzer::analyze_post_pass2();
            errors.append(&mut analyzer.analyze_pass3("project", "", &parser.veryl, &info));

            let err = !errors.is_empty();

            let content = if err {
                let mut text = String::new();
                for e in errors {
                    text.push_str(&render_err(e.into()));
                }
                text
            } else {
                let mut emitter = Emitter::new(
                    &metadata,
                    &PathBuf::from("input.veryl"),
                    &PathBuf::from("input.sv"),
                    &PathBuf::from("input.sv.map"),
                );
                emitter.emit("project", &parser.veryl);
                emitter.as_str().to_owned()
            };

            Result { err, content }
        }
        Err(e) => Result {
            err: true,
            content: render_err(e.into()),
        },
    }
}

#[wasm_bindgen]
pub fn format(source: &str) -> Result {
    let metadata = metadata();
    match Parser::parse(source, &"") {
        Ok(parser) => {
            let mut formatter = Formatter::new(&metadata);
            formatter.format(&parser.veryl);
            Result {
                err: false,
                content: formatter.as_str().to_owned(),
            }
        }
        Err(e) => Result {
            err: true,
            content: render_err(e.into()),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use wasm_bindgen_test::*;

    fn get_default_code() -> String {
        let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let mut path = PathBuf::from(path);
        path.push("html");
        path.push("index.html");
        let text = std::fs::read_to_string(path).unwrap();
        let mut code = false;
        let mut code_text = String::new();
        for line in text.lines() {
            if line.contains("</textarea") {
                code = false;
            }
            if code {
                code_text.push_str(&format!("{line}\n"));
            }
            if line.contains("<textarea") {
                code = true;
            }
        }
        code_text
    }

    const SRC: &str = "// module definition
module ModuleA #(
    param ParamA: u32 = 10,
    const ParamB: u32 = 10, // trailing comma is allowed
) (
    i_clk : input  clock            ,
    i_rst : input  reset            ,
    i_sel : input  logic            ,
    i_data: input  logic<ParamA> [2], // `[]` means unpacked array
    o_data: output logic<ParamA>    , // `<>` means packed array
) {
    // const parameter declaration
    //   `param` is not allowed in module
    const ParamC: u32 = 10;

    // variable declaration
    var r_data0: logic<ParamA>;
    var r_data1: logic<ParamA>;

    // value binding
    let _w_data2: logic<ParamA> = i_data[0];

    // always_ff statement with reset
    //   `always_ff` can take a mandatory clock and a optional reset
    //   `if_reset` means `if (i_rst)`. This conceals reset porality
    //   `()` of `if` is not required
    //   `=` in `always_ff` is non-blocking assignment
    always_ff (i_clk, i_rst) {
        if_reset {
            r_data0 = 0;
        } else if i_sel {
            r_data0 = i_data[0];
        } else {
            r_data0 = i_data[1];
        }
    }

    // always_ff statement without reset
    always_ff (i_clk) {
        r_data1 = r_data0;
    }

    assign o_data = r_data1;
}
";

    #[test]
    fn build_default_code() {
        let text = get_default_code();
        let ret = build(&text);

        assert_eq!(ret.err, false);
        assert_ne!(ret.content, "");
    }

    #[test]
    fn format_default_code() {
        let text = get_default_code();
        let ret = format(&text);

        assert_eq!(ret.err, false);
        assert_eq!(ret.content, text);
    }

    #[wasm_bindgen_test]
    fn build_on_wasm() {
        let ret = build(&SRC);

        assert_eq!(ret.err, false);
        assert_ne!(ret.content, "");
    }

    #[wasm_bindgen_test]
    fn format_on_wasm() {
        let ret = format(&SRC);

        assert_eq!(ret.err, false);
        assert_eq!(ret.content, SRC);
    }
}
