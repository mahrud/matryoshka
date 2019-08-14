# Matryoshka Rings

Small experiment in comparing how to define a type `ring` with another `ring` as its coefficient ring in Rust and C++. Sample result:
```
[mahrud@noether matryoshka]$ cargo run
   Compiling matryoshka v0.1.0 (/home/mahrud/Projects/rust/matryoshka)
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/matryoshka`
24: [5, 6]
24: Ring([5, 6])
24: Ring([5, 6])
24: Ring(Ring(Ring(Ring([5, 6]))))
[mahrud@noether matryoshka]$ g++ --std=gnu++17 -g src/main.cpp; ./a.out 
24: Monoid(56)
16: Ring(Monoid(56))
16: Ring(Monoid(56))
16: Ring(Ring(Ring(Ring(Monoid(56)))))
```
