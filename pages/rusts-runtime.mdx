# Rust's Runtime

Rust has a runtime. No really it does! I blew past that aspect in my most recent
post ["Oxidizing the technical
interview"](/posts/oxidizing-the-technical-interview/) when
I put in the line:

> The greatest trick the Devil ever played was convincing C and Rust programmers
> their language has no runtime.

and you can see there's a bit more to the machinery in running a Rust program
with this bit of code used in that post:

```rust
#[lang = "start"]
fn start<T: Termination + 'static>(main: fn() -> T, _: isize, _: *const *const u8) -> isize {
    main().report() as isize
}

fn main() {
    unsafe {
        printf(
            "The value is: %d\n\0" as *const str as *const u8 as *const c_char,
            DUPLICATE,
        );
    }
}
```

There's this start function and it's a lang item as well. I think Manishearth
did a fantastic job of answering ["What is a lang
item?"](https://manishearth.github.io/blog/2017/01/11/rust-tidbits-what-is-a-lang-item/)
though the signature for start has changed since then a little bit. If you don't
know what a lang item is, go read it and come back here before we head further
down this road. The important bit to take away here is start is the entry point
to all Rust programs, not main. We'll see how the runtime defines this start
function and what it does before it calls main soon enough. You should note that
the runtime is specifically for libstd based programs since libstd is designed
to be aware of what platform it's running on versus libcore which is platform
agnostic and does not have things that allocate inside of it either unless you
set a global allocator and pull in the alloc crate. With that being said let's
get started.

## Understanding the runtime

Let's look at what the runtime code looks like and break down what it's doing
bit by bit. You can find a link to the full source code on GitHub here if you
want to peruse it, but I'll be putting all the code from there as small snippets
here as we walk through it. I'm using permalinks here and throughout the rest of
this article so that things will always point to the correct code that exists at
this point in time, but things get outdated and the implementation might change!
I do find it amusing that one of the first bits in the module is these few
lines:

```rust
#![unstable(
    feature = "rt",
    reason = "this public module should not exist and is highly likely \
              to disappear",
    issue = "none"
)]
#![doc(hidden)]
```

If you look at the git history for the reason line it has two commits, one for
formatting, and the other 5 years ago adding this line:

```bash
rust/src/libstd on  master is 📦 v0.0.0 via 🦀 v1.43.0 took 2s
❯ git log -u -L 11,12:rt.rs
commit 4436c9d35498e7ae3da261f6141d6d73b915e1e8
Author: David Tolnay <dtolnay@gmail.com>
Date:   Wed Nov 27 10:29:00 2019 -0800

    Format libstd with rustfmt

    This commit applies rustfmt with rust-lang/rust's default settings to
    files in src/libstd *that are not involved in any currently open PR* to
    minimize merge conflicts. THe list of files involved in open PRs was
    determined by querying GitHub's GraphQL API with this script:
    https://gist.github.com/dtolnay/aa9c34993dc051a4f344d1b10e4487e8

    With the list of files from the script in outstanding_files, the
    relevant commands were:

        $ find src/libstd -name '*.rs' \
            | xargs rustfmt --edition=2018 --unstable-features --skip-children
        $ rg libstd outstanding_files | xargs git checkout --

    Repeating this process several months apart should get us coverage of
    most of the rest of libstd.

    To confirm no funny business:

        $ git checkout $THIS_COMMIT^
        $ git show --pretty= --name-only $THIS_COMMIT \
            | xargs rustfmt --edition=2018 --unstable-features --skip-children
        $ git diff $THIS_COMMIT  # there should be no difference

diff --git a/src/libstd/rt.rs b/src/libstd/rt.rs
--- a/src/libstd/rt.rs
+++ b/src/libstd/rt.rs
@@ -9,4 +11,2 @@
-#![unstable(feature = "rt",
-            reason = "this public module should not exist and is highly likely \
-                      to disappear",
-            issue = "0")]
+    reason = "this public module should not exist and is highly likely \
+              to disappear",

commit f4be2026dfb507e5db919cc5df8fd934e05fa0b8
Author: Alex Crichton <alex@alexcrichton.com>
Date:   Tue Sep 8 15:53:46 2015 -0700

    std: Internalize almost all of `std::rt`

    This commit does some refactoring to make almost all of the `std::rt` private.
    Specifically, the following items are no longer part of its API:

    * DEFAULT_ERROR_CODE
    * backtrace
    * unwind
    * args
    * at_exit
    * cleanup
    * heap (this is just alloc::heap)
    * min_stack
    * util

    The module is now tagged as `#[doc(hidden)]` as the only purpose it's serve is
    an entry point for the `panic!` macro via the `begin_unwind` and
    `begin_unwind_fmt` reexports.
```

Well it's been 5 years and it still hasn't been removed and does a few more
things beyond being an entry point for the panic! macro. Let's take a look!
The first bit we see are the only imports directly reexported for other crates:

```rust
// Re-export some of our utilities which are expected by other crates.
pub use crate::panicking::{begin_panic, begin_panic_fmt, update_panic_count};
```

We can trace what all these do if we visit the panicking module of libstd and
all of the intricate details, but the big take away is whenever you see this:

```bash
~/panic-at-the-function-call is 📦 v0.1.0 via 🦀 v1.43.0
❯ cargo run
   Compiling panic-at-the-function-call v0.1.0 (/Users/michael/panic-at-the-function-call)
    Finished dev [unoptimized + debuginfo] target(s) in 0.75s
     Running `target/debug/panic-at-the-function-call`
thread 'main' panicked at 'This is my roaring, roaring panic's!', src/main.rs:2:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

the `begin_panic` function handles how to deal with well panicking. This calls
the machinery to print out that error message we see above or in the case of you
setting [your own panic
hook](https://doc.rust-lang.org/std/panic/fn.set_hook.html) it will dispatch
that hook instead of the default built in one. If we didn't have the machinery
set up to catch and print out panics in the runtime we'd have to do it
ourselves. Pretty annoying. This is why even with `panic = "abort"` set in your
`Cargo.toml` file you'll still see a decent sized chunk of code in the assembly
output. It still has code related to aborting, which is less than the normal
panic machinery which does stack unwinding and lets you catch the panic if it
happens, but it still has code related to aborting in it. The only way to get
rid of it all is to write your own `start` function and not setting up any of
this panic/abort machinery like I had done in my "Oxidizing the technical
interview" article. This really isn't a good tradeoff unless binary size really
matters, say in like an embeded environment. Being able to abort properly or
catching panics and unwinding the stack is good! Panics are great when the state
of the program or the assumptions you made no longer are in a recoverable state.
Having the scaffolding in place to catch these automatically for us or at least
aborting the program for us is great and we don't even have to set it up by hand
every time. Okay so we know we export some functions for panicking but what
about the actual setup code for the runtime?

```rust
// To reduce the generated code of the new `lang_start`, this function is doing
// the real work.
#[cfg(not(test))]
fn lang_start_internal(
    main: &(dyn Fn() -> i32 + Sync + crate::panic::RefUnwindSafe),
    argc: isize,
    argv: *const *const u8,
) -> isize {
```

It looks like at a certain point the code in the module got split into two
functions for codegen purposes. If we look the only other function defined in
the file is at the bottom and is the actual `start` function I showed you
earlier:

```rust
#[cfg(not(test))]
#[lang = "start"]
fn lang_start<T: crate::process::Termination + 'static>(
    main: fn() -> T,
    argc: isize,
    argv: *const *const u8,
) -> isize {
    lang_start_internal(&move || main().report(), argc, argv)
}
```

This is where the program actually starts and it just calls the internal
function and passes the values to it. So let's look at the arguments we pass to
`lang_start_internal`. The first argument is a closure invoking the function
pointer to our `main` function! When this closure is called our program actually
runs, but as we'll see it won't run until we run a few other things in
preparation for it. We can also see it calls `report` inside of this closure.
This is from the `Termination` trait which was added as part of RFC 1937. As you
can see here at least as of Rust 1.43.1, the trait is not stabilized. It's only
usable on stable for the main function and is what allows us to use `?` in
`main`. `main` only implements `Termination` for `!` (the never type as in this
will never return), `()` your default `fn main()` return type, `Result<(), E>`
or `Result<!, E>` where `E: std::error::Error`, or `ExitCode` which is also
unstable right now. The thing to note is that report returns an `i32` which is
passed to the operating system to say what exit code it ended with. Though these
exit codes tend to be fairly platform specific in terms of what they mean, most
if not all define return values of zero to indicate a success and non zero
  values as failures. We can see that `lang_start` returns an `isize` and this
  is what the return value of `report` gets turned into and is used for in the
  compiler's codegen to set the exit code in the program for each platform.

Next up we see `argc` and `argv` which is how most programming languages let you
pass arguments to your program. The platform, when running the program, will
pass in how many args were given to it as a number (`argc`), and a pointer to an
array of string pointers (`argv`). Technically it's a pointer to an arbitrary
array of bytes, but they're meant to be interpreted as strings.  We don't see
these two arguments in Rust's `main` function as we get access to these via
`std::env::args`. The reason for this is again platform differences with how
they're handled. That and dereferencing raw pointers is an action that requires
unsafe. `std::env::args` handles all of these for us in a safe way and wraps it
up in a really nice interface we can iterate over and turns all the raw pointers
into Rust's `String` type! Could you imagine having to do this yourself every
time you wrote a program and needed access to the argument values? No thanks,
I'd rather not mess with raw pointers or make the code work on every platform
myself. This is a huge benefit of Rust's runtime and standard library. You have
a safe well thought out interface to catch all those gotchas in libstd and you
have code that makes this possible by dealing with these values for you. Let's
take a look then at the code that gets called immediately after
`lang_start_internal` gets invoked.

```rust
    use crate::panic;
    use crate::sys;
    use crate::sys_common;
    use crate::sys_common::thread_info;
    use crate::thread::Thread;

    sys::init();
```

We import some items from libstd into this function then call `sys::init()`.
What does this function do? Depends on the platform! If you [look
here](https://github.com/rust-lang/rust/blob/de6060b09d23d8c50afe88354358aa056c27ad9f/src/libstd/sys/mod.rs)
you can see that the code that gets reexported here varies. On a Unix system we
see [this code for
`init`](https://github.com/rust-lang/rust/blob/de6060b09d23d8c50afe88354358aa056c27ad9f/src/libstd/sys/unix/mod.rs#L77).

```rust
#[cfg(not(test))]
pub fn init() {
    // By default, some platforms will send a *signal* when an EPIPE error
    // would otherwise be delivered. This runtime doesn't install a SIGPIPE
    // handler, causing it to kill the program, which isn't exactly what we
    // want!
    //
    // Hence, we set SIGPIPE to ignore when the program starts up in order
    // to prevent this problem.
    unsafe {
        reset_sigpipe();
    }

    #[cfg(not(any(target_os = "emscripten", target_os = "fuchsia")))]
    unsafe fn reset_sigpipe() {
        assert!(signal(libc::SIGPIPE, libc::SIG_IGN) != libc::SIG_ERR);
    }
    #[cfg(any(target_os = "emscripten", target_os = "fuchsia"))]
    unsafe fn reset_sigpipe() {}
}
```

It's real neat that Rust just handles this for us. If this happened to me I
probably would not even know how to begin to debug this. To quote James
Mickens:

> When you debug systems code, there are no high-level debates about font
> choices and the best kind of turquoise, because this is the Old Testament, an
> angry and monochro-matic world, and it doesn’t matter whether your Arial is
> Bold or Condensed when people are covered in boils and pestilence and Egyptian
> pharaoh oppression. - [The Night Watch](https://www.usenix.org/system/files/1311_05-08_mickens.pdf)

On Windows thought [init is just an empty
function](https://github.com/rust-lang/rust/blob/de6060b09d23d8c50afe88354358aa056c27ad9f/src/libstd/sys/windows/mod.rs#L53)!
Neat huh? Not having to worry about these little gotchas and having to write a
ton of platform dependent code ourselves is absolutely a killer feature. Having
the program start up and being able to say "It Just Works ™" on all the
platforms Rust supports without ignoring the fact that different platforms exist
[like Go does all the
time](https://fasterthanli.me/blog/2020/i-want-off-mr-golangs-wild-ride/) is
awesome. It just goes to show how much work goes into libstd in terms of making
it a small but powerful library. It attempts to make things simple and where it
can't it'll warn you as to why and use types to force you to deal with these
problems, like `OsString` for paths. I'm digressing a bit though. What happens
after the runtime initializes the system?

```rust
    unsafe {
        let main_guard = sys::thread::guard::init();
        sys::stack_overflow::init();
```

First it tries to setup a stack guard for the thread which, again is highly
platform dependent. Take for example the [implementation for Unix based
platforms](https://github.com/rust-lang/rust/blob/de6060b09d23d8c50afe88354358aa056c27ad9f/src/libstd/sys/unix/thread.rs#L215-L434).
There's a few different ways to calculate this and set this up depending on
which Unix system you're on. Yet for Windows there is [no need for it to setup
one](https://github.com/rust-lang/rust/blob/de6060b09d23d8c50afe88354358aa056c27ad9f/src/libstd/sys/windows/thread.rs#L101-L110)
according to it's implementation. It's just another case of platform dependent
code we don't need to worry about since the runtime handles it for us!

We then call an `init` function for handling stack overflow errors on a per
platform basis. [For Windows we do
this](https://github.com/rust-lang/rust/blob/de6060b09d23d8c50afe88354358aa056c27ad9f/src/libstd/sys/windows/stack_overflow.rs)
by registering a `VectoredExceptionHandler` with the system and then calling
`SetThreadStackGuarantee` which are part of `WinAPI`. Consequently this leads to
my favorite comment I've found about platform specific code issues I've found
when reading all this code for the article:

```rust
impl Handler {
    pub unsafe fn new() -> Handler {
        // This API isn't available on XP, so don't panic in that case and just
        // pray it works out ok.
        if c::SetThreadStackGuarantee(&mut 0x5000) == 0 {
            if c::GetLastError() as u32 != c::ERROR_CALL_NOT_IMPLEMENTED as u32 {
                panic!("failed to reserve stack space for exception handling");
            }
        }
        Handler
    }
}
```

I mean I too hope no one is using Windows XP anymore, but hey the Army was
teaching me how to install operating systems with XP in 2014 at AIT when it was
past EOL so who knows, maybe someone does write Rust for XP. At least it's being
handled! On Unix of course [the implementation is also
different](https://github.com/rust-lang/rust/blob/de6060b09d23d8c50afe88354358aa056c27ad9f/src/libstd/sys/unix/stack_overflow.rs)
but it does more or less the same thing in terms of setting up code to handle
stack overflows. Again, I'm so glad I never ever have to write this code myself.
What's next then that the runtime does?

```rust
        // Next, set up the current Thread with the guard information we just
        // created. Note that this isn't necessary in general for new threads,
        // but we just do this to name the main thread and to give it correct
        // info about the stack bounds.
        let thread = Thread::new(Some("main".to_owned()));
        thread_info::set(main_guard, thread);
```

Whenever your code panics on the main thread and you see `thread 'main' panicked
at` that value `main` comes right here from this bit of code. Otherwise it would
be a thread with no name. `Thread::new` is also an internal function that allows
a Thread type to be constructed without actually spawning a thread. Why would we
need to do this? Well the code at this point is already spawned as the main
thread. This bit of code just names it and sets the guard information for it, no
need to spawn a new thread to run the code on. The runtime handles all this for
us! Next up is our good old friends `argc` and `argv`!

```rust
        // Store our args if necessary in a squirreled away location
        sys::args::init(argc, argv);
```

Which again is platform dependent! It's pretty much the running theme of this
article at this point. I found my other favorite bit of code here with a
function called `really_init` in the Linux implementation of `sys::args::init`

```rust
    static mut ARGC: isize = 0;
    static mut ARGV: *const *const u8 = ptr::null();
    // We never call `ENV_LOCK.init()`, so it is UB to attempt to
    // acquire this mutex reentrantly!
    static LOCK: Mutex = Mutex::new();

    unsafe fn really_init(argc: isize, argv: *const *const u8) {
        let _guard = LOCK.lock();
        ARGC = argc;
        ARGV = argv;
    }

    #[inline(always)]
    pub unsafe fn init(_argc: isize, _argv: *const *const u8) {
        // On Linux-GNU, we rely on `ARGV_INIT_ARRAY` below to initialize
        // `ARGC` and `ARGV`. But in Miri that does not actually happen so we
        // still initialize here.
        #[cfg(any(miri, not(all(target_os = "linux", target_env = "gnu"))))]
        really_init(_argc, _argv);
    }

```

On Windows and OSX this is just an empty function, but just below the function
definition for `init` we can see how it gets these args for the call to
`std::env::args`. For Windows it uses this code:

```rust
pub fn args() -> Args {
    unsafe {
        let lp_cmd_line = c::GetCommandLineW();
        let parsed_args_list = parse_lp_cmd_line(lp_cmd_line as *const u16, || {
            current_exe().map(PathBuf::into_os_string).unwrap_or_else(|_| OsString::new())
        });


        Args { parsed_args_list: parsed_args_list.into_iter() }
    }
}
```

For OSX it's also somewhat involved:

```rust
    pub fn args() -> Args {
        use crate::os::unix::prelude::*;
        extern "C" {
            // These functions are in crt_externs.h.
            fn _NSGetArgc() -> *mut libc::c_int;
            fn _NSGetArgv() -> *mut *mut *mut libc::c_char;
        }


        let vec = unsafe {
            let (argc, argv) =
                (*_NSGetArgc() as isize, *_NSGetArgv() as *const *const libc::c_char);
            (0..argc as isize)
                .map(|i| {
                    let bytes = CStr::from_ptr(*argv.offset(i)).to_bytes().to_vec();
                    OsStringExt::from_vec(bytes)
                })
                .collect::<Vec<_>>()
        };
        Args { iter: vec.into_iter(), _dont_send_or_sync_me: PhantomData }
    }

```

For Linux it's just:

```rust
    pub fn args() -> Args {
        Args { iter: clone().into_iter(), _dont_send_or_sync_me: PhantomData }
    }
```

But it's more complex than that because of this bit:

```rust
    /// glibc passes argc, argv, and envp to functions in .init_array, as a non-standard extension.
    /// This allows `std::env::args` to work even in a `cdylib`, as it does on macOS and Windows.
    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    #[used]
    #[link_section = ".init_array.00099"]
    static ARGV_INIT_ARRAY: extern "C" fn(
        crate::os::raw::c_int,
        *const *const u8,
        *const *const u8,
    ) = {
        extern "C" fn init_wrapper(
            argc: crate::os::raw::c_int,
            argv: *const *const u8,
            _envp: *const *const u8,
        ) {
            unsafe {
                really_init(argc as isize, argv);
            }
        }
        init_wrapper
    };


    pub unsafe fn cleanup() {
        let _guard = LOCK.lock();
        ARGC = 0;
        ARGV = ptr::null();
    }


    pub fn args() -> Args {
        Args { iter: clone().into_iter(), _dont_send_or_sync_me: PhantomData }
    }


    fn clone() -> Vec<OsString> {
        unsafe {
            let _guard = LOCK.lock();
            (0..ARGC)
                .map(|i| {
                    let cstr = CStr::from_ptr(*ARGV.offset(i) as *const libc::c_char);
                    OsStringExt::from_vec(cstr.to_bytes().to_vec())
                })
                .collect()
        }
    }

```

Again I cannot stress enough how glad I am to not be writing this code myself.
If you want to peruse the code a bit more you can find the `args` implementation
for Unix systems
  [here](https://github.com/rust-lang/rust/blob/de6060b09d23d8c50afe88354358aa056c27ad9f/src/libstd/sys/unix/args.rs)
  and Windows
  [here](https://github.com/rust-lang/rust/blob/de6060b09d23d8c50afe88354358aa056c27ad9f/src/libstd/sys/windows/args.rs).
  Now finally after all of that we run our main function:

```rust
        // Let's run some code!
        let exit_code = panic::catch_unwind(|| {
            sys_common::backtrace::__rust_begin_short_backtrace(move || main())
        });
```

It creates a backtrace for our `main` function by passing a closure into the
`__rust_begin_short_backtrace` function and that gets wrapped as a closure which
is [passed to
catch_unwind](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html). This
function will invoke the closure passed to it which will call our `main`
function all while letting us catch panics that unwind (not necessarily those
that abort) and get the `exit_code` from it! Well specifically a `Result<i32,
Box<dyn std::error::Error>>`. This is because of the last few lines of code that
exist in the runtime:

```rust
        sys_common::cleanup();
        exit_code.unwrap_or(101) as isize
    }
}
```

First we want to cleanup whatever we need to before exiting. That code is:

```rust
/// One-time runtime cleanup.
pub fn cleanup() {
    static CLEANUP: Once = Once::new();
    CLEANUP.call_once(|| unsafe {
        sys::args::cleanup();
        sys::stack_overflow::cleanup();
        at_exit_imp::cleanup();
    });
}
```

We run the platform specific code to cleanup the arguments, the stack overflow
guards, and then run any other cleanup code with the `at_exit_imp::cleanup`
function that each platform defines. Then lastly we either unwrap the value that
we would get from `main().report()` or if the code panicked and we were able to
catch it we use a value of `101` instead and then cast the `i32` to an `isize`.
This is then used to set the exit code for the program like we mentioned before.
After that the platform can then use it to determine what to do next and that's
it! That's Rust's runtime.

## Conclusion
We walked through Rust's runtime that, while not too big, is deceptive in the
complexity it hides. Most of what it does is setting up some things that are
useful to have like handlers for stack overflows or safely abstracting over
`argc` and `argv` all in platform specific ways. It also runs our `main`
function after which it cleans up the things it setup, as well as any platform
specific cleanup. The runtime also helps catch panics for us if possible and
returns the proper exit code to the operating system if it does or does not
panic. It's really nice to have code that handles this complexity for us, is
made to work for the platform we compile our code on, and simplifying everything
needed to have a relatively well protected program in the case of unfortunate
things happening. All without us needing to write any of the code ourselves. I
hope you enjoyed this little dive into the rustc codebase and that you learned
something interesting as a result of it.
