# Oxidizing the Technical Interview

*Inspired by [Reversing the technical interview](https://aphyr.com/posts/340-reversing-the-technical-interview) and [Hexing the technical interview](https://aphyr.com/posts/341-hexing-the-technical-interview)*

Years ago, as a young'in from the Rust Belt, you dreamed of becoming a systems
programmer, an Abyss Gazer if you will. You wanted to live, breathe, and die by
being as close to the metal as you could. You grew up in a harsh environment of
reading assembly output to shave off a few instructions in an attempt to make
the fastest code, segfaults, and poorly documented hardware, all to become a
systems programmer. You look back upon those years fondly as it's turned you
into the programmer you are today. An unparalleled Abyss Gazer, an Eldritch One,
one who makes mortal men fear and worship the code you've wrought upon this
world that holds the very foundations of it together. Today you've come for an
interview at the request of a friend for a position using Rust. They hope to be
able to hire you so that you may channel the will of Ferris the Rustacean into
all the code that flies from your fingers for them. Of course they can't just
give you the job. Your potential future comrades must see for themselves whether
the rumors about your skill are true. You look to your interviewer who asks if
you're ready to get started. You nod, ready to take on any task given to you.
"You're given an array of size n + 1 and the integers 0 to n. Find the duplicate
in the list using an algorithm that optimizes for size and one that optimizes
for execution in terms of Big O. Make sure to only use the stable release of
Rust as we do not use nightly here.", your interviewer explains. "Does it
matter if I just do 1 algorithm that can do both in O(1)?", you ask. The
interviewer looks over their notes in confusion as this was not one of the
answers they were told was possible. They're a younger engineer and you are
being tested for a Principal Engineering position. It's nice that you're
already able to prove you can do your job to teach those beneath you in terms
of seniority. "Uh sure. If you can uh prove it.", they say. You whip out your
MacBook to act as a conduit to Wozniak himself, so that he may guide you as he
has many times before. You empty yourself to become the vessel for those who
gazed into the abyss before you. "Let's get started by creating a project to
house the summoning." you say, making sure to explain your process. The
interviewer gives a slight nod, but they seem more on guard than they were
earlier. Maybe summoning wasn't the right word. Incantation maybe? Either way,
you type the magic words to your console like all Rustaceans before, now, and
after will to start the world anew.

```bash
cargo new interview
```

"Alright let's setup our manifest to ship high quality artisanal hand crafted
code to our end users." You open up the `Cargo.toml` file and fill it with the
incantation of optimization you've ingrained into your soul

```toml
[package]
name = "interview"
version = "0.1.0"
authors = ["Z̴̧̡̢̨̛A̷̡̡̢̢̡L̶̢̧̢̪̬Ģ̵̡̢̢̧̛Ơ̵̡̡̬͇̬ ̵̡̧̢̛̳Ċ̷̢̨̢̼̰Ǫ̷̨̧̢̛͎M̵̡̨̖̹̣E̴̢̧̨̨̛S̸̱̠̹̮̣  <Z̴̧̡̢̨̛A̷̡̡̢̢̡L̶̢̧̢̪̬Ģ̵̡̢̢̧̛Ơ̵̡̡̬͇̬ ̵̡̧̢̛̳Ċ̷̢̨̢̼̰Ǫ̷̨̧̢̛͎M̵̡̨̖̹̣E̴̢̧̨̨̛S̸̱̠̹̮̣@gmail.com>"]
edition = "2018"

[dependencies]

[profile.release]
panic = "abort"
lto = true
codegen-units=1
opt-level="s"
```

"Wait what's up with your name?", the interviewer asks. You wave them aside and
reply, "Worry not sweet summer child, lest yee be cursed as I have been. The
important part is that we've optimized for code gen size and to strip out as
many unnecessary assembly instructions as possible from the code." "Wouldn't
that mean you should use z for size?", they ask. "What? And add one whole
useless instruction to the output? I think not." You quickly open up your editor
to `src/main.rs`. You crack your knuckles and start off your incantation by
releasing the limiters.

```rust
#![feature(const_fn)]
#![feature(const_fn_union)]
#![feature(const_panic)]
#![feature(const_if_match)]
#![feature(const_raw_ptr_deref)]
#![feature(lang_items)]
#![feature(no_core)]
#![feature(optin_builtin_traits)]
#![feature(allow_internal_unstable)]
#![feature(intrinsics)]
#![feature(untagged_unions)]
```

"Wait wait wait. I said use a stable Rust compiler and these are all nightly
features. Also what in the world do you need all this for?" asks your
interviewer in anger. "I am using a stable Rust compiler. Just like you asked"
you say showing them the version number, though this is more a technicality, but
they don't know it yet. "Uhhh okay whatever keep going".

"No libstd, no core, only me" you whisper as you place the line that will enable
true freedom from the shackles of having any Rust dependencies whatsoever.

```rust
#![no_core]
```

It's then that you see the look of abject horror on your interviewer's face.
Good good let them stare into the depths of what system's programming truly is.
"Alright so IO is a thing as much as functional programmers try to avoid it by
using 'pure' functions. If we want to see the output of something we'll need to
make some kind of syscall since operating systems exist. We'll link to libc so
that we can use printf at least. We'll link directly to the OSX libc here with
this line, but depending on the OS we would want to link a different libc
implementation instead." It pains you to have any dependency whatsoever, but
you're not trying to also write a whole Operating System today, that's for next
week's interview. There's no librs you can use yet unfortunately and so, like a
blacksmith carefully linking metal rings in a set of chainmail, you link to the
venerable libc.

```rust
#[allow(non_camel_case_types)]
pub type c_char = i8;
#[allow(non_camel_case_types)]
pub type c_int = i32;
#[link(name = "System")] // OSX libc
extern "C" {
    pub fn printf(format: *const c_char, ...) -> c_int;
}
```

"Okay okay, but are you going to even get to the actual problem?", the
interviewer asks, clearly miffed that you're bothering to use C code instead of
`println`. Oh the sweet summer child. Everything is built on top the work of
Kerninghan & Ritchie. We can't escape their legacy that easily. "Okay let's add
some macros to cut down on some of the repetitive stuff later. We have to put
them here otherwise they won't exist at all if they come after the places we
invoke them at." you utter, in an attempt to convince the clearly bereaved
interviewer that you do actually understand the language you're using and that
you're not yanking them around for no reason.

```rust
macro_rules! matches {
    ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )?) => {
        match $expression {
            $( $pattern )|+ $( if $guard )? => true,
            _ => false
        }
    }
}
macro_rules! copy_clone_eq_impls {
   ($($t:ty)*) => {
     $(impl Copy for $t {}
       impl Clone for $t {
         fn clone(&self) -> Self {
            *self
         }
       }
       impl PartialEq for $t {
           fn eq(&self, other: &$t) -> bool {
               (*self) == (*other)
           }
       }
     )*
   }
}
```

"Why not derive for those?", they ask. You look at them dead in the eyes and
explain to them, "I can't implement these for primitives via a derive macro. I
have to *become* the compiler itself in order for these to work. Now let's
continue. We have an apple pie to make and first we must invent the universe.
Let's add some traits and import some compiler intrinsics we're going to need."

```rust
#[lang = "sized"]
pub trait Sized {}
#[lang = "freeze"]
auto trait Freeze {}
extern "rust-intrinsic" {
    fn offset<T>(dst: *const T, offset: isize) -> *const T;
}
#[lang = "receiver"]
trait Receiver {}
#[lang = "index"]
trait Index<Idx: ?Sized> {
    type Output: ?Sized;
    fn index(&self, index: Idx) -> &Self::Output;
}
impl<T, I> Index<I> for [T]
where
    I: SliceIndex<[T]>,
{
    type Output = I::Output;
    fn index(&self, index: I) -> &I::Output {
        index.index(self)
    }
}
trait SliceIndex<T: ?Sized> {
    type Output: ?Sized;
    fn get(self, slice: &T) -> Option<&Self::Output>;
    fn get_mut(self, slice: &mut T) -> Option<&mut Self::Output>;
    unsafe fn get_unchecked(self, slice: &T) -> &Self::Output;
    unsafe fn get_unchecked_mut(self, slice: &mut T) -> &mut Self::Output;
    fn index(self, slice: &T) -> &Self::Output;
    fn index_mut(self, slice: &mut T) -> &mut Self::Output;
}
#[lang = "copy"]
trait Copy: Clone {}
trait Clone: Sized {
    fn clone(&self) -> Self;
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}
#[lang = "eq"]
trait PartialEq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool;
    fn ne(&self, other: &Rhs) -> bool {
        !self.eq(other)
    }
}
#[lang = "partial_ord"]
trait PartialOrd<Rhs: ?Sized = Self>: PartialEq<Rhs> {
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;
    fn lt(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Less))
    }
    fn le(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Less) | Some(Equal))
    }
    fn gt(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Greater))
    }
    fn ge(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Greater) | Some(Equal))
    }
}
#[lang = "not"]
trait Not {
    type Output;

    fn not(self) -> Self::Output;
}
#[lang = "neg"]
trait Neg {
    type Output;
    fn neg(self) -> Self::Output;
}
#[lang = "sub"]
trait Sub<Rhs = Self> {
    type Output;
    fn sub(self, rhs: Rhs) -> Self::Output;
}
trait Termination {
    fn report(self) -> i32;
}
```

Like any good Rustacean worth their sea salt, you've drilled every bit of libstd
and libcore into your mind so that you only pull in the items that you need and
nothing more. You believe in zero cost abstractions, no, you are zero cost
abstractions. Even the possibility of paying for just a CPU cycle or bit more
disgusts you. Only run what must be run and nothing more. "Wait aren't these
traits and items from libcore and libstd? What. What's going on here?" your
interviewer asks, clearly distressed that it's taking this much code. "Well yes
since we're using `#![no_core]` they simply do not exist anymore. We're only
defining exactly what we need from there and nothing more. Besides I'm being as
explicit as possible by letting you stare at the true cost it takes to program
on any machine, by taking the syntax sugar and prebuilt code away. Speaking of
sugar got any coffee? I like it black like the soul I sold to 'them' to get
better at this kind of stuff." Oof maybe the self deprecating joke was a bit
much, even if it was true. You were only trying to lighten the mood, but the
interviewer seems to have pretended to hear nothing. "Okay now lets implement
all these traits then!" you say, fingers flying over the keyboard, never leaving
insert mode. Esc, h, j, k, l? Ha those keys are for those who haven't dedicated
themselves to the abyss.

```rust
impl<T, I> Index<I> for [T]
where
    I: SliceIndex<[T]>,
{
    type Output = I::Output;
    fn index(&self, index: I) -> &I::Output {
        index.index(self)
    }
}
trait SliceIndex<T: ?Sized> {
    type Output: ?Sized;
    fn get(self, slice: &T) -> Option<&Self::Output>;
    fn get_mut(self, slice: &mut T) -> Option<&mut Self::Output>;
    unsafe fn get_unchecked(self, slice: &T) -> &Self::Output;
    unsafe fn get_unchecked_mut(self, slice: &mut T) -> &mut Self::Output;
    fn index(self, slice: &T) -> &Self::Output;
    fn index_mut(self, slice: &mut T) -> &mut Self::Output;
}
#[lang = "copy"]
trait Copy: Clone {}
trait Clone: Sized {
    fn clone(&self) -> Self;
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}
#[lang = "eq"]
trait PartialEq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool;
    fn ne(&self, other: &Rhs) -> bool {
        !self.eq(other)
    }
}
#[lang = "partial_ord"]
trait PartialOrd<Rhs: ?Sized = Self>: PartialEq<Rhs> {
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;
    fn lt(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Less))
    }
    fn le(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Less) | Some(Equal))
    }
    fn gt(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Greater))
    }
    fn ge(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Greater) | Some(Equal))
    }
}
#[lang = "not"]
trait Not {
    type Output;

    fn not(self) -> Self::Output;
}
#[lang = "neg"]
trait Neg {
    type Output;
    fn neg(self) -> Self::Output;
}
#[lang = "sub"]
trait Sub<Rhs = Self> {
    type Output;
    fn sub(self, rhs: Rhs) -> Self::Output;
}
trait Termination {
    fn report(self) -> i32;
}
#[lang = "slice"]
impl<T> [T] {
    #[allow(unused_attributes)]
    #[allow_internal_unstable(const_fn_union)]
    const fn len(&self) -> usize {
        unsafe { Repr { rust: self }.raw.len }
    }

    fn as_ptr(&self) -> *const T {
        self as *const [T] as *const T
    }

    fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut [T] as *mut T
    }
}

#[lang = "const_ptr"]
impl<T: ?Sized> *const T {
    unsafe fn add(self, count: usize) -> Self
    where
        T: Sized,
    {
        self.offset(count as isize)
    }

    unsafe fn offset(self, count: isize) -> *const T
    where
        T: Sized,
    {
        offset(self, count)
    }
}
#[lang = "mut_ptr"]
impl<T: ?Sized> *mut T {
    unsafe fn add(self, count: usize) -> *mut T
    where
        T: Sized,
    {
        self.offset(count as isize)
    }
    unsafe fn offset(self, count: isize) -> *mut T
    where
        T: Sized,
    {
        offset(self, count) as *mut T
    }
}
impl<T> SliceIndex<[T]> for usize {
    type Output = T;
    fn get(self, slice: &[T]) -> Option<&T> {
        if self < slice.len() {
            unsafe { Some(self.get_unchecked(slice)) }
        } else {
            None
        }
    }
    fn get_mut(self, slice: &mut [T]) -> Option<&mut T> {
        if self < slice.len() {
            unsafe { Some(self.get_unchecked_mut(slice)) }
        } else {
            None
        }
    }
    unsafe fn get_unchecked(self, slice: &[T]) -> &T {
        &*slice.as_ptr().add(self)
    }
    unsafe fn get_unchecked_mut(self, slice: &mut [T]) -> &mut T {
        &mut *slice.as_mut_ptr().add(self)
    }
    fn index(self, slice: &[T]) -> &T {
        &(*slice)[self]
    }
    fn index_mut(self, slice: &mut [T]) -> &mut T {
        &mut (*slice)[self]
    }
}
#[allow(unconditional_recursion)]
impl PartialOrd for usize {
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        self.partial_cmp(other)
    }
}
impl Not for bool {
    type Output = bool;
    fn not(self) -> bool {
        !self
    }
}
impl Neg for isize {
    type Output = isize;

    fn neg(self) -> isize {
        -self
    }
}
impl Sub for usize {
    type Output = usize;
    fn sub(self, rhs: usize) -> Self::Output {
        self - rhs
    }
}
impl Termination for () {
    fn report(self) -> i32 {
        0
    }
}
impl<T: ?Sized> Receiver for &T {}
impl<T: ?Sized> Receiver for &mut T {}
copy_clone_eq_impls!(usize u64 bool);
```

"Wait hold on, the op Sub uses - as part of it's definition? The thing it's
supposed to define and wait Not does this too with ! and wait wait wait." Your
interviewer is sputtering in disbelief. Maybe they were too young to be
corrupted by your influence. It's too late now. You've already shown them what
the grimoire contains. You can only go deeper now. You're almost done.

```rust
#[allow(dead_code)] // It's actually needed! Zombie Ordering
enum Ordering {
    Less = -1,
    Equal = 0,
    Greater = 1,
}
use Ordering::*;
enum Option<T> {
    Some(T),
    None,
}
use crate::Option::*;
#[repr(C)]
pub(crate) union Repr<T> {
    pub(crate) rust: *const [T],
    rust_mut: *mut [T],
    pub(crate) raw: FatPtr<T>,
}
#[repr(C)]
pub(crate) struct FatPtr<T> {
    data: *const T,
    pub(crate) len: usize,
}
```

You create the only concrete types that you'll need beyond the primitives, but
they're transitory like the feeble lives we live. Your interviewer just looks at
you as you show them the depths of what Rust truly is and the secrets it hides
behind the veil. Your interviewer begins to sputter unintelligible words.
"nmbrs. enms in em?!?" You better wrap this up quickly and bring them back to
the living before they cross the river Styx. You're trying to get a job here.

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

This is it. The final stretch. The anticipation is killing you. "What's with
this type signature? What's start?! Rust has no runtime!", your interviewer
sputters. You turn to them, "The greatest trick the Devil ever played was
convincing C and Rust programmers their language has no runtime." You turn back
to your computer. You sigh and say, "Look we're almost done. This is the last
bit I swear." You begin typing in the last words that will let you have an O(1)
runtime and O(1) space usage when you finally execute the code. Truly the most
magical of the incantations you've wrought thus far.

```rust
pub const DUPLICATE: u64 = {
    pub const LIST: &'static [u64] = {
        unsafe {
            &*Repr {
                raw: FatPtr {
                    data: &[
                        1u64, 2u64, 3u64, 7u64, 4u64, 5u64,
                        6u64, 7u64, 8u64, 9u64, 10u64,
                    ] as *const u64,
                    len: 11,
                },
            }
            .rust
        }
    };

    const fn recurse(item: usize, compare: usize) -> u64 {
        if compare == 0usize && item != 0usize {
            recurse(item - 1, 10)
        }
        // else if compare == 0 && item == 0
        // Pigeon Hole principle means we're guaranteed to handle this case
        else if compare == item {
            recurse(item, compare - 1)
        } else {
            if LIST[compare] == LIST[item] {
                return LIST[compare];
            } else {
                recurse(item, compare - 1)
            }
        }
    }
    recurse(10, 10)
};
```

"Wait wait wait that's the naive solution! This is O(n^2) runtime!", your
interviewer says exasperated at the rigamarole you've run them through. "No no
no no that's only theoretically true!", you say, "In practice the use of `const`
causes the compiler to make this constant time and space! Watch!" You crack your
hands and open up the gates of hell yourself typing the heretical incantation
only the privileged rustc coders are allowed to use to compile the compiler lest
the spawn of the underworld destroy the illusion of stability.

```bash
interview is 📦 v0.1.0 via 🦀 v1.43.0
❯ RUSTC_BOOTSTRAP=1 cargo run --release
   Compiling oxidizing-the-interview v0.1.0 (/Users/Z̴̧̡̢̨̛A̷̡̡̢̢̡L̶̢̧̢̪̬Ģ̵̡̢̢̧̛Ơ̵̡̡̬͇̬ ̵̡̧̢̛̳Ċ̷̢̨̢̼̰Ǫ̷̨̧̢̛͎M̵̡̨̖̹̣E̴̢̧̨̨̛S̸̱̠̹̮̣/interview)
    Finished release [optimized] target(s) in 0.17s
     Running `target/release/interview`
The value is: 7

interview is 📦 v0.1.0 via 🦀 v1.43.0
❯ RUSTC_BOOTSTRAP=1 cargo rustc --release -- --emit asm
    Compiling interview v0.1.0 (/Users/Z̴̧̡̢̨̛A̷̡̡̢̢̡L̶̢̧̢̪̬Ģ̵̡̢̢̧̛Ơ̵̡̡̬͇̬ ̵̡̧̢̛̳Ċ̷̢̨̢̼̰Ǫ̷̨̧̢̛͎M̵̡̨̖̹̣E̴̢̧̨̨̛S̸̱̠̹̮̣/interview)
     Finished release [optimized] target(s) in 0.19s
    
interview is 📦 v0.1.0 via 🦀 v1.43.0
❯ cat target/release/deps/oxidizing_the_interview-6201aba656776dbc.s
    .section	__TEXT,__text,regular,pure_instructions
    .macosx_version_min 10, 7
    .globl    _main
_main:
    pushq    %rbp
    movq    %rsp, %rbp
    leaq    l___unnamed_1(%rip), %rdi
    movl    $7, %esi
    xorl    %eax, %eax
    callq    _printf
    xorl    %eax, %eax
    popq    %rbp
    retq

    .section    __TEXT,__const
l___unnamed_1:
    .asciz    "The value is: %d\n"


.subsections_via_symbols

interview is 📦 v0.1.0 via 🦀 v1.43.0
❯ cat target/release/deps/interview-6201aba656776dbc.s | wc -l
    20
```

"See I told you I would only use a stable compiler. You just had to trust me!
Now look at this code's asm output. Only 20 lines! 17 if you cut out the empty
newlines! No handwritten assembly needed! It's all high level Rust code and look
the 7 is in there and it just gets pushed into a register and printed out, so
I've proved it's constant time and space in practice!" Your interviewer, eyes
white and rolled back into their head says, "We'll contact you later. Thanks for
coming in." Oh no, you've heard this line before. You might not get the job.
"Look I know I wrote the code with only one array, but I could make this work
for any array by using a build.rs script to insert them into the file at compile
time based off an external file used as input to define what numbers it should
contain and thus allowing the code to find any duplicates like the problem
asked for!", you say hoping to impress the interviewer in a last ditch effort.
"No really. We'll contact you. You've more than answered the question." your
interviewer sputters, rushing to get you out of the room. You can't help but
feel like you've failed the interview, but your dark patron is pleased for
showing another the truth and the way of the computer they use. Oh well, onto
the next interview. Maybe they'll be ready for your enlightenment.

*You can find the code from this blog post
[here](https://github.com/mgattozzi/oxidizing-the-technical-interview).*
