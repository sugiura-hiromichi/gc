use std::env::args;
use sugiura_hiromichi_mylibrary::cli::CliParser as _;
use sugiura_hiromichi_mylibrary::sh_cmd;

fn main() -> anyhow::Result<(),> {
	let mut url = args().to_string();
	if url.find("/",).is_none() {
		url = format!("sugiura-hiromichi/{url}");
	}
	url = format!("git@github.com:{url}");

	let o = sh_cmd!("git", ["clone", "--depth", "1", &url])?.unwrap();
	println!("exit_status |>{}", o.status);
	Ok((),)
}
