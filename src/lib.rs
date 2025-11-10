mod data;
mod resource;

use anyhow::Result;
use rglua::prelude::*;

#[gmod_open]
fn gmod_open(lua: LuaState) -> Result<i32> {
	data::scan(lua)?;

	lua_pushcfunction(lua, Some(|lua| -> i32 {
		if let Err(err) = data::scan(lua) {
			lua_pushstring(lua, format!("mask_fs_scan failed: {}", err));
			lua_error(lua);
		}
		0
	}));
	lua_setglobal(lua, cstr!("mask_fs_scan"));

	Ok(0)
}

#[gmod_close]
fn gmod_close(_lua: LuaState) -> i32 {
	0
}
