# Rust and it's Orphan Rules

Recently at work I managed to hit the Orphan Rules implementing some things for
an internal crate. Orphan Rules you say? These are ancient rules passed down
from the [before times (pre 1.0)](http://smallcultfollowing.com/babysteps/blog/2015/01/14/little-orphan-impls)
that have to do with trait coherence. Mainly, if you and I both implement a
trait from another crate on the same type in another crate and we compile the
code, which implementation do we use? Coherence is about knowing exactly which
implementation of the code we use. Unfortunately this can be a bit strict at
times and was the issue I ran into that I want to talk about how it was solved.

I'm going to be intentionally vague here, but bear with me as the thing I want
to talk about is the actual solution that you might need dear reader. My work
code base is a chonky one, around 330,000 lines of actual code (which is all
pre-macro) according to tokei. We absolutely have to break this up into multiple
crates. There's just no other way to cache this effectively (fun times for CI)
and separate logical concerns. This means we use types in the main application
that come from these more foundational crates.

In this specific instance we have a crate that acts like a store of different
types. It's a HashMap<&'static str, HashMap<String, Box<dyn Any + Send + Sync>>
wrapped in a type that has methods to access it. A more redacted slimmed down
version is this:

```rust
pub struct Store {
  // The first key is the "Type" of the object, while the second key is
  // it's "Name". We can have multiple of the same "Type"
  inner: HashMap<&'static str, HashMap<String, Box<dyn Any + Send + Sync>>
}

impl Store {
  pub fn get<T>(name: &str) -> Option<&T>
    where T: Storeable
  {
    //.....
  }

  // Some other similar methods for retrieval
}
```

The key here is this Storeable trait that is pretty important it looks like:

```rust
pub trait Storeable {
  const TYPE: &'static str;
  fn cast(val: Value) -> Result<Box<(dyn Any + Send + Sync)>, Error> {
    let val = Self::try_from(val)?;
    Ok(Box::new(val))
  }
}
```

We essentially take some unformed blob of data and cast it and put it in the
store when we create it. How the try_from is implemented is not important, just
the fact that we cast the value to some Rust type. This is part of a whole crate
to just take things in and turn them into this Store that we can pull different
Rust types out of. As such we needed a way to pass data from the main
application across crate boundaries so we could know what data needed to be
casted from and to what types. This leads us to the Registry:

```rust
type RegMap =
  HashMap<&'static str, Box<dyn Fn(Value) -> Result<Box<(dyn Any + Send + Sync)>, Error>>>;

pub struct Registry {
  reg: RegMap
}

impl Registry {
  pub fn new() -> Self {
    Self {
      reg: HashMap::new(),
    }
  }

  pub fn register<T>(&mut self)
  where
    T: Storeable,
    Error: std::convert::From<<U as TryFrom<Value>>::Error>,
  {
    self.reg.entry(U::TYPE).or_insert(
      Box::new(U::cast) as Box<dyn Fn(Value) -> Result<Box<(dyn Any + Send + Sync)>, Error>>
    );
  }
  // The actual code that takes in values and runs it's casting functions on 
  // them that have been registered
}
```

Every thing you want to be converted needs to be registered! Now we used to
error if we had extra blobs of stuff and this signalled you needed to register
things, which was fine when everything was in one repo, but we split it out. So
you might run old code with brand new blobs causing it to fail since they
weren't tied together. We had to relax this error, but this means if someone
fails to register their type in the main application then we might not find out
for a while since all they'll get is a None from the Stores get function. This
meant we wanted to switch it to a compile time error instead! The solution?
More trait bounds!

```rust
// We created a trait in the crate
pub trait Registered {}

// We then created a little macro as the only way to register something
// hiding the documentation around the `register` function for the registry

macro_rules! register {
  ($reg:ident, $type:ty) => {
    impl cratename::Registered for $type {}
    $reg.register::<$type>();
  }
}
```

Then in the main code we replaced all calls to register the function with the
macro instead. Then we changed the function signature for the get method to `fn
get<T>(name: &str) -> Option<&T> where T: Storeable + Registered`. This means
that if you tried to pull out a type from the Store but forgot to register it
the compiler would fail since you didn't implement the trait for it. Perfect
right? Well all was right in the world until I hit E0210. Remember how I said
some of the types get defined in other subcrates. Well some of those types were
registered prior to the change. Implementing the Registered trait in the type's
crate also defeats the whole exercise here which is to enforce invocation of a
function and make it a compile time error otherwise. We hit the Orphan Rules!
Which inevitably to frustration trying to figure out ways to get around it.

Now normally the advice is make a local New Type and impl the trait on that. For
instance we could do:

```rust
impl Registered for Foo {}
struct Foo(ForeignType);
```

But that means we would have to create a New Type for every single foreign type
and then our calls to get would be the wrapped type not the actual type which is
confusing because you would ask for type Foo in the function and get a Bar
instead which is just bad API design in this case. Well then we could do
something like this then right?

```rust
// In the crate change the impl to
pub trait Registered<T> {}

// In our main code
impl Registered<LocalType> for ForeignType {} 

// And changing our function defs to
pub fn register<T, U>() 
  where T: Storeable + Registered<U>
{
  //....
}

pub fn get<T, U>(name: &str) -> Option<&T>
  where T: Storeable + Registered<U>
{
  //....
}
```

This works actually! Which is fine for the call to register but this means for
every single call to get your code looks like `store.get::<ActualType,
LocalType>("some name")` which is hardly intuitive either. We want the API to
remain the same because all the caller actually cares about is the type they
want out of the Store. We can do this with some other shennanigans though by
making the U apart of the types themselves. By changing the types to this:

```rust
use std::marker::PhantomData;

pub struct Register<U> {
  reg: RegMap
  _phantom: PhantomData<U>
}

pub struct Store<U> {
  inner: HashMap<&'static str, HashMap<String, Box<dyn Any + Send + Sync>>
  _phantom: PhantomData<U>
}
```

We have constrained the local type so to speak. First we would change our macro
to look like this:

```rust
macro_rules! register {
  ($reg:ident, $local:ty , $type:ty) => {
    impl cratename::Registered<$ty> for $type {}
    $reg.register::<$type>();
  }
}
```

We then would change our functions to look like:

```rust
pub fn register<T>() 
  where T: Storeable + Registered<U>
{
  //....
}

pub fn get<T>(name: &str) -> Option<&T>
  where T: Storeable + Registered<U>
{
  //....
}
```

And in our impl that `U` comes from the type itself e.g. `impl<U> Store<U>`. Now the
only places that need the local type to be typed out at all are function calls
the Store gets passed to which isn't the most ideal thing, but it declutters the
function calls which are arguably more important to understand.

## Conclusion

If you hit these rules yourself with similar constraints as ours, then I hope
this bit of knowledge will help you overcome that hump. More often than not the
New Type way should suffice, but in this case we needed something different. I'd
really like if the rules could be relaxed if the crates are all in the same
workspace and internal like they are at work, but I'm sure there are
difficulties on figuring out if crates in a workspace are up on crates.io let
alone another registry etc. etc. Nothing about designing a language is easy and
there are a lot of reasons these rules do exist and are really constrained. Just
like how NLL helped relax what the borrow checker would accept in terms of valid
programs, I'm hoping future versions of the compiler might ease up a bit more on
the rules. Either way there was at least a solution, even if it wasn't my most
favorite one, but I got to learn a lot in the process and hopefully you did too!
