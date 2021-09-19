use git2::{Repository, StatusOptions};
use mlua::prelude::*;

fn get_branch() -> String {
    let directory = ".";
    return match Repository::discover(directory) {
        Ok(repository) => {
            return match repository.head() {
                Ok(head) => {
                    let head_name = head.shorthand().unwrap();

                    let branch = repository
                        .find_branch(head_name, git2::BranchType::Local)
                        .unwrap()
                        .name()
                        .unwrap()
                        .unwrap()
                        .to_string();

                    return branch;
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

#[mlua::lua_module]
fn git_utils(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("branch", lua.create_function(get_branch_name)?)?;

    Ok(exports)
}
