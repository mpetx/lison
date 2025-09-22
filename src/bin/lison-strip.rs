
use std::env;
use std::fs;

use lison::image::*;

struct StripConfig {
    input: String,
    output: String
}

enum Config {
    Help,
    Strip(StripConfig)
}

fn parse_args(args: &[String]) -> Result<Config, String> {
    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        return Ok(Config::Help);
    }

    if args.len() == 1 {
        let input = args[0].clone();
        let output = format!("stripped-{}", input);
        Ok(Config::Strip(StripConfig { input, output }))
    } else if args.len() == 3 && args[0] == "-o" {
        let input = args[2].clone();
        let output = args[1].clone();
        Ok(Config::Strip(StripConfig { input, output }))
    } else {
        Err(String::from("invalid arguments."))
    }
}

const HELP_MESSAGE: &str = r#"usage: lison-strip [-h] [-o output] input
options:
  -h        : print help message.
  -o <file> : output file name."#;

fn flatten_shape(shapes: &mut Vec<Shape>, shape: &Shape) {
    match shape {
        Shape::Group(group) => {
            for child in group.content.iter() {
                flatten_shape(shapes, child);
            }
        },
        _ => {
            shapes.push(shape.clone());
        }
    }
}

fn strip_image(image: &mut Image) {
    image.editor = None;

    let mut shapes: Vec<Shape> = Vec::new();

    for shape in image.shapes.iter() {
        flatten_shape(&mut shapes, shape);
    }

    image.shapes = shapes;
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let conf = parse_args(&args[1..])?;

    match conf {
        Config::Help => {
            eprintln!("{}", HELP_MESSAGE);
        },
        Config::Strip(conf) => {
            let image_str = fs::read_to_string(&conf.input)
                .or_else(|_| Err(format!("failed to read '{}'.", &conf.input)))?;

            let mut image: Image = serde_json::from_str(&image_str)
                .or_else(|_| Err(format!("failed to parse '{}'.", &conf.input)))?;

            strip_image(&mut image);

            let stripped_image_str = serde_json::to_string(&image)
                .or_else(|_| Err(String::from("failed to strip the image.")))?;

            fs::write(&conf.output, &stripped_image_str)
                .or_else(|_| Err(format!("failed to write to '{}'.", &conf.output)))?;
        }
    }

    Ok(())
}
