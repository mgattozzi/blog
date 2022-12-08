# Weird Expressions and Where to Find Them

This is a written transcript of my RustConf 2022 talk using rustc at commit
6491eb1e6c9fac20faad11e5da16db3185b2410d You can find the video of the talk
[here](https://www.youtube.com/watch?v=tNVDjMxCz-c).

--------------------------------------------------------
## Introduction

My name is Michael Gattozzi and today I want to bring you on a journey from
"What?" to "No way that works" to "Wait why does that work?!" and then true
understanding and acceptance. Many an article or talk has been created about
"Idiomatic Rust" or "Proper Code" or whatever else the kids say these days.
However, I've spent a long time gazing into the "non-idiomatic" abyss of the
weird-exprs.rs test file. A file which contains the kinds of programs that are
non-sensical Rust, but technically correct Rust, the best kind of correct. A
file hidden deep in the bowels of the Rust Compiler Test Suite designed to
protect against messing up parsing rules and testing weird edge cases that rustc
must accept, even if we don't want it too. A test file known to the compiler
devs as a necessary evil and sequestered from the public lest it make both old
and new Rustaceans run away in abject horror. A test file that shows just what
kinds of wretched programs mortals are capable of inflicting upon the the world.
Come with me on a journey to truly understand this file by first understanding
where and why it came into being.

--------------------------------------------------------
## History

In the beginning there was commit (664b0ad)3fcead4fe4d22c05065a82a338770c429
made on August 19th, 2011. In it was a file wierd-exprs.rs, this was a mistake
that was not noticed for a few commits and then renamed to weird-exprs.rs on Sep
26, 2011. This file will be 11 years old in two weeks. It's been around for
quite some time, because no matter what point in time the language exists at we
have valid programs that we can write because of how the language is structured,
that we wouldn't want to write, but we should still test for to make sure we
don't break any edge cases and invariants. There are a lot of programs that
rustc can accept and produce code for, but that we as programmers would find are
not easy to read, helpful, or do anything we'd actually want a computer to do.
This test suite has an equivalent analogy in the English sentence Buffalo
buffalo Buffalo buffalo buffalo buffalo Buffalo buffalo. It follows all of the
correct grammar of the language, however, it is nonsensical, but is also
technically allowed. weird-exprs.rs is the same thing. Here is one of those test
cases from the file and trust me when I say this is the least gnarly one to look
at.

```rust
fn evil_lincoln() {
    let _evil = println!("lincoln");
}
```

It just prints "lincoln" and assigns the unit type returned from println to
_evil. Sure you can assign the unit value to a variable, but we wouldn't
actually want to do that in the code we write. This is what I mean by valid, but
not helpful, code. Now you might be wondering, "Why is it called evil_lincoln?".
Well let's look at it in the original commit that added it.

```rust
fn evil_lincoln() {
    let evil <- log "lincoln";
}
```

As you can see, println used to be called log and for anyone not up to date on US
Presidential History Lincoln lived in a log cabin he had built himself in
Illinois back in the day. The name of the test survived, but not the pre 1.0
syntax. Rust and what it looks like has both changed a lot and not a lot in the
11 years since this test was added. We use println! a macro, not log a built in
keyword, and we assign with equals not left arrow these days. weird-exprs.rs has
existed through most of Rust's creation and every step of the way since its
inception it has guarded the language from breaking its grammar and parsing
rules. It was here before many of us started using Rust and it will be here
longer than many of us will be. Now that we understand just where this file came
from, let's start looking into a few choice cases to learn a bit more about how
Rust works and let's get weird.

--------------------------------------------------------
## Let's Get Weird

```rust
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_braces, unused_must_use, unused_parens)]
#![allow(uncommon_codepoints, confusable_idents)]
```

If you look at the file today you can see these allow pragmas meaning we're
going to have a good time because we're allowing ourselves to use the good code.
Do note I can't cover every case with the time we have today and so I have
chosen the ones I think we can learn the most from. If you want to see all of
the tests, which I really think you should, they're absolutely fascinating and
will make you scratch your head for a bit as you figure them out, then I suggest
reading them all here at src/test/ui/weird-exprs.rs in the rustc repo at commit
(491eb1)e6c9fac20faad11e5da16db3185b2410d.

```rust
if true {
    println!("Hello I'm true");
}
```

Let's talk about if. if lets you test some expression for a boolean value of
true and if it's true executes the block. In this case we check it's true and
then execute the println! macro.

```rust
if false {
    println!("Hello I'm true");
} else {
    println!("Hello I'm false");
}
```

We can also have an optional else block that let's us do something in case it's
false. In this case we'd print "Hello I'm false". Did you know that if is an
expression though in Rust and not a statement? This means it's far more flexible
and can be used in cases that expect an expression such as variable assignment.

```rust
let x = if true {
    1
} else {
    0
};
```

This means you can test a condition and assign it a value depending on whether
the condition is true or false so long as the type in both blocks is the same.
Did you know that the condition if accepts is also an expression. The only thing
the condition must do is evaluate to a boolean.

```rust
if {
  println!("Evaluating the expression block");
  true
} {
    println!("Hello I'm true");
} else {
    println!("Hello I'm false");
}
```

In this case we use an expression block which will execute everything inside the
block first. In this case we print the sentence "Evaluating the expression
block", before evaluating and returning whatever the final value is without a
semicolon, in this case true. Something to note about expression blocks is that
they always return a type, it just defaults to the unit type if there is no
type.

```rust
match 1u8 {
    1 => println!("I am one"),
    _ => println!("I am neither one or zero"),
}
```

Now let's talk about match for a second. It is also an expression that takes an
expression, in this case the value 1 which is a u8, and pattern matches on it.
It tests the patterns in order of writing and then stops at the first one that
matches the pattern and then it executes the expression to the right of it. Rust
forces you to match every possible pattern and so we can use _ as the catchall
pattern for every other number that a u8 can be. In this example 1 matches on 1
and prints "I am one".

```rust
let x = 1u8;
match x {
    _ if x.is_even() => println!("I am an even number"),
    _ if x.is_odd() => println!("I am an odd number"),
    _ => unreachable!("Broke math itself, x is neither odd or even"),
}
```

We can go even further with match as each pattern can accept an optional if
guard alongside the pattern. Here we check the first pattern for x and since
it's the catchall we match and then ask if x is even. We see that it's not and
move on to the next pattern. We know the pattern will match and then check x is
odd which it is since 1 is an odd number and then print "I am an odd number".

Let's recap:

1. if is an expression
2. if accepts expressions that can evaluate to a boolean
3. match is an expression
4. match patterns can have an optional if expression
5. expressions that evaluate to a value can be assigned to a variable

Now you might be seeing where I'm going with this. If you can put an expression
inside an expression and if is an expression then you can get something like
this test from weird-exprs.rs which is designed to test that you can arbitrarily
nest if expressions and that you can nest them in match expressions

```rust
fn match_nested_if() {
    let val = match () {
        () if if if if true {true} else {false} {true} else {false} {true} else {false} => true,
        _ => false,
    };
    assert!(val);
}
```

So let's walk through what this test is doing.

1. We first match on the unit type and go to the first pattern
2. We see the pattern is the unit type and so we now need to check the nested if statements
3. We evaluate the first statement if true and see we should use the if block, not the else block which evaluates to true
4. We then use the true value from the first if as the input to the second if which means we choose the if block, not the else block which evaluates to true
5. We then evaluate the third if and just like the previous two we choose the if block and
not the else block which evaluates to true
6. Then we check if the last if is true for the match pattern which it is and so we choose that pattern in the match statement.
7. We then assign the value true to the variable val
8. We then assert that val is true, which it is, and so the test passes

Let's kick things up a notch and talk about functions.

```rust
fn example() {
    println!("I am a function")
}
```

We can define a function example with the above syntax and when called it will
execute anything in the block, in this case it will print out "I am a function".
Okay that's easy. Now we can also define functions for types like so:

```rust
struct Bar;

impl Bar {
    fn example(self) {
        println!("I am a function that takes Bar as input");
    }
}
```

Here we have a struct Bar that we impl a function example for that takes Bar by
value with the self argument and then prints out "I am a function that takes Bar
as input". Now what if we changed it so that it also returned a type Bar instead
of printing out something? It would look like this:

```rust
struct Bar;

impl Bar {
    fn example(self) -> Self {
        Self
    }
}
```

Where the only difference is we're returning Self which is an alias for Bar here
given we're inside an impl block. This would let us write some interesting code
because then we could call example as many times as we wanted like this:

```rust
let bar = Bar.example()
    .example()
    .example()
    .example();
```

We could keep chaining example here because we'd create a new Bar in each call
to example. I promise this is going somewhere. I want to talk about function
traits for a second. These let us pass functions into other functions
generically. Here's what I mean.

```rust
fn example<F>(function: F)
    where F: FnOnce() -> String
{
    println!("function string: {}", function());
}

example(|| String::from("foo"));
example(|| String::from("bar"));
```

We define a function example that has a generic parameter F that is the type of
the argument function. We restrict what F is with a where clause saying that it
must impl FnOnce and that it takes no args and returns a String. We then define
a body that will print out "function string: " with the value of the function
that we pass in that we invoke. In this case foo and bar. We should note
closures implement FnOnce as do named functions.

Let's recap:

1. We can define functions that when called will execute what's inside the block
2. We can implement a function for a type that will take itself as the input
3. We can have a function return itself as a type
4. We can therefore call a function over and over again
5. We also have a trait FnOnce that means whatever implements it can be invoked as a function that takes itself by value and it can have 0 or more arguments and it can return a type.

Now FnOnce is a trait right? Sure closures and named functions inherently
implement it, but shouldn't we be able to implement it for any type we want so
that we can call it as a function? We can on nightly Rust and therefore in
weird-exprs.rs we do.

```rust
fn function() {
    struct foo;
    impl FnOnce<()> for foo {
        type Output = foo;
        extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
            foo
        }
    }
    let foo = foo () ()() ()()() ()()()() ()()()()();
}
```

First we define a struct named foo inside of a function, which you can do! It
limits it to only being created inside the function. A neat trick if you need an
intermediate type not exposed in the API. We then impl FnOnce for foo by saying
it takes no args by using a tuple of size 0 and that it's return type for the
function call should be another type foo. We then we define the function body
where we return a new type foo after taking the old one by value with self and
dropping it at the end of the function body. We then call foo and much like
before where we kept calling example we just keep invoking foo as an FnOnce
function and eventually assign foo to the variable foo. This test is just making
sure that yes we can arbitrarily nest function calls one right after the other,
so long as a function returns a function that you can call, which yes you can do
so by just having a function return a function that impls Fn, FnOnce, or FnMut.
Now let's talk about loops and the never type.

```rust
loop {
    println!("stdout go brrrrrrrr");
}
```

With loops we have a loop keyword that will let us infinitely run the code
inside the block. We can end a loop by using  the break keyword.

```rust
loop {
    println!("stdout go brrrrrr");
    break;
}
```

Now here's a neat thing, loop is an expression and so it can go wherever you'd
want an expression. This means we can return a value from a loop like so:

```rust
let x = loop {
    break 5;
};

assert_eq!(x, 5);
```

Here we return a value five from the loop by calling break and assign it to x We
then assert it's equal to five! Now loops also have one other keyword, continue
which means "stop evaluating this loop and start from the beginning".

```rust
loop {
    println!("stdout go brrrrrr");
    continue;
    unreachable!("can't touch this");
}
```

Here we print out "stdout go brrrrrr" and then restart the loop again and never
hit the unreachable statement. Okay so we know a bit about loops, but what about
that never type I mentioned. never is an inbuilt primitive. Let's look at a
quick example.

```rust
let x: ! = {
    return 123
};
```

Here the exclamation point is how we represent the never type. Since we return
from the function early with the value 123 we can never assign it to x. never is
how we represent things that we can't construct or code we will never execute.
Some control flow statements are this never type. return is one, but so are
continue and break as they cause the code to stop where it is and jump to some
other place. Another interesting thing is that these keywords that are never can
type check to anything as you will never need the type for that part so for
example:

```rust
let x: String = return;
let x: i32 = return;
```

This compiles just fine as we exit the function early and can't assign any
value to x, no matter what type it is.

Let's recap then:
1. We have a loop keyword which loops infinitely
2. We can use break to exit a loop or exit a loop and return a value
3. We can use continue to start at the beginning of a loop
4. Some control flow words like break and continue are what's known as the never type
5. never type checks as any type

With this we're ready to look at our next test case.

```rust
fn angrydome() {
    loop { if break { } }
    let mut i = 0;
    loop { i += 1; if i == 1 { match (continue) { 1 => { }, _ => panic!("wat") } }
      break; }
}
```

For the sake of brevity I'm gonna just show you the control flow with this handy
diagram. I'm kidding, but the important part here is what this test is testing
for which is that break and continue can be used anywhere and type check, while
still letting the loop execute where it can. A bit nonsensical for control
flow, but absolutely necessary.

Let's talk about keywords. Rust has 3 types of keywords, reserved, strict, and
weak. Reserved key words are words that we might use in the future, but have no
purpose yet. They're reserved so that no one uses them in their code which could
cause it to break in later versions of the Rust compiler where they become used
for something. We also have strict key words which means these words cannot be
used as the name of items, variables, function parameters, fields, variants,
type parameters, lifetime parameters, loop labels, macros, attributes, macro
placeholders, or crates. Words like loop, return and fn fall into this
category. We also have weak keywords. These are only special in certain
contexts and so can be used in places you couldn't use strict keywords. union,
macro_rules!, and 'static are the weak keywords. dyn was also a weak keyword
in 2015 edition, but was promoted to a strict one in 2018. Now with all of
this in mind what about primitives in Rust like u8. Is it not a strict
keyword? My editor higlights u8 as well as other keywords so certainly it's a
special word. It's not. It's just an inbuilt type which means we need to test
that primitives can be used anywhere as a name you would not be able to for
strict keywords like so:

```rust
fn u8(u8: u8) {
    if u8 != 0u8 {
        assert_eq!(8u8, {
            macro_rules! u8 {
                (u8) => {
                    mod u8 {
                        pub fn u8<'u8: 'u8 + 'u8>(u8: &'u8 u8) -> &'u8 u8 {
                            "u8";
                            u8
                        }
                    }
                };
            }

            u8!(u8);
            let &u8: &u8 = u8::u8(&8u8);
            ::u8(0u8);
            u8
        });
    }
}
```

This function is a bit hard to parse, but let me break it down for you. This
function first checks that the argument input which is a u8 named u8 is not
equal to zero which we are specifying is a u8 explicitly. We then assert that a
value eight of type u8 is equal to the return value of an expression block which
defines a macro u8 that takes the literal token u8 to define a module u8 with a
function named u8 with a named lifetime 'u8, that must outlive the lifetimes 'u8
and 'u8, with an argument u8 that's a ref'u8 of a u8 that returns an &'u8 of
type u8. In that function it creates an &'static str with the value u8 and
returns the function argument u8. We then call the macro u8 with the argument
u8, create a variable named &u8, with type &u8, to assign it &8u8 as the value
after calling the function u8 from the module u8 and then we call the original
function u8 recursively with the value 0u8, which hits the if statement and
returns, and then return the value u8 from the block which in this case is eight
since we pass in that value in the test case meaning eight is equal to eight!
See not hard to understand, just hard to parse.

--------------------------------------------------------
## Conclusion

It's been a short yet dense journey that I've taken you on today. We've seen a
lot, maybe too much for mortal eyes. I sincerely apologize for showing you even
a fraction of the weird-exprs.rs test file, but I hope you see the necessary
evil that this file is in order to have the language we have today. I hope you
go forth to read the rest of the file and just write more weird code. It's fun
even if it isn't useful and you can learn a thing or two about Rust you didn't
even know was possible. Especially now that you know about Weird Expressions and
Where to Find Them. Thank you for your time.
