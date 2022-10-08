use mylibrary::sh_cmd;
use std::env::args;

fn main() {
   let mut arg = args();
   arg.next();
   let mut url: String = arg.collect();
   if url.find("/",).is_none() {
      url = format!("sugiura-hiromichi/{url}");
   }
   url = format!("https://github.com/{url}");

   sh_cmd!("git", ["clone".to_string(), url]);
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn arg_tes() {
      for arg in args() {
         println!("{arg}");
      }
   }
}
