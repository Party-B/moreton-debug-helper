# The Moreton Debugger-Helper

Like the humble Moreton Bay Bug.

I use GDB, I use Vim. I learnt how to program using the VBA IDE which - as an interpretted language has what I still think to this day is an incredible debugger.
Using Termdebug I can never call rust functions on the fly to check what return values are like. So I put this together, a partial list of those functions renamed so that the debugger has access to it.

## Usage: (SEE NOTE)
1. Drop this file into your project's src/ directory
2. Add to your main.rs or lib.rs:
  mod moreton;


## NOTE:

The rust compiler is just too smart. Smarter than me.
I thought I could just chuck in a bunch of code that's not used and call it willy-nilly. But nooooo. I have to use the damn code.
So your options for using moreton are
1. build your project with the flag to compile dead code into your bin:  RUSTFLAGS="-C link-dead-code" cargo build
2. Edit moreton.rs to include a force_link_all function which does a let call on each function - this seems to convince the compiler that you should be able to access it.

You still need to call the crate namespace while in gdb - ie. print binary_file::moreton::d_trim("Trim the text  ")
If I can get around this in the future, I'll change it. I'm planning on doing some more testing to figure this out.

All functions are:
 - Only compiled in debug builds (#[cfg(debug_assertions)])
 - Never inlined so GDB can find them (#[inline(never)])
 - Have stable names for easy GDB access
