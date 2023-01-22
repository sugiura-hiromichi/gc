use mylibrary::cli::CliParser;
use mylibrary::sh_cmd;
use std::env::args;

fn main() {
	let mut url = args().to_string();
	if url.find("/",).is_none() {
		url = format!("sugiura-hiromichi/{url}");
	}
	url = format!("git@github.com:{url}");

	sh_cmd!("git", ["clone".to_string(), url.clone()]);
}
