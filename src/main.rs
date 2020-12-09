use structopt::StructOpt;
use dr::DockerRegistry;

#[derive(StructOpt, Debug)]
#[structopt(about = "docker registry client")]
enum Args {
    ListRepos {
        url: String
    }
}

#[tokio::main]
async fn main() -> dr::Result<()> {
    pretty_env_logger::init_timed();
    let args: Args = Args::from_args();
    match args {
        Args::ListRepos { url } => list_repos(url).await?
    };
    Ok(())
}

async fn list_repos(url: String) -> dr::Result<()> {
    let registry = DockerRegistry::new(&url)?;
    for repo in registry.list_repos().await?.into_iter() {
        println!("{}", repo)
    }
    Ok(())
}

