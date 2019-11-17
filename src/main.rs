use std::fs::File;
use std::io;
use std::io::Write;

use std::process::{Command, Stdio};

use scraper::{Html, Selector};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(name = "PACKAGE")]
    package_name: String,
}

#[derive(Clone)]
struct PackageInfo {
    pub name: String,
    pub url: String,
}

async fn download_package(pkg: &PackageInfo, path: &str) -> Result<File, String> {
    // TODO(smolck): Add shasum checking or whatever.
    let bytes = reqwest::get(&pkg.url)
        .await.expect("Couldn't get package.")
        .bytes()
        .await.expect("Couldn't get package.");


    let file = File::create(&path);
    let mut file = match file {
        Ok(file) => file,
        Err(_) => return Err(format!("Couldn't create {}", path)),
    };

    match file.write(&bytes) {
        Ok(_) => Ok(file),
        Err(_) => Err(format!("Couldn't write to {}", path)),
    }
}

async fn fetch_packages_info(name: &str) -> Result<Vec<PackageInfo>, String> {
    if let Some(first_letter) = name.chars().nth(0) {
        let req_url = format!(
            "https://archive.archlinux.org/packages/{}/{name}/",
            first_letter,
            name = name);

        let resp = reqwest::get(&req_url)
        .await
        .expect("Failed to get packages.")
        .text()
        // NOTE(smolck): Shouldn't fail, unless something about the Arch Linux Archive changes.
        .await
        .unwrap();

        let dom = Html::parse_document(&resp);

        let pre_selector = Selector::parse("pre").unwrap();
        let a_selector = Selector::parse("a").unwrap();
        // Get first <pre> tag from package page.
        let pre = dom.select(&pre_selector).next().unwrap();

        let mut names = vec![];
        for name in pre.select(&a_selector) {
            names.push(PackageInfo {
                // TODO(smolck): Is `req_url.clone()` necessary here? Probably a better way of
                // doing this.
                url: req_url.clone() + &String::from(name.value().attr("href").unwrap()),
                name: String::from(name.inner_html()),
            });
        }

        // TODO(smolck): Possibly change this, just to remove the first element since it's `../`.
        // Also, should change the `x.clone()` part probably.
        Ok(names[1..names.len()]
            .iter()
            .map(|x| x.clone())
            .collect())
    } else {
        Err(String::from("Passed string without a first letter"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = Options::from_args().package_name;

    println!("Fetching packages.....");
    let packages = fetch_packages_info(&n).await?;

    let mut i: i32 = 0;
    for pkginfo in packages.iter() {
        if pkginfo.name.contains(".sig") { // Don't show packages ending in ".sig"
            continue;
        }

        println!("{}{indent}{}", i, pkginfo.name, indent = " ".repeat(5));
        i += 1;
    }
    println!();

    let index;
    loop {
        println!("Package to install: (e.g. 1)");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input.");

        match input.trim().parse::<i32>() {
            Ok(pkg_index) => {
                if pkg_index < packages.len() as i32 {
                    index = pkg_index;
                    break;
                } else {
                    println!("There isn't a package number {}!", input.trim());
                }
            }
            Err(_) => println!("Must input a number!"),
        }
    }

    let download_path = format!("/tmp/{}", packages[index as usize].name);

    println!("Downloading package.....");
    let _ = download_package(&packages[index as usize], &download_path).await?;

    Command::new("sudo")
        .arg("-S")
        .arg("pacman")
        .arg("-U")
        .arg(format!("{}", download_path))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()?;

    std::fs::remove_file(download_path)?;
    Ok(())
}
