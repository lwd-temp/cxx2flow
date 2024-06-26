use clap::Parser;
use once_cell::sync::Lazy;

static NONE: &str = "None";
static LONG_VERSION: Lazy<String> = Lazy::new(|| {
    format!(
        "
Version:             {}
Build Timestamp:     {}
Build Git Describe:  {}
Commit SHA:          {}
Commit Date:         {}
Commit Branch:       {}
Cargo Target Triple: {}
",
        env!("CARGO_PKG_VERSION"),
        env!("VERGEN_BUILD_TIMESTAMP"),
        env!("VERGEN_GIT_DESCRIBE"),
        option_env!("VERGEN_GIT_SHA").unwrap_or(NONE),
        option_env!("VERGEN_GIT_COMMIT_TIMESTAMP").unwrap_or(NONE),
        option_env!("VERGEN_GIT_BRANCH").unwrap_or(NONE),
        env!("VERGEN_CARGO_TARGET_TRIPLE"),
    )
});
#[derive(Parser, Debug)]
#[clap(about, version, long_version(LONG_VERSION.as_str()) ,author, after_help("Note that you need to manually compile the dot file using graphviz to get SVG or PNG files.

EXAMPLES:
    cat main.cpp | cxx2flow | dot -Tsvg -o test.svg
    cxx2flow test.cpp | dot -Tpng -o test.png
    cxx2flow main.cpp my_custom_func | dot -Tsvg -o test.svg

Please give me star if this application helps you!
如果这个应用有帮助到你，请给我点一个 star！
https://github.com/Enter-tainer/cxx2flow
"))]
pub struct Args {
    #[clap(
        short,
        long,
        help(
            "Sets the output file.
If not specified, result will be directed to stdout.
e.g. graph.dot"
        )
    )]
    pub output: Option<String>,

    #[clap(
        short,
        long,
        help(
            "Sets the style of the flow chart.
If specified, output flow chart will have curly connection line."
        )
    )]
    pub curly: bool,

    #[clap(long, help("Use C preprocessor."))]
    pub cpp: bool,

    #[clap(short, long, help("Use tikz backend."))]
    pub tikz: bool,

    #[clap(short, long, help("Use d2 backend."))]
    pub d2: bool,

    #[clap(long, help("Dump AST(For debug purpose only)."))]
    pub dump_ast: bool,

    #[clap(help(
        "Sets the path of the input file. e.g. test.cpp
If not specified, cxx2flow will read from stdin."
    ))]
    pub input: Option<String>,

    #[clap(
        default_value("main"),
        help("The function you want to convert. e.g. main")
    )]
    pub function: String,
}
