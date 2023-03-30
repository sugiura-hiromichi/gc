use mylibrary_::sh_cmd;
use std::env::args;

fn main() {
	let urls: Vec<String,> = args().collect();
	urls.iter().for_each(|url| {
		let mut repo = String::new();
		if url.find("/",).is_none() {
			repo = format!("sugiura-hiromichi/{url}");
		}
		repo = format!("git@github.com:{repo}");

		let o = sh_cmd!("git", ["clone", "--depth", "1", &repo]).unwrap().unwrap();
		if !o.status.success() {
			println!("execution of `git clone {repo}` failed");
			println!("stderr:\n\t{}", std::str::from_utf8(&o.stderr).unwrap());
			println!("stdout:\n\t{}", std::str::from_utf8(&o.stdout).unwrap());
		}
	},);
}
