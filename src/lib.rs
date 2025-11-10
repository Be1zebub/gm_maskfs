mod data;
mod resource;

use anyhow::Result;
use rglua::prelude::*;
use std::ffi::CString;

#[gmod_open]
fn gmod_open(lua: LuaState) -> Result<i32> {
	data::scan(lua)?;

	extern "C" fn mask_fs_scan(lua: LuaState) -> i32 {
		if let Err(err) = data::scan(lua) {
			unsafe {
				let msg = CString::new(format!("mask_fs_scan failed: {}", err))
					.unwrap_or_else(|_| CString::new("mask_fs_scan failed").unwrap());
				lua_pushstring(lua, msg.as_ptr());
				lua_error(lua);
			}
		}
		0
	}

	lua_pushcfunction(lua, mask_fs_scan);
	lua_setglobal(lua, cstr!("mask_fs_scan"));

	Ok(0)
}

#[gmod_close]
fn gmod_close(_lua: LuaState) -> i32 {
	0
}
