use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "docker registry client")]
enum Args {
    ListRepos {
        url: String
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();
    let args: Args = Args::from_args();
    println!("{:#?}", args)
}

