
use std::env;
use std::fs;

use lison::image::*;
use lison::render::*;

struct ConvertConfig {
    input: String,
    output: String,
    resolution: f64,
    scale: f64
}

enum Config {
    Help,
    Convert(ConvertConfig)
}

fn parse_args(mut args: &[String]) -> Result<Config, String> {
    let mut output = String::new();
    let mut resolution = 96.0;
    let mut scale = 1.0;

    while !args.is_empty() {
        let arg = &args[0];

        match arg.as_str() {
            "-h" | "--help" => {
                return Ok(Config::Help);
            },
            "-o" => {
                if args.len() == 1 {
                    return Err(String::from("missing operand after '-o'."));
                }

                output = args[1].clone();
                args = &args[2..];
            },
            "-r" => {
                if args.len() == 1 {
                    return Err(String::from("missing operand after '-r'."));
                }

                resolution = args[1]
                    .parse()
                    .or_else(|_| Err(String::from("invalid resolution value.")))?;
                args = &args[2..];
            },
            "-s" => {
                if args.len() == 1 {
                    return Err(String::from("missing operand after '-s'."));
                }

                scale = args[1]
                    .parse()
                    .or_else(|_| Err(String::from("invalid scale value.")))?;
                args = &args[2..];
            },
            option if option.starts_with("-") => {
                return Err(format!("unknown option '{}'.", option));
            },
            _ => {
                break;
            }
        }
    }

    if args.is_empty() {
        return Err(String::from("missing operand."));
    } else if args.len() > 1 {
        return Err(String::from("too many operands."));
    }

    let input = args[0].clone();

    if output.is_empty() {
        output = format!("{}.png", &input);
    }

    Ok(Config::Convert(ConvertConfig { input, output, resolution, scale }))
}

const HELP_MESSAGE: &str = r#"usage: lison-to-png [-h] [-o output] [-r resolution] [-s scale] input
options:
  -h        : print help message.
  -o <file> : output file name.
  -r <num>  : resolution in ppi.
  -s <num>  : scale ratio."#;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let conf = parse_args(&args[1..])?;

    match conf {
        Config::Help => {
            eprintln!("{}", HELP_MESSAGE);
        },
        Config::Convert(conf) => {
            let image_str = fs::read_to_string(&conf.input)
                .or_else(|_| Err(format!("failed to read '{}'.", &conf.input)))?;

            let image: Image = serde_json::from_str(&image_str)
                .or_else(|_| Err(format!("failed to parse '{}'.", &conf.input)))?;

            let width = (image.width * conf.resolution / image.unit_per_inch * conf.scale).round();
            let height = (image.height * conf.resolution / image.unit_per_inch * conf.scale).round();

            if width <= 0.0 || width > i32::MAX.into() || height <= 0.0 || height > i32::MAX.into() {
                return Err(String::from("bad image dimension."));
            }

            let width = width as i32;
            let height = height as i32;

            let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, width, height)
                .or_else(|_| Err(String::from("surface creation failed.")))?;

            let context = cairo::Context::new(&surface)
                .or_else(|_| Err(String::from("context creation failed.")))?;

            render(&context, &image, conf.resolution, conf.scale)
                .or_else(|_| Err(String::from("rendering operation failed.")))?;

            let mut output_file = fs::File::create(&conf.output)
                .or_else(|_| Err(format!("failed to create '{}'.", &conf.output)))?;

            surface.write_to_png(&mut output_file)
                .or_else(|_| Err(format!("failed to write to '{}'.", &conf.output)))?;
        }
    }

    Ok(())
}
