use std::collections::HashMap;
use crate::image;

#[derive(Debug)]
pub enum Command {
    Blur,
    Brighten,
    Crop,
    Rotate,
    Invert,
    Grayscale,
    Fractal,
    Generate,
    None
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // or, alternatively:
        std::fmt::Debug::fmt(self, f)
    }
}

pub struct CommandLineArgs {
    pub command_type: Command,
    pub args: HashMap<String, String>
}

impl CommandLineArgs {
    pub fn process_args_by_arg_list(
        self: &mut Self, 
        arg_list: &Vec<&'static str>,
        command_line_args_str: &mut Vec<String>,
    ) -> Result<(), &'static str> {
        for (i, arg) in arg_list.iter().enumerate() {
            self.args.insert(
                arg.to_string(), 
                match command_line_args_str.get_mut(i + 1) {
                    Some(n) => n.to_string(),
                    None => {
                        return Err(
                            Box::leak(
                                format!("Missing {} argument at arg pos {}!", arg, i)
                                .into_boxed_str()
                            )
                        );
                    }
                }
            );
        }
        Ok(())
    }
}

pub struct CommandOp {
    pub command_type: Command,
    pub command: fn(CommandLineArgs) -> Result<(), &'static str> 
}

pub struct Parser { }

impl Parser {
    pub fn parse(self: &Self, command_line_args_str: &mut Vec<String>) 
        -> Result<(CommandOp, CommandLineArgs), &'static str> {
        
        // Set Subcommand
        let subcommand: String = match command_line_args_str.get_mut(0) {
            Some(n) => n.to_string(),
            None => { return Err("Missing subcommand argument!") }
        };
        
        // Set Infile and Outfile
        let mut command_line_args: CommandLineArgs = CommandLineArgs{
            command_type: Command::None,
            args: HashMap::new(),
        };

        // Set command specific args
        match subcommand.as_str() {
            "blur" => {
                command_line_args.command_type = Command::Blur;
                let arg_list = vec!["infile", "outfile", "blur_value"];

                if let Err(e) = command_line_args.process_args_by_arg_list(
                    &arg_list, 
                    command_line_args_str
                ) {
                    return Err(e);
                };

                let command_op = CommandOp {
                    command_type: Command::Blur,
                    command: image::blur
                };

                Ok((command_op, command_line_args))
            },
            "brighten" => {
                command_line_args.command_type = Command::Brighten;

                let arg_list = vec!["infile", "outfile", "brighten_value"];

                if let Err(e) = command_line_args.process_args_by_arg_list(
                    &arg_list, 
                    command_line_args_str
                ) {
                    return Err(e);
                };

                let command_op = CommandOp {
                    command_type: Command::Brighten,
                    command: image::brighten
                };

                Ok((command_op, command_line_args))
            },
            "crop" => {
                command_line_args.command_type = Command::Crop;

                let arg_list = vec![
                    "infile", 
                    "outfile", 
                    "x",
                    "y",
                    "width",
                    "height"
                ];

                if let Err(e) = command_line_args.process_args_by_arg_list(
                    &arg_list, 
                    command_line_args_str
                ) {
                    return Err(e);
                };
                let command_op = CommandOp {
                    command_type: Command::Crop,
                    command: image::crop
                };

                Ok((command_op, command_line_args))
            },
            "rotate" => {
                command_line_args.command_type = Command::Rotate;

                let arg_list = vec!["infile", "outfile", "deg_rotation"];

                if let Err(e) = command_line_args.process_args_by_arg_list(
                    &arg_list, 
                    command_line_args_str
                ) {
                    return Err(e);
                };

                let command_op = CommandOp {
                    command_type: Command::Rotate,
                    command: image::rotate
                };

                Ok((command_op, command_line_args))
            },
            "invert" => {
                command_line_args.command_type = Command::Invert;

                let arg_list = vec!["infile", "outfile"];

                if let Err(e) = command_line_args.process_args_by_arg_list(
                    &arg_list, 
                    command_line_args_str
                ) {
                    return Err(e);
                };

                let command_op = CommandOp {
                    command_type: Command::Invert,
                    command: image::invert
                };

                Ok((command_op, command_line_args))
            },

            "grayscale" => {
                command_line_args.command_type = Command::Grayscale;
                let arg_list = vec!["infile", "outfile"];

                if let Err(e) = command_line_args.process_args_by_arg_list(
                    &arg_list, 
                    command_line_args_str
                ) {
                    return Err(e);
                };

                let command_op = CommandOp {
                    command_type: Command::Grayscale,
                    command: image::grayscale
                };

                Ok((command_op, command_line_args))
            },
            "generate" => {
                command_line_args.command_type = Command::Generate;
                let arg_list = vec!["outfile", "R", "G", "B"];

                if let Err(e) = command_line_args.process_args_by_arg_list(
                    &arg_list, 
                    command_line_args_str
                ) {
                    return Err(e);
                };

                let command_op = CommandOp {
                    command_type: Command::Generate,
                    command: image::generate
                };

                Ok((command_op, command_line_args))
            },
            "fractal" => {
                command_line_args.command_type = Command::Fractal;

                let arg_list = vec!["outfile"];

                if let Err(e) = command_line_args.process_args_by_arg_list(
                    &arg_list, 
                    command_line_args_str
                ) {
                    return Err(e);
                };

                let command_op = CommandOp {
                    command_type: Command::Fractal,
                    command: image::fractal
                };

                Ok((command_op, command_line_args))
            },
            _ => {
                Err("Mismatched Type!")
            }
        }
    }
}
