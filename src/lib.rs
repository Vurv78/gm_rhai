#[macro_use] extern crate rglua;

use rglua::{
    types::{
        LuaState
    },
    cstring,
    rstring,
    lua_shared::*
};

use rhai::{Engine};

pub extern fn run_code(state: LuaState) -> i32 {
    let code = rstring!( lua_tostring!(state, 1) );

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
    lua_getglobal!(state, cstring!("print"));
    lua_pushstring(state, cstring!("Hello from rust!"));
    lua_call(state, 1, 0);

    // local t = {}
    lua_createtable(state, 0, 0);

    // t.run = function()
    lua_pushcfunction!(state, run_code);
    lua_setfield(state, -2, cstring!("run"));

    // _G.rhai = t
    lua_setglobal!(state, cstring!("rhai"));
    0
}

#[no_mangle]
pub extern fn gmod13_close(_state: LuaState) -> i32 {
    0
}