# The Moreton Debugger-Helper

Like the humble Moreton Bay Bug.

I use GDB, I use Vim. I learnt how to program using the VBA IDE which - as an interpretted language has what I still think to this day is an incredible debugger.
Using Termdebug I can never call rust functions on the fly to check what return values are like. So I put this together, a partial list of those functions renamed so that the debugger has access to it.

##Usage:
1. Drop this file into your project's src/ directory
2. Add to your main.rs or lib.rs: mod moreton;
3. Compile with debug symbols: cargo build
4. In GDB at breakpoint: call debug_helpers::d_trim(my_string)

All functions are:
 - Only compiled in debug builds (#[cfg(debug_assertions)])
 - Never inlined so GDB can find them (#[inline(never)])
 - Have stable names for easy GDB access
