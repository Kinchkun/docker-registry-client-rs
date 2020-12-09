use structopt::StructOpt;
use dr::DockerRegistry;
use log::info;

#[derive(StructOpt, Debug)]
#[structopt(about = "docker registry client")]
enum Args {
    ListRepos {
        url: String
    },
    ListTags {
        url: String,
        repo: String
    },
    DeleteTags {
        url: String,
        repo: String,
        tags: Vec<String>
    }
}

#[tokio::main]
async fn main() -> dr::Result<()> {
    pretty_env_logger::init_timed();
    let args: Args = Args::from_args();
    match args {
        Args::ListRepos { url } => list_repos(url).await?,
        Args::ListTags { url, repo} => list_tags(url, repo).await?,
        Args::DeleteTags { url, repo, tags } => delete_tags(url, repo, tags).await?
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


async fn list_tags(url: String, repo: String) -> dr::Result<()> {
    let registry = DockerRegistry::new(&url)?;
    for tag in registry.list_tag_per_repo(repo.as_str()).await?.into_iter() {
        println!("{}", tag)
    }
    Ok(())
}

async fn delete_tags(url: String, repo: String, tags: Vec<String>) -> dr::Result<()> {
    let registry = DockerRegistry::new(&url)?;
    for tag in tags.into_iter() {
        info!("Deleting: {}", tag);
        registry.delete_tag(repo.as_str(), tag.as_str()).await?;
    }
    Ok(())
}