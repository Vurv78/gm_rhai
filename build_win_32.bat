@echo off
rustup target add i686-pc-windows-msvc
cargo build --release --target=i686-pc-windows-msvc
move %cd%\target\i686-pc-windows-msvc\release\gmrhai.dll %cd%\gmsv_rhai_win32.dll
pause