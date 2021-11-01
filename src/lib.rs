use git2::{Repository, StatusOptions};
use mlua::prelude::*;
use std::path::Path;

// Redit https://github.com/xcambar/purs/blob/master/src/precmd.rs
fn get_head_shortname(repository: &Repository) -> Option<String> {
    let head = repository.head().ok()?;
    if let Some(shorthand) = head.shorthand() {
        if shorthand != "HEAD" {
            return Some(shorthand.to_string());
        }
    }

    let object = head.peel(git2::ObjectType::Commit).ok()?;
    let short_id = object.short_id().ok()?;

    Some(format!(
        ":{}",
        short_id.iter().map(|ch| *ch as char).collect::<String>()
    ))
}

pub fn get_branch() -> String {
    let directory = ".";
    return match Repository::discover(directory) {
        Ok(repository) => {
            return match repository.head() {
                Ok(_) => {
                    return get_head_shortname(&repository).unwrap();
                }
                Err(_) => "".to_string(),
            };
        }
        Err(_) => "".to_string(),
    };
}

// Redit https://github.com/xcambar/purs/blob/master/src/precmd.rs
fn count_change() -> String {
    let directory = ".";
    return match Repository::discover(directory) {
        Ok(repository) => {
            let mut opts = StatusOptions::new();
            opts.include_untracked(true);

            fn count_files(statuses: &git2::Statuses<'_>, status: git2::Status) -> usize {
                statuses
                    .iter()
                    .filter(|entry| entry.status().intersects(status))
                    .count()
            }

            let statuses = repository.statuses(Some(&mut opts)).unwrap();

            if let Some((index_change, wt_change, conflicted, untracked)) = Some((
                count_files(
                    &statuses,
                    git2::Status::INDEX_NEW
                        | git2::Status::INDEX_MODIFIED
                        | git2::Status::INDEX_DELETED
                        | git2::Status::INDEX_RENAMED
                        | git2::Status::INDEX_TYPECHANGE,
                ),
                count_files(
                    &statuses,
                    git2::Status::WT_MODIFIED
                        | git2::Status::WT_DELETED
                        | git2::Status::WT_TYPECHANGE
                        | git2::Status::WT_RENAMED,
                ),
                count_files(&statuses, git2::Status::CONFLICTED),
                count_files(&statuses, git2::Status::WT_NEW),
            )) {
                if index_change != 0 || wt_change != 0 || conflicted != 0 || untracked != 0 {
                    return "*".to_string();
                }

                return "".to_string();
            }

            return "".to_string();
        }
        Err(_) => "".to_string(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_branch() {
        assert_eq!("main", get_branch());
    }

    #[test]
    fn test_count_change() {
        assert_eq!("*", count_change());
    }
}

fn get_branch_name(lua: &Lua, _: String) -> LuaResult<LuaString> {
    let mut branch = get_branch();

    if branch != "" {
        let status = count_change();
        branch = format!("{}{}", branch, status);
    }

    let branch_name = lua.create_string(&branch)?;

    Ok(branch_name)
}

fn get_blame_file(lua: &Lua, (path, line): (String, f64)) -> LuaResult<LuaTable> {
    let blame = lua.create_table()?;

    let path_str = Path::new(&path);
    let directory = ".";

    match Repository::discover(directory) {
        Ok(repository) => {
            let repository_path = repository.path().parent().unwrap();
            let relative_path = path_str.strip_prefix(repository_path).unwrap();

            let blame_file = repository.blame_file(relative_path, None).unwrap();

            match blame_file.get_line(line as usize) {
                Some(blame_line) => {
                    let blame_line_commit_id = blame_line.final_commit_id();

                    let commit = repository.find_commit(blame_line_commit_id).unwrap();

                    blame.set("author", commit.author().name().unwrap_or(""))?;
                    blame.set("message", commit.message().unwrap_or(""))?;
                }
                None => (),
            };
        }
        Err(_) => (),
    };

    Ok(blame)
}

#[mlua::lua_module]
fn git_utils(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("branch", lua.create_function(get_branch_name)?)?;
    exports.set("blame", lua.create_function(get_blame_file)?)?;

    Ok(exports)
}
