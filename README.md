# The Moreton Debugger-Helper

Like the humble Moreton Bay Bug.

I use GDB, I use Vim. I learnt how to program using the VBA IDE which - as an interpretted language has what I still think to this day is an incredible debugger.
Using Termdebug I can never call rust functions on the fly to check what return values are like. So I put this together, a partial list of those functions renamed so that the debugger has access to it.

## Usage: (SEE NOTE)
1. Drop this file into your project's src/ directory
2. Add to your main.rs or lib.rs: mod moreton;
3. In the main.rs, just after fn main, insert a call to the force all: moreton::_force_link_all();
4. From the debugging (eg gdb), call the functions prepended by "d_" (eg binary_file::moreton::d_trim(var))

## NOTE:

The rust compiler is just too smart. Smarter than me.
I thought I could just chuck in a bunch of code that's not used and call it willy-nilly. But nooooo. I have to use the damn code.
Originally, I considered using a build flag to link dead code, but this brought in all dead code, not just the moreton helpers: RUSTFLAGS="-C link-dead-code" cargo build
  
You still need to call the crate namespace while in gdb - ie. print binary_file::moreton::d_trim("Trim the text  ")
If I can get around this in the future, I'll change it. I'm planning on doing some more testing to figure this out.

