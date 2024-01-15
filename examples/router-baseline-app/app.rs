#![allow(warnings)]
use {
    router::*,
    std::{env, path::PathBuf},
    tree_pack::TreePack,
};
const HELP: &str = "\
App

USAGE:
  app [OPTIONS] --number NUMBER [INPUT]..

FLAGS:
  -h, --help            Prints help information

OPTIONS:
  --number NUMBER       Sets a number
  --opt-number NUMBER   Sets an optional number
  --width WIDTH         Sets width [default: 10]

ARGS:
  <INPUT>
";

#[derive(Debug)]
struct AppArgs {
    number: u32,
    opt_number: Option<u32>,
    width: u32,
    input: Vec<std::path::PathBuf>,
}

fn parse_width(s: &str) -> Result<u32, String> {
    let w = s.parse().map_err(|_| "not a number")?;
    if w != 0 {
        Ok(w)
    } else {
        Err("width must be positive".to_string())
    }
}

fn main() {
    #[derive(Copy, Clone)]
    enum O {
        Number = 0,
        OptNumber,
        Width,
    }
    let summaries = vec![
        "",
        "Set a number (required)",
        "Set an optional number",
        "Set a width (non-zero, default 10)",
    ];
    let mut router = Router {
        tree: TreePack::new(),
        segments: &[Seg {
            name: 0,
            opt_groups: 0,
        }],
        actions: &[],
        options: &[
            Opt {
                kind: OptArgKind::Single,
                name: 1,
            },
            Opt {
                kind: OptArgKind::Single,
                name: 2,
            },
            Opt {
                kind: OptArgKind::Single,
                name: 3,
            },
        ],
        short_option_mappers: &[(0, 'n'), (1, 'o'), (2, 'w')],
        names: &["root", "number", "opt-number", "width"],
    };
    let mut args = AppArgs {
        number: 0,
        opt_number: None,
        width: 0,
        input: Vec::new(),
    };
    let (c) = parse_route(&router, RouteKind::CLI, env::args()).unwrap();
    for o in &c.option_args {
        match o.0 {
            0 => args.number = c.saved_args[o.1 as usize].parse::<u32>().unwrap(),
            1 => args.opt_number = c.saved_args[o.1 as usize].parse::<u32>().ok(),
            2 => args.number = c.saved_args[o.1 as usize].parse::<u32>().unwrap(),
            _ => {}
        }
    }
    let a = c.saved_args[match router.options[c.option_args.last().unwrap().0 as usize].kind {
        OptArgKind::Multiple => c.arg_ranges[c.option_args.last().unwrap().0 as usize].end,
        _ => c.option_args.last().unwrap().1 + 1,
    } as usize..]
        .iter()
        .map(|arg| arg.parse::<PathBuf>().unwrap_or_default())
        .collect::<Vec<PathBuf>>();

    println!("{:#?}", args.number);
    println!("{:#?}", args.opt_number);
    println!("{:#?}", args.width);
    if 10 < args.input.len() {
        println!("{:#?}", args.input.len());
    } else {
        println!("{:#?}", args);
    }
}
