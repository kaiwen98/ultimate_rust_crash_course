use crate::parser::CommandLineArgs;

pub fn blur(command_line_args: CommandLineArgs) -> Result<(), &'static str> {
    // Here's how you open an existing image file
    let img = image::open(command_line_args.args.get("infile").unwrap()).expect("Failed to open INFILE.");
    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.
    let input_value =  command_line_args.args
        .get("blur_value")
        .unwrap();

    match input_value
        .parse::<f32>() {
        Ok(n) => {
            let img2 = img.blur(n);
            img2.save(command_line_args.args.get("outfile").unwrap()).expect("Failed writing OUTFILE.");
            return Ok(());
        },
        Err(_e) => {
            let err_msg = format!("Parsing failed! Cannot read blur_value: {}", input_value);
            return Err(Box::leak(err_msg.into_boxed_str()));
        }
    }
}

pub fn brighten(command_line_args: CommandLineArgs) -> Result<(), &'static str> {
    // Here's how you open an existing image file
    let img = image::open(command_line_args.args.get("infile").unwrap()).expect("Failed to open INFILE.");
    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.
    let input_value =  command_line_args.args
        .get("brighten_value")
        .unwrap();

    match input_value
        .parse::<i32>() {
        Ok(n) => {
            let img2 = img.brighten(n);
            img2.save(command_line_args.args.get("outfile").unwrap()).expect("Failed writing OUTFILE.");
            return Ok(());
        },
        Err(_e) => {
            let err_msg = format!("Parsing failed! Cannot read blur_value: {}", input_value);
            return Err(Box::leak(err_msg.into_boxed_str()));
        }
    }
}

pub fn crop(command_line_args: CommandLineArgs) -> Result<(), &'static str> {

    // See blur() for an example of how to open an image.

    // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
    // You may hard-code them, if you like.  It returns a new image.

    // Challenge: parse the four values from the command-line and pass them
    // through to this function.

    // See blur() for an example of how to save the image.
    // Here's how you open an existing image file
    let mut img = image::open(command_line_args.args.get("infile").unwrap()).expect("Failed to open INFILE.");
    let arg_list = vec!["x", "y", "height", "width"];
    let mut value_list: Vec<u32> = Vec::new();

    for arg in arg_list.iter() {
        let input_value =  command_line_args.args
            .get(*arg)
            .unwrap();

        match input_value
            .parse::<u32>() {
            Ok(n) => {
                value_list.push(n)
            },
            Err(_e) => {
                let err_msg = format!("Parsing failed! Cannot read <{}>: {}", arg, input_value);
                return Err(Box::leak(err_msg.into_boxed_str()));
            }
        }
    }

    let img2 = img.crop(value_list[0], value_list[1], value_list[2], value_list[3]);
    img2.save(command_line_args.args.get("outfile").unwrap()).expect("Failed writing OUTFILE.");
    return Ok(());
}

pub fn rotate(command_line_args: CommandLineArgs) -> Result<(), &'static str> {
    // See blur() for an example of how to open an image.

    // There are 3 rotate functions to choose from (all clockwise):
    //   .rotate90()
    //   .rotate180()
    //   .rotate270()
    // All three methods return a new image.  Pick one and use it!

    // Challenge: parse the rotation amount from the command-line, pass it
    // through to this function to select which method to call.

    // See blur() for an example of how to save the image.
    
    let mut img = image::open(command_line_args.args.get("infile").unwrap()).expect("Failed to open INFILE.");
    
    let arg_list = vec!["deg_rotation"];
    let mut value_list: Vec<u32> = Vec::new();

    for arg in arg_list.iter() {
        let input_value =  command_line_args.args
            .get(*arg)
            .unwrap();

        match input_value
            .parse::<u32>() {
            Ok(n) => {
                value_list.push(n)
            },
            Err(_e) => {
                let err_msg = format!("Parsing failed! Cannot read <{}>: {}", arg, input_value);
                return Err(Box::leak(err_msg.into_boxed_str()));
            }
        }
    }

    let img2 = match value_list[0] % 360 {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        0 => img,
        _ => {
            let err_msg = format!("Parsing failed! Angle degree must be multiples of 90...");
            return Err(Box::leak(err_msg.into_boxed_str()));
        }
    };

    img2.save(command_line_args.args.get("outfile").unwrap()).expect("Failed writing OUTFILE.");
    return Ok(());
}

pub fn invert(command_line_args: CommandLineArgs) -> Result<(), &'static str> {
    // See blur() for an example of how to open an image.

    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.

    // See blur() for an example of how to save the image.
    // Here's how you open an existing image file
    let mut img = image::open(command_line_args.args.get("infile").unwrap()).expect("Failed to open INFILE.");
    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.

    img.invert();
    img.save(command_line_args.args.get("outfile").unwrap()).expect("Failed writing OUTFILE.");
    Ok(())
}

pub fn grayscale(command_line_args: CommandLineArgs) -> Result<(), &'static str> {
    // See blur() for an example of how to open an image.

    // .grayscale() takes no arguments. It returns a new image.

    // See blur() for an example of how to save the image.
    let img = image::open(command_line_args.args.get("infile").unwrap()).expect("Failed to open INFILE.");
    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.

    let img2 = img.grayscale();
    img2.save(command_line_args.args.get("outfile").unwrap()).expect("Failed writing OUTFILE.");
    Ok(())
}

pub fn generate(command_line_args: CommandLineArgs) -> Result<(), &'static str> {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;
        let green = (0.3 * if y>x {y} else {x} as f32) as u8;

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }
    imgbuf.save(command_line_args.args.get("outfile").unwrap()).unwrap();

    Ok(())
    // Create an ImageBuffer -- see fractal() for an example

    // Iterate over the coordinates and pixels of the image -- see fractal() for an example

    // Set the image to some solid color. -- see fractal() for an example

    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.

    // Challenge 2: Generate something more interesting!

    // See blur() for an example of how to save the image
}

// This code was adapted from https://github.com/PistonDevelopers/image
pub fn fractal(command_line_args: CommandLineArgs) -> Result<(), &'static str> {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(command_line_args.args.get("outfile").unwrap()).unwrap();
    Ok(())
}
