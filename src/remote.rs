use std::ffi::OsStr;
use std::fmt::Display;
use std::path::Path;
use errors::Result;
use query::Query;
use vcs;

/// Perform to clone repository into local path.
pub fn do_clone<P, S>(query: Query, root: P, args: &[S], dry_run: bool) -> Result<()>
  where P: AsRef<Path>,
        S: AsRef<OsStr> + Display
{
  let path = query.to_local_path()?;
  let path = root.as_ref().join(path);
  let url = query.to_url()?;
  if vcs::detect_from_path(&path).is_some() {
    println!("The repository has already cloned.");
    return Ok(());
  }
  if dry_run {
    println!("[debug] git clone '{}' '{}' {}",
             url.as_str(),
             path.display(),
             args.iter().fold(String::new(), |s, a| format!("{} {}", s, a)));
    Ok(())
  } else {
    vcs::git::clone(&url, &path, args)
  }
}
