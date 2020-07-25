#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}
fn main() {
    let args = Cli::from_args();

    let path = std::path::Path::new(&args.path);

    let language = path.extension().unwrap().to_str().unwrap();

    let content = std::fs::read_to_string(&args.path)
        .expect("could not read file");
    
    let mut converted = Vec::new();
    for line in content.lines() {
        converted.push(convert_line(language, line));
    }

    for line in converted {
        println!("{}", line);
    }
}

#[derive(Debug)]
struct CodeEmoji {
    pub find: &'static str,
    pub replace: &'static str,
}

impl CodeEmoji {
    fn new(find: &'static str, replace: &'static str) -> CodeEmoji {
        CodeEmoji { find, replace }
    }
}

lazy_static! {
    static ref CODE_EMOJIS_LANGUAGE_MAP: HashMap<&'static str,Vec<CodeEmoji>> = {
        let mut m = HashMap::new();
        // javascript
        let mut js = Vec::new();
        // rust
        let mut rs = Vec::new();
        // python
        let mut py = Vec::new();
        // java
        let mut java = Vec::new();

        // function
        let abacus = "🧮";
        js.push(CodeEmoji::new("function", abacus));
        rs.push(CodeEmoji::new("fn", abacus));
        py.push(CodeEmoji::new("def", abacus));

        // class
        let robot = "🤖";
        java.push(CodeEmoji::new("class", robot));

        // print
        let printer = "🖨️";
        js.push(CodeEmoji::new("log", printer));
        rs.push(CodeEmoji::new("println!", printer));
        py.push(CodeEmoji::new("print", printer));
        java.push(CodeEmoji::new("println", printer));

        // round brackets
        let melon = "🍈";
        let orange = "🍊";
        js.push(CodeEmoji::new("(", melon));
        rs.push(CodeEmoji::new("(", melon));
        py.push(CodeEmoji::new("(", melon));
        java.push(CodeEmoji::new("(", melon));
        js.push(CodeEmoji::new(")", orange));
        rs.push(CodeEmoji::new(")", orange));
        py.push(CodeEmoji::new(")", orange));
        java.push(CodeEmoji::new(")", orange));

        // squiggly brackets
        let grape = "🍇";
        let strawberry = "🍓";
        js.push(CodeEmoji::new("{", grape));
        rs.push(CodeEmoji::new("{", grape));
        py.push(CodeEmoji::new("{", grape));
        java.push(CodeEmoji::new("{", grape));
        js.push(CodeEmoji::new("}", strawberry));
        rs.push(CodeEmoji::new("}", strawberry));
        py.push(CodeEmoji::new("}", strawberry));
        java.push(CodeEmoji::new("}", strawberry));

        // square brackets
        let bread = "🍞";
        let waffle = "🧇";
        js.push(CodeEmoji::new("[", bread));
        rs.push(CodeEmoji::new("[", bread));
        py.push(CodeEmoji::new("[", bread));
        java.push(CodeEmoji::new("[", bread));
        js.push(CodeEmoji::new("]", waffle));
        rs.push(CodeEmoji::new("]", waffle));
        py.push(CodeEmoji::new("]", waffle));
        java.push(CodeEmoji::new("]", waffle));

        // public
        let unlocked = "🔓";
        let locked = "🔒";
        rs.push(CodeEmoji::new("pub", unlocked));
        java.push(CodeEmoji::new("public", unlocked));
        java.push(CodeEmoji::new("private", locked));

        // String
        let memo = "📝";
        rs.push(CodeEmoji::new("String", memo));
        java.push(CodeEmoji::new("String", memo));

        // quote
        let pencil = "✏️";
        js.push(CodeEmoji::new("'", pencil));
        rs.push(CodeEmoji::new("\"", pencil));
        py.push(CodeEmoji::new("\"", pencil));
        java.push(CodeEmoji::new("\"", pencil));

        // void
        let hole = "🕳️";
        java.push(CodeEmoji::new("void", hole));

        // return
        let house = "🏠";
        js.push(CodeEmoji::new("return", house));
        rs.push(CodeEmoji::new("return", house));
        py.push(CodeEmoji::new("return", house));
        java.push(CodeEmoji::new("return", house));

        // minus
        let minus = "➖";
        js.push(CodeEmoji::new("-", minus));
        rs.push(CodeEmoji::new("-", minus));
        py.push(CodeEmoji::new("-", minus));
        java.push(CodeEmoji::new("-", minus));

        // plus
        let plus = "➕";
        js.push(CodeEmoji::new("+", plus));
        rs.push(CodeEmoji::new("+", plus));
        py.push(CodeEmoji::new("+", plus));
        java.push(CodeEmoji::new("+", plus));

        // less
        let trend_up = "↗️";
        js.push(CodeEmoji::new("<", trend_up));
        rs.push(CodeEmoji::new("<", trend_up));
        py.push(CodeEmoji::new("<", trend_up));
        java.push(CodeEmoji::new("<", trend_up));

        // greater
        let trend_down = "↘️";
        js.push(CodeEmoji::new(">", trend_down));
        rs.push(CodeEmoji::new(">", trend_down));
        py.push(CodeEmoji::new(">", trend_down));
        java.push(CodeEmoji::new(">", trend_down));

        // equal
        let equal = "🏳️‍🌈";
        js.push(CodeEmoji::new("=", equal));
        rs.push(CodeEmoji::new("=", equal));
        py.push(CodeEmoji::new("=", equal));
        java.push(CodeEmoji::new("=", equal));

        // if
        let traffic_light = "🚦";
        js.push(CodeEmoji::new("if", traffic_light));
        rs.push(CodeEmoji::new("if", traffic_light));
        py.push(CodeEmoji::new("if", traffic_light));
        java.push(CodeEmoji::new("if", traffic_light));

        // else
        let stop_sign = "🛑";
        js.push(CodeEmoji::new("else", stop_sign));
        rs.push(CodeEmoji::new("else", stop_sign));
        py.push(CodeEmoji::new("el", stop_sign));
        py.push(CodeEmoji::new("else", stop_sign));
        java.push(CodeEmoji::new("else", stop_sign));

        // import
        let package = "📦";
        js.push(CodeEmoji::new("import", package));
        rs.push(CodeEmoji::new("crate", package));
        py.push(CodeEmoji::new("import", package));
        java.push(CodeEmoji::new("import", package));

        // int
        let numbers = "🔢";
        rs.push(CodeEmoji::new("i32", numbers));
        java.push(CodeEmoji::new("int", numbers));

        // static
        let plug = "🔌";
        rs.push(CodeEmoji::new("static", plug));
        java.push(CodeEmoji::new("static", plug));

        // throw
        let baseball = "⚾";
        js.push(CodeEmoji::new("throw", baseball));
        rs.push(CodeEmoji::new("panic!", baseball));
        py.push(CodeEmoji::new("raise", baseball));
        java.push(CodeEmoji::new("throw", baseball));

        // new
        let baby = "👶";
        js.push(CodeEmoji::new("new", baby));
        java.push(CodeEmoji::new("new", baby));

        m.insert("js", js);
        m.insert("rs", rs);
        m.insert("py", py);
        m.insert("java", java);

        m
    };
}

lazy_static! {
    static ref QUOTES_LANGUAGE_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("js", "\'");
        m.insert("rs", "\"");
        m.insert("py", "\"");
        m.insert("java", "\"");
        m
    };
}

fn convert_line(language: &str, line: &str) -> String {
    let quotes = QUOTES_LANGUAGE_MAP.get(language).unwrap();
    let mut converted = String::new();
    let mut in_quotes = false;
    for e in line.split(quotes) {
        if in_quotes {
            converted.push_str("✏️");
            converted.push_str(e);
            converted.push_str("✏️");
            in_quotes = false;
        } else {
            converted.push_str(convert_expression(language, e).as_str());
            in_quotes = true;
        }
    }
    converted
}

fn convert_expression(language: &str, expression: &str) -> String {
    let code_emojis = CODE_EMOJIS_LANGUAGE_MAP.get(language).unwrap();
    let mut converted = expression.to_owned();
    for ce in code_emojis {
        converted = converted.replace(ce.find, ce.replace);
    }
    converted
}