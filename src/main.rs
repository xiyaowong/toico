use std::{
    fs::{self},
    path,
    sync::Arc,
};

use clap::{CommandFactory, Parser};
use image::{GenericImageView, ImageReader};
use std::thread;

#[derive(Parser, Debug)]
#[command(
    name = "toico",
    version,
    author,
    about = "Convert images to ICO format",
    after_help = r#"
Examples:
    toico image.png --size 64
    toico image.png -s 64 --output my_icon

Tips:
    Drag and drop an image file onto the executable to convert it to ICO format.
"#
)]
struct ToIconArgs {
    #[arg(help = "The image file to convert")]
    image: String,

    #[arg(
        short,
        long,
        default_value_t = 64,
        help = "The size of the icon in pixels: 16, 24, 32, 48, 64, 128, 256"
    )]
    size: u16,

    #[arg(
        short,
        long,
        help = r#"The output file name, defaults to the input file name

If the --all option is specified, files will be saved in a directory named <output>_ico
If --all is not specified, the file will be saved as <output>.ico
    "#
    )]
    output: Option<String>,

    #[arg(
        short = 'a',
        long = "all",
        help = "Whether to generate all icon sizes (16, 24, 32, 48, 64, 128, 256)"
    )]
    all: bool,

    #[arg(short = 'f', long = "force", help = "Force overwrite existing files")]
    force: bool,
}

/// # Returns
/// - `bool`: `true` if everything went well, `false` otherwise.
fn core(args: ToIconArgs) -> bool {
    let image_file = path::Path::new(&args.image);
    println!("Processing: {}", image_file.display());
    if !image_file.exists() {
        eprintln!("Error: The image file '{}' does not exist.", args.image);
        return false;
    }

    let image_name = image_file
        .file_stem()
        .unwrap_or(args.image.as_ref())
        .to_string_lossy()
        .to_string();

    let img_reader = match ImageReader::open(&args.image) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Error: Failed to open image file '{}': {}", args.image, e);
            return false;
        }
    };

    let img = match img_reader.decode() {
        Ok(img) => img,
        Err(e) => {
            match e {
                image::ImageError::Unsupported(_) => {
                    eprintln!(
                        "Error: The image format of '{}' is not supported.",
                        args.image
                    );
                }
                _ => {
                    eprintln!("Error: Failed to decode image file '{}': {}", args.image, e);
                }
            }
            return false;
        }
    };

    let output = args.output.unwrap_or(image_name);
    let output = output.trim_end_matches(".ico");

    if !args.all {
        let save_path = format!("{output}.ico");
        let save_path = path::Path::new(&save_path);
        return save_icon(&img, save_path, args.size.into(), args.force);
    }

    let output_dir = format!("{output}_ico");
    let output_dir = path::Path::new(&output_dir);

    if let Err(e) = fs::create_dir_all(&output_dir) {
        eprintln!(
            "Error: Failed to create output directory '{}': {}",
            output_dir.display(),
            e
        );
        return false;
    }

    let img = Arc::new(img);

    let mut handles = vec![];

    {
        let save_path = output_dir.join(format!("{output}.ico"));
        let img_clone = Arc::clone(&img);
        let handle =
            thread::spawn(move || save_icon(&*img_clone, &save_path, args.size.into(), args.force));
        handles.push(handle);
    }

    let sizes = [16, 24, 32, 48, 64, 128, 256];
    for size in sizes {
        let save_path = output_dir.join(format!("{output}_{size}.ico"));
        let img_clone = Arc::clone(&img);
        let handle = thread::spawn(move || save_icon(&*img_clone, &save_path, size, args.force));
        handles.push(handle);
    }

    handles
        .into_iter()
        .fold(true, |acc, handle| handle.join().unwrap_or(false) && acc)
}

/// # Returns
/// - `bool`: `true` if the icon was saved successfully, `false` otherwise.
fn save_icon(img: &image::DynamicImage, path: &path::Path, size: u32, force: bool) -> bool {
    if path.exists() && !force {
        eprintln!(
            "Warning: File '{}' already exists. Use --force to overwrite.",
            path.display()
        );
        return false;
    }

    let img = if img.dimensions() == (size, size) {
        img
    } else {
        &img.resize_exact(size, size, image::imageops::FilterType::Lanczos3)
    };

    println!(
        "Saving icon to: '{}' with size {}x{}",
        path.display(),
        size,
        size
    );

    if let Err(e) = img.save(path) {
        eprintln!("Error: Failed to save '{}': {}", path.display(), e);
        false
    } else {
        true
    }
}

fn main() {
    if std::env::args().any(|arg| arg == "-V" || arg == "--version") {
        println!("{}", ToIconArgs::command().get_version().unwrap());
        return;
    }

    let ok = if let Ok(args) = ToIconArgs::try_parse() {
        core(args)
    } else {
        _ = ToIconArgs::command().print_help();
        false
    };

    if ok {
        return;
    }

    // println!("\nPress Enter or wait 3 seconds to exit...");

    // let (tx, rx) = std::sync::mpsc::channel();

    // let tx1 = tx.clone();
    // thread::spawn(move || {
    //     thread::sleep(Duration::from_secs(3));
    //     _ = tx1.send(true);
    // });

    // thread::spawn(move || {
    //     let mut buf = [0u8; 1];
    //     if let Ok(_) = io::stdin().read(&mut buf) {
    //         _ = tx.send(true);
    //     }
    // });

    // _ = rx.recv();
}
