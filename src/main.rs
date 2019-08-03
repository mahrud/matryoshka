#[derive(Debug,Copy,Clone)]
struct Matryoshka<T> {
    x: T,
}

trait Onion<T> {
    fn pile(self) -> Matryoshka::<Matryoshka::<T>>;
    fn peel(self) -> T;
}

impl<T> Onion<T> for Matryoshka::<T> {
    fn pile(self) -> Matryoshka::<Matryoshka::<T>> { Matryoshka::<Matryoshka::<T>> { x: self } }
    fn peel(self) -> T { self.x }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("hmm");
    let count: u32 = input.trim().parse().unwrap();
    
    let a = Matryoshka::<u32> { x: count };
    println!("{:?}", a);

    let b = a.pile();
    println!("{:?}", b);

    println!("{:?}", b.pile().pile().pile().peel().peel().peel());
    println!("{:?}", b.peel().pile());
}
