mod repl;

use {
    koto::{Koto, KotoSettings},
    repl::Repl,
    std::fs,
};

#[cfg(all(jemalloc, not(target_env = "msvc")))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn version_string() -> String {
    format!("Koto {}", env!("CARGO_PKG_VERSION"))
}

fn help_string() -> String {
    format!(
        "{version}

USAGE:
    koto [FLAGS] [script] [<args>...]

FLAGS:
    -i, --show_instructions  Show compiled instructions annotated with source lines
    -b, --show_bytecode      Show the script's compiled bytecode
    -t, --tests              Run the script's tests before running the script
    -h, --help               Prints help information
    -v, --version            Prints version information

ARGS:
    <script>     The koto script to run
    <args>...    Arguments to pass into the script
",
        version = version_string()
    )
}

#[derive(Default)]
struct KotoArgs {
    help: bool,
    version: bool,
    run_tests: bool,
    show_bytecode: bool,
    show_annotated: bool,
    script: Option<String>,
    script_args: Vec<String>,
}

fn parse_arguments() -> Result<KotoArgs, String> {
    let mut args = pico_args::Arguments::from_env();

    let help = args.contains(["-h", "--help"]);
    let version = args.contains(["-v", "--version"]);
    let run_tests = args.contains(["-t", "--tests"]);
    let show_bytecode = args.contains(["-b", "--show_bytecode"]);
    let show_annotated = args.contains(["-i", "--show_instructions"]);

    let script = args
        .subcommand()
        .map_err(|e| format!("Error while parsing arguments: {}", e))?;

    let script_args = match args.free() {
        Ok(extra_args) => extra_args,
        Err(e) => {
            return Err(match e {
                pico_args::Error::UnusedArgsLeft(unused) => {
                    format!("Unsupported argument: {}", unused.first().unwrap())
                }
                other => format!("Error while parsing arguments: {}", other),
            })
        }
    };

    Ok(KotoArgs {
        help,
        version,
        run_tests,
        show_bytecode,
        show_annotated,
        script,
        script_args,
    })
}

fn main() {
    let args = match parse_arguments() {
        Ok(args) => args,
        Err(error) => {
            println!("{}\n\n{}", help_string(), error);
            return;
        }
    };

    if args.help {
        println!("{}", help_string());
        return;
    }

    if args.version {
        println!("{}", version_string());
        return;
    }

    let mut settings = KotoSettings::default();
    settings.run_tests = args.run_tests;
    settings.show_bytecode = args.show_bytecode;
    settings.show_annotated = args.show_annotated;

    if let Some(script_path) = args.script {
        let mut koto = Koto::with_settings(settings);

        let mut prelude = koto.context().prelude.clone();
        prelude.add_map("json", koto_json::make_module());
        prelude.add_map("random", koto_random::make_module());
        prelude.add_map("tempfile", koto_tempfile::make_module());
        prelude.add_map("toml", koto_toml::make_module());

        let script = fs::read_to_string(&script_path).expect("Unable to load script");
        koto.set_script_path(Some(script_path.into()));
        match koto.compile(&script) {
            Ok(_) => match koto.run_with_args(&args.script_args) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e);
                }
            },
            Err(e) => {
                eprintln!("{}", koto.format_loader_error(e, &script));
            }
        }
    } else {
        let mut repl = Repl::with_settings(settings);
        repl.run();
    }
}
