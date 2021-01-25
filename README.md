# Rats
Rats is a library that provides abstractions for functional programming in Rust. 

The name is a play on the Scala library [Cats](https://typelevel.org/cats/), which the design of this library is 
inspired. The goal is to have a similar ecosystem to Cats, so hopefully we will eventually provide something similar 
to [cats-effect](https://typelevel.org/cats-effect/)

Currently the library depends on the nightly compiler for support of Generalized Associated Types.
[See this rust issue for more details](https://github.com/rust-lang/rust/issues/44265)

The initial design of the categories is based on [this blog post](https://www.fpcomplete.com/blog/monads-gats-nightly-rust/).
