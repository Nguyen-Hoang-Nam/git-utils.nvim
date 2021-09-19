use git2::Repository;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!("", get_branch());
    }
}

fn get_branch_name(lua: &Lua, _: String) -> LuaResult<LuaString> {
    let branch = get_branch();
    let branch_name = lua.create_string(&branch)?;

    Ok(branch_name)
}

#[mlua::lua_module]
fn git_utils(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("branch", lua.create_function(get_branch_name)?)?;

    Ok(exports)
}
