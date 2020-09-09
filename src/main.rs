use {
    anyhow::bail,
    clap::{App, AppSettings, Arg, ArgMatches, SubCommand},
};

enum Target {
    Windows,
    Xlib,
    Macos,
}

fn cli() -> anyhow::Result<(ArgMatches<'static>, Target)> {
    let target = if cfg!(windows) {
        Target::Windows
    } else if cfg!(target_os = "linux") {
        Target::Xlib
    } else if cfg!(target_os = "macos") {
        Target::Macos
    } else {
        bail!("Your OS is not supported at the moment")
    };

    Ok((App::new("wall")
        .version(clap::crate_version!())
        .author(clap::crate_authors!("\n"))
        .about(clap::crate_description!())
        .settings(&[
            AppSettings::ArgRequiredElseHelp,
            AppSettings::ColoredHelp,
            AppSettings::ColorAuto,
        ])
        .subcommand({
            let subcmd = SubCommand::with_name("set")
                .about("Set the provided image as the wallpaper");

            match target {
                Target::Windows | Target::Macos => subcmd.arg(
                        Arg::with_name("FULL_PATH")
                            .help("Full path to the image")
                            .index(1)
                            .required(true),
                    ),
                Target::Xlib => subcmd.arg(
                        Arg::with_name("PATH")
                            .help("Path to the image")
                            .index(1)
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("format")
                            .help("By default the image format will be guessed from the path, but with this option the format specified will be used.")
                            .short("f")
                            .long("format")
                            .value_name("FORMAT")
                            .possible_values(&[
                                "png", "jpeg", "gif", "bmp", "ico", "tiff", "webp", "pnm", "dds",
                                "tga", "farbfeld",
                            ]),
                    ),
        }})
        .subcommand({
            let subcmd = SubCommand::with_name("get")
                .about("Get's the current wallpaper");

            match target {
                Target::Windows | Target::Macos => subcmd,
                Target::Xlib => subcmd.arg(
                        Arg::with_name("PATH")
                            .help("Path where to save the image")
                            .index(1)
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("format")
                            .help("By default the image format will be guessed from the path, but with this option the format specified will be used.")
                            .short("f")
                            .long("format")
                            .value_name("FORMAT")
                            .possible_values(&[
                                "png", "jpeg", "gif", "bmp", "ico", "tiff", "pnm", "tga",
                                "farbfeld",
                            ]),
                    ),
        }})
        .get_matches(), target))
}

#[cfg(target_os = "linux")]
use wall::xlib::ImageFormat;
#[cfg(target_os = "linux")]
fn str_to_format(s: &str) -> Result<ImageFormat, &str> {
    match s {
        "png" => Ok(ImageFormat::Png),
        "jpeg" => Ok(ImageFormat::Jpeg),
        "gif" => Ok(ImageFormat::Gif),
        "bmp" => Ok(ImageFormat::Bmp),
        "ico" => Ok(ImageFormat::Ico),
        "tiff" => Ok(ImageFormat::Tiff),
        "pnm" => Ok(ImageFormat::Pnm),
        "tga" => Ok(ImageFormat::Tga),
        "farbfeld" => Ok(ImageFormat::Farbfeld),
        _ => Err(s),
    }
}

fn main() -> anyhow::Result<()> {
    let (matches, target) = cli()?;

    if let Some(matches) = matches.subcommand_matches("get") {
        match target {
            Target::Macos => {
                #[cfg(target_os = "macos")]
                {
                    let wall = wall::macos::get()?;
                    println!("{}", wall);
                };
            }
            Target::Windows => {
                #[cfg(windows)]
                {
                    let wall = wall::windows::get()?;
                    println!("{}", wall.display());
                };
            }
            Target::Xlib => {
                #[cfg(target_os = "linux")]
                {
                    let xlib = wall::xlib::Xlib::new()?;
                    let save_path = matches.value_of("PATH").unwrap();
                    let format = matches
                        .value_of("format")
                        .and_then(|s| str_to_format(s).ok());
                    xlib.get(save_path, format)?;
                };
            }
        }
        return Ok(());
    }

    if let Some(matches) = matches.subcommand_matches("set") {
        match target {
            Target::Macos => {
                #[cfg(target_os = "macos")]
                {
                    let full_path = matches.value_of("FULL_PATH").unwrap();
                    wall::macos::set(full_path)?;
                };
            }
            Target::Windows => {
                #[cfg(windows)]
                {
                    let full_path = matches.value_of("FULL_PATH").unwrap();
                    wall::windows::set(full_path)?;
                };
            }
            Target::Xlib => {
                #[cfg(target_os = "linux")]
                {
                    let xlib = wall::xlib::Xlib::new()?;
                    let src_path = matches.value_of("PATH").unwrap();
                    let format = matches
                        .value_of("format")
                        .and_then(|s| str_to_format(s).ok());
                    xlib.set(src_path, format)?;
                };
            }
        }
        return Ok(());
    }

    Ok(())
}
