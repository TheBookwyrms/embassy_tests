# use PowerShell instead of sh on windows:
set windows-shell  := ["powershell.exe", "-c"]

embassy := "usb_raw_bulk"

ship file=embassy:
   just build {{file}}
   just to_uf2 {{file}}
# send to pico, with debug probe

build file:
   cargo build --bin {{file}}

to_uf2 file=embassy:
   elf2uf2-rs target/thumbv6m-none-eabi/debug/{{file}} target/thumbv6m-none-eabi/debug/{{file}}.uf2