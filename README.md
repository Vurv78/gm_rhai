## gm_rhai
This is so far just a binary module that adds [rhai](https://github.com/rhaiscript/rhai) scripting to garrysmod using (rglua)[https://github.com/Vurv78/rglua].

It adds a table that only contains one function ``run``.
It only has ``print`` exposed to print to the gmod console though, since this is just a proof of concept until I expose entity methods or something.

### Lua Usage

```lua
require "rhai"
rhai.run([[
print("Hello world!")
]])
```

### Why
1. I wanted to do something with ``rglua``.
2. It is incredibly easy to sandbox and expose functions to.
3. It's syntax is really close to Rust's, and while [mun](https://github.com/mun-lang/mun) might be closer, #2 made me pick this for now.

Because of these reasons, I made this and I want to experiment with it so here's this repo.