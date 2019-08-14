use std::mem;

type Monoid = Vec<i32>;
// struct Monoid(Vec<i32>);

#[derive(Debug,Copy,Clone)]
struct Ring<T>(T);

trait Onion<T> {
    type Wrapper;
    fn pile(self) -> Self::Wrapper;
    fn peel(self) -> T;
}

impl<T> Onion<T> for Ring<T> {
    type Wrapper = Ring<Self>;
    fn pile(self) -> Ring::<Self> { Ring::<Self>(self) }
    fn peel(self) -> T { self.0 }
}

fn main() {
    let mut monoid = Monoid::new();
    monoid.push(5);
    monoid.push(6);

    println!("{}: {:?}", mem::size_of_val(&monoid), monoid);
    
    let ring0 = Ring(monoid);
    println!("{}: {:?}", mem::size_of_val(&ring0), ring0);

    let ring1 = ring0.pile().peel();
    println!("{}: {:?}", mem::size_of_val(&ring1), ring1);

    let ring2 = ring1.pile();
    let ring3 = ring2.peel();
    let ring4 = ring3.pile().pile().pile();
    println!("{}: {:?}", mem::size_of_val(&ring4), ring4);
}
