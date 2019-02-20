# hexchat-rs

A safe API for creating HexChat plugins. 

To get started, create a struct representing your plugin, and implement `Plugin` for it. Then,
call `plugin!` on the struct.

All plugins should be built as cdylibs, or if for some reason you have no other choice, dylibs.
Do not attempt to define a `main()` symbol; `Plugin::new` is your plugin's 'entry point'. For that
matter, do not attempt to define the HexChat C docs' described `extern fn`s - this is taken care
of for you by the `plugin!` macro.