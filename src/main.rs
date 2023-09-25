use std::process::exit;
use url::Url;
use regex::Regex;
use reqwest;
use clap::Parser;

#[derive(Parser)]
#[clap(name="rustnext.exe", author="joaojj)", version="1.0", about="Code that retrieves paths from the next.js framework", long_about = None)]
struct Args {
    #[clap(short, long, required = true)]
    url: String,

    #[clap(short, long, default_value = "https")]
    proto: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let args = Args::parse();
    let url = args.url;
    let proto = args.proto;

    match paths(&url).await {
        Ok(path) => {
            let format_url = Url::parse(&url);
            match format_url {
                Ok(url_format) => {
                    let host = url_format.host_str().unwrap();
                    let host = format!("{proto}://{host}");
                    let full_url = format!("{}{}", host, path);
                    let response = reqwest::get(full_url)
                        .await
                        .unwrap_or_else(|e| {
                            eprintln!("{e}");
                            exit(0); 
                        });
        
                    let body: String = response.text().await.expect("Failed to read response text");
                    let regex_path = Regex::new(r#"(/[^"]+)"|'(/[^']+)'"#).unwrap();
                    for cap in regex_path.captures_iter(&body) {
                        println!("{}", &cap[0].replace('"', ""));
                    }
                }, 
                Err(erro) => {
                    println!("{}", erro);
                    exit(0);
                }
            }
        },
        Err(error) => {
            println!("{error}");
        }
    }
    Ok(())
    
}

async fn paths(url: &String) -> Result<String, String> {
    let response2 = reqwest::get(url)
        .await.unwrap_or_else(|e| {
        eprintln!("{e}");
        exit(0);
    });
    let body = response2
        .text()
        .await
        .expect("Failed to read response text");

    let re = Regex::new(r"/_next/static/[^/]+/_buildManifest.js").unwrap();

    let result = re
        .find(&body)
        .map_or("0".to_string(), |matched| {
            matched.as_str().to_string()
        });
    
    if result == "0"{
        return Err(String::from("Couldn't find the corresponding file, is the site really next.js?"));
    }else {
        return Ok(result);
    }
}
