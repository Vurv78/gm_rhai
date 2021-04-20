use rglua::{
    types::{
        LuaState
    },
    cstring,
    rstring,
    LUA_SHARED
};

use rhai::{Engine};

pub extern fn run_code(state: LuaState) -> i32 {
    let shared = *LUA_SHARED;

    let code = rstring!(shared.lua_tostring(state, 1));

    let mut engine = Engine::new();

    // Expose a 'print' function that just prints to gmod console.
    engine.register_fn("print", move |x: String| {
        rglua::printgm!(state, "{}", x);
    });

    engine.set_max_string_size(500);

    let engine = engine; // Make engine variable immutable.

    let result = match engine.eval::<i64>(code) {
        Ok(res) => res,
        Err(why) => {
            rglua::printgm!(state, "Code errored. {}", why);
            return 0; // Return prematurely
        }
    };

    rglua::printgm!(state, "Got an i64! {}", result );
    0
}

#[no_mangle]
pub extern fn gmod13_open(state: LuaState) -> i32 {
    let could_mount_shared = match &*rglua::LUA_SHAREDR {
        Ok(_) => true,
        Err(why) => {
            eprintln!("Couldn't mount lua_shared.dll! {}", why);
            false
        }
    };

    if could_mount_shared {
        let shared = *LUA_SHARED;

        shared.lua_getglobal(state, cstring!("print"));
        shared.lua_pushstring(state, cstring!("Hello from rust!"));
        shared.lua_call(state, 1, 0);
    
        // local t = {}
        shared.lua_createtable(state, 0, 0);
    
        // t.run = function()
        shared.lua_pushcfunction(state, run_code);
        shared.lua_setfield(state, -2, cstring!("run"));
    
        // _G.rhai = t
        shared.lua_setglobal(state, cstring!("rhai"));
    }
    0
}

#[no_mangle]
pub extern fn gmod13_close(_state: LuaState) -> i32 {
    0
}