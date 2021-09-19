# Git-utils.nvim

Collection of utils for git in NeoVim were written in Rust.

![Git hover](https://raw.githubusercontent.com/Nguyen-Hoang-Nam/readme-image/main/git-utils.nvim/git-utils-githover.jpg)

## Installation

You must have Rust to build this lib to shared library.

For Linux

```bash
git clone https://github.com/Nguyen-Hoang-Nam/git-utils.nvim.git
cd git-utils.nvim
mkdir -p lua
cargo build --release
cp target/release/libembed.so lua/git_utils.so
```

For MacOS, you need some configuration from this
[article](https://blog.kdheepak.com/loading-a-rust-library-as-a-lua-module-in-neovim.html)

## Usage

### Get current git's branch

The empty string in branch function is require by mlua 😥.
If you know how to create function without parameter,
please make a pull request thanks.

```lua
require('git_utils').branch('')
```

#### Before

![Before](https://raw.githubusercontent.com/Nguyen-Hoang-Nam/readme-image/main/git-utils.nvim/git-utils-branch-after.jpg)

#### After

![After](https://raw.githubusercontent.com/Nguyen-Hoang-Nam/readme-image/main/git-utils.nvim/git-utils-branch-before.jpg)

### Get line blame

Gitsigns.nvim've already had line blame, but they
only show summary of commit. This function will
show all information.

```lua
require('git_utils').blame(vim.fn.expand('%:p'), vim.api.nvim_win_get_cursor(0)[1])
```

Currently, this function only returns 2 properties, author and message.

#### Print

![Git hover](https://raw.githubusercontent.com/Nguyen-Hoang-Nam/readme-image/main/git-utils.nvim/git-utils-githover.jpg)

## Contributing

Pull requests are welcome. For major changes,
please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
