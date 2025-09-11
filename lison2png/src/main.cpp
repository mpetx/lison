
#include <lison/parse.hpp>
#include <lison/render.hpp>

#include <cmath>
#include <cstddef>
#include <cstdlib>
#include <iostream>
#include <format>
#include <fstream>
#include <memory>
#include <string>
#include <string_view>
#include <variant>
#include <vector>

struct Error
{
    std::string message;
    int status;
};

struct Config
{
    std::string input_path;
    std::string output_path;
    double resolution;
    double scale;
};

struct Help
{
};

std::variant<Config, Help> parse_args(const std::vector<std::string> &args)
{
    std::size_t i = 0;
    std::string input, output;
    double resolution = 0, scale = 0;

    while (i < args.size())
    {
	const std::string &arg = args[i];

	if (arg == "-h" || arg == "--help")
	{
	    return Help { };
	}
	if (arg == "-o")
	{
	    if (i + 1 >= args.size())
	    {
		throw Error { "missing operand for '-o'.", 1 };
	    }

	    output = args[i + 1];
	    i += 2;
	}
	else if (arg == "-r")
	{
	    if (i + 1 >= args.size())
	    {
		throw Error { "missing operand for '-r'.", 1 };
	    }

	    resolution = std::atof(args[i + 1].c_str());
	    i += 2;
	}
	else if (arg == "-s")
	{
	    if (i + 1 >= args.size())
	    {
		throw Error { "missing operand for '-s'.", 1 };
	    }

	    scale = std::atof(args[i + 1].c_str());
	    i += 2;
	}
	else if (arg.size() != 0 && arg[0] == '-')
	{
	    throw Error { std::format("unknown option '{}'.", arg), 1 };
	}
	else
	{
	    break;
	}
    }

    if (i + 1 < args.size())
    {
	throw Error { "too many operands.", 1 };
    }
    else if (i + 1 > args.size())
    {
	throw Error { "missing operand.", 1 };
    }

    input = args[i];

    return Config {
	input,
	output.empty() ? std::format("{}.png", input) : output,
	resolution == 0 ? 72 : resolution,
	scale == 0 ? 1 : scale
    };
}

lison::Image create_image_from_path(const std::string &path)
{
    std::ifstream in { path };

    if (!in)
    {
	throw Error { std::format("failed to open '{}'.", path), 2 };
    }

    in.seekg(0, std::ios_base::end);
    auto size = static_cast<std::size_t>(in.tellg());
    in.seekg(0, std::ios_base::beg);

    auto content = std::make_unique<char[]>(size);
    in.read(content.get(), size);

    in.close();

    std::string_view text { content.get(), size };

    auto res = lison::parse(text);

    if (std::holds_alternative<lison::ParseFailure>(res))
    {
	throw Error { std::format("failed to parse '{}'.", path), 2 };
    }

    return std::get<lison::Image>(res);
}

cairo_surface_t *create_output_surface(const lison::Image &image, const Config &conf)
{
    auto transform = [&image, &conf](double x) {
	return static_cast<int>(std::round(x * conf.resolution / image.unit_per_inch * conf.scale));
    };

    int width = transform(image.width);
    int height = transform(image.height);

    return cairo_image_surface_create(CAIRO_FORMAT_ARGB32, width, height);
}

const std::string help_message {
    "usage: lison2png [-o output] [-r resolution] [-s scale] input\n"
    "options:\n"
    "  -h        : print help message.\n"
    "  -o <file> : output file name.\n"
    "  -r <num>  : resolution in ppi.\n"
    "  -s <num>  : magnification ratio.\n"
};

void print_help()
{
    std::cerr << help_message << std::flush;
}

int main(int argc, char **argv)
{
    std::vector<std::string> args { argv + 1, argv + argc };

    try
    {
	auto res = parse_args(args);

	if (std::holds_alternative<Help>(res))
	{
	    print_help();
	    return 0;
	}

	const Config &conf = std::get<Config>(res);

	lison::Image image = create_image_from_path(conf.input_path);

	cairo_surface_t *surface = create_output_surface(image, conf);
	cairo_t *cr = cairo_create(surface);

	lison::render(image, cr, conf.resolution, conf.scale);

	cairo_destroy(cr);
	cairo_surface_write_to_png(surface, conf.output_path.c_str());
	cairo_surface_destroy(surface);

	return 0;
    }
    catch (const Error &e)
    {
	std::cerr << std::format("lison2png: {}\n", e.message) << std::flush;

	return e.status;
    }
}
