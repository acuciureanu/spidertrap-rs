use clap::{command, Parser};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use rand::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_random_delay() -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(250..=1000)
}

fn create_card_content(
    rng: &mut StdRng,
    length_range: (i32, i32),
    address: String,
    counter: usize,
) -> String {
    let title = format!(
        "{} #{}",
        lipsum::lipsum_words(rng.gen_range(3..=5) as usize),
        counter,
    );
    let description = lipsum::lipsum(rng.gen_range(length_range.0..=length_range.1) as usize);
    format!(
        "<div class=\"card\" style=\"background-color: #f2f2f2; border: 1px solid #ddd; border-radius: 5px; padding: 10px;\"><a href=\"{}\"><h1>{}</h1></a><p>{}</p><a href=\"{}\"><button style=\"background-color: #4CAF50; border: none; color: white; padding: 10px 20px; text-align: center; text-decoration: none; display: inline-block; font-size: 16px; margin: 4px 2px; cursor: pointer;\">Details</button></a></div><br>\n",
        address, title, description, address
    )
}

async fn handle(
    req: Request<Body>,
    webpages: Arc<Vec<String>>,
    link_range: (i32, i32),
    length_range: (i32, i32),
) -> Result<Response<Body>, hyper::Error> {
    let delay = get_random_delay();
    sleep(Duration::from_millis(delay)).await;

    let path = req.uri().path().to_string();

    let mut s = DefaultHasher::new();
    path.hash(&mut s);
    let seed = s.finish();
    let mut rng = StdRng::seed_from_u64(seed);

    let num_pages = rng.gen_range(link_range.0..=link_range.1);

    let mut html = "<html>".to_string();

    for i in 0..num_pages {
        let address = if webpages.is_empty() {
            format!("/{}", rng.gen::<u32>())
        } else {
            webpages.choose(&mut rng).unwrap().clone()
        };
        html.push_str(&create_card_content(
            &mut rng,
            length_range,
            address,
            i as usize,
        ));
    }

    html.push_str("</div>\n</body>\n</html>");

    Ok(Response::new(Body::from(html)))
}

#[derive(Parser)]
#[command(name = "SpiderTrap")]
#[command(author = "Alex Cuciureanu")]
#[command(about = "Traps web spiders", long_about = None)]
struct Cli {
    #[arg(long, default_value = "8080")]
    port: u16,
    #[arg(long)]
    directories: Option<String>,
    #[arg(long, default_value = "5")]
    min_links: i32,
    #[arg(long, default_value = "10")]
    max_links: i32,
    #[arg(long, default_value = "3")]
    min_length: i32,
    #[arg(long, default_value = "20")]
    max_length: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();

    let link_range = (cli.min_links, cli.max_links);
    let length_range = (cli.min_length, cli.max_length);

    let directories = match &cli.directories {
        Some(path) => {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            
            let directories: Vec<String> = reader
                .lines()
                .filter_map(|res| match res {
                    Ok(line) => {
                        if line.trim_start().starts_with('#') {
                            None
                        } else {
                            Some(line)
                        }
                    }
                    Err(_) => None,
                })
                .collect();
            
            Arc::new(directories)
        }
        None => {
            eprintln!("No directories file provided. Using default configuration.");
            Arc::new(Vec::new())
        }
    };    

    let addr = SocketAddr::from(([0, 0, 0, 0], cli.port));

    let make_svc = make_service_fn(move |_conn| {
        let directories = Arc::clone(&directories);
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                handle(req, Arc::clone(&directories), link_range, length_range)
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Starting server on port {}...", cli.port);
    server.await?;

    Ok(())
}
