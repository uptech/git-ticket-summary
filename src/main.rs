use structopt::StructOpt;
mod utils;

#[derive(Debug, StructOpt)]
#[structopt(name = "git-ticket-summary")]
pub struct ApplicationArguments {
  pub ticket_id: String,
  #[structopt(default_value = "main", name = "branch")]
  pub branch: String
}

fn main() {
  let opt = ApplicationArguments::from_args();
  generate_summary(&opt);
}

pub fn generate_summary(args: &ApplicationArguments) {
  let repo = git2::Repository::discover("./").unwrap();

  utils::execute("git", &["fetch"]).unwrap();

  let config = repo.config().unwrap();
  let tracking_branch = config.get_string(format!("branch.{}.merge", args.branch).as_str()).unwrap();
  let tracking_remote = config.get_string(format!("branch.{}.remote", args.branch).as_str()).unwrap();
  let tracking_remote_url = config.get_string(format!("remote.{}.url", tracking_remote).as_str()).unwrap();

  let fixed_remote_url = tracking_remote_url.replace("git@github.com:", "https://github.com/");
  let base_url = &fixed_remote_url[..(fixed_remote_url.len() - 4)];

  let commit_base_url = format!("{}/commit/", base_url);

  let short_tracking_branch = tracking_branch.replace("refs/heads/", "");
  let remote_branch = format!("{}/{}", tracking_remote, short_tracking_branch);
  let remote_branch_ref_name = format!("refs/remotes/{}", remote_branch);

  let ref_origin_main = repo.find_reference(&remote_branch_ref_name).unwrap();
  let ref_origin_main_oid = ref_origin_main.target().unwrap();

  let mut rev_walk = repo.revwalk().unwrap();
  rev_walk.push(ref_origin_main_oid).unwrap();

  let re = regex::Regex::new(&args.ticket_id).unwrap();

  println!("\n## Ticket Summary\n");
  println!("ticket: {}", args.ticket_id);
  println!("repository: {}", tracking_remote_url);
  println!("\n### Commits\n");

  rev_walk
    .map(|opt_oid| repo.find_commit(*opt_oid.as_ref().unwrap()).unwrap())
    .filter(commit_summary_or_message_matches_regex(re))
    .for_each(|commit| {
      let oid = commit.id();
      let summary = commit.summary().unwrap();
      println!("[{:.6}]({}{}) {}", oid, commit_base_url, oid, summary);
    });

  println!();
}

pub fn commit_summary_or_message_matches_regex(re: regex::Regex) -> impl Fn(&git2::Commit) -> bool {
  move |commit: &git2::Commit| {
    let summary = commit.summary().unwrap();
    let message = commit.message().unwrap();
    re.captures(summary).is_some() || re.captures(message).is_some()
  }
}
