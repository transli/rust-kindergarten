use std::{thread::sleep, time::Duration};

const CREAR: &str = "\x1B[2J\x1B[1;1H";


struct Progress<Iter> {
    iter: Iter,
    i: usize,
    bound: Option<usize>
}

impl<Iter> Progress<Iter> {
    pub fn new(iter: Iter) -> Self {
        Progress{iter, i: 0, bound: None}
    }
}

impl<Iter> Progress<Iter>
    where Iter: ExactSizeIterator
{
    pub fn with_bound(mut self) -> Self {
        self.bound = Some(self.iter.len());
        self
    }
}
impl<Iter> Iterator for Progress<Iter> 
where Iter: Iterator {
    type Item = Iter::Item;
    fn next(&mut self) -> Option<Self::Item> {
        println!("{}", CREAR);
        match self.bound {
            Some(bound) =>
                println!("[{}{}]",
                "*".repeat(self.i),
                " ".repeat(bound - self.i),
            ),
            None => 
                println!("{}", "*".repeat(self.i)),
               
        };
        self.i += 1;
        self.iter.next()
    }
}

trait ProgessIterExt: Sized {
    fn progr_ess(self) -> Progress<Self>;
}

impl<Iter> ProgessIterExt for Iter 
    where Iter: Iterator
{
    fn progr_ess(self) -> Progress<Self> {
        Progress::new(self)
    }
}

fn expensive_cal(_n: &i32) {
    sleep(Duration::from_secs(1));
}

fn main() {
    let v = vec![1, 2, 3, 4];
    // let y = 1.progr_ess();
    // for n in (0 ..).progr_ess() {
    //     expensive_cal(&n);
    // }
    for n in v.iter().progr_ess().with_bound() {
        expensive_cal(n)
    }
    // for n in v.iter().progr_ess() {
    //     expensive_cal(n);
    // }
}
//  
