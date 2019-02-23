# hexchat-rs

A safe API for creating HexChat plugins. 

To get started, create a struct representing your plugin, and implement `Plugin` for it. Then,
call `plugin!` on the struct.

All plugins should be built as cdylibs, or if for some reason you have no other choice, dylibs.
Do not attempt to define a `main()` symbol; `Plugin::new` is your plugin's 'entry point'. For that
matter, do not attempt to define the HexChat C docs' described `extern fn`s - this is taken care
of for you by the `plugin!` macro.

If window manipulation is desired, then the `window` feature should be
enabled.

Static variables holding heap resources are discouraged and will cause memory leaks. This crate
provides a `safe_static!` macro for this purpose. Please note that any thread that accesses a safe
static must be killed in your plugin's `Drop` implementation, and it's undefined not to. You should
kill them anyway even if you don't use this, because they'll be a memory leak too otherwise.
