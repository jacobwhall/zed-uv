# uv slash commands for Zed

This is an extension for [Zed](https://zed.dev) that adds "slash commands" to the AI assistant panel that allow you to control [uv](https://docs.astral.sh/uv/).

> [!WARNING]
> I made this because I thought it would be amusing...
> I am quite sure you'd be better off using Zed's built-in terminal to manage Python environments with uv.

Zed and uv are both written in Rust, meant to help manage coding projects...wouldn't they go well together?
I hope Zed will consider allowing extensions to add commands to the command palette, as that would make more sense for a tool like this and further decrease the time I spend dealing with Python dependency hell.

## Usage

There are two crates in this repository: the extension and the server.
Yep, there is a server.

First you will need to [install rust](https://www.rust-lang.org/tools/install).

Follow [these instructions](https://zed.dev/docs/extensions/developing-extensions#developing-an-extension-locally) to install the extension, and just run `cargo run` to run the server.
While the server is running, type `/uv` in the AI assistant panel to see the list of possible arguments.

## License

This project is licensed under GPLv3 or any later version.
For more information, please see [LICENSE.md](LICENSE.md).
