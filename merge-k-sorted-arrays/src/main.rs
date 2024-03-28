use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::cmp::Ordering;
// the problem
// we are given `k` array of items in a sorted order
// we are supposed to impl a function that give a `k` sorted arrays, merge them
// and return a sorted array
// e.g `a = [6, 8, 9]`
// `b = [2, 4, 7]` where `k = 2`
// to be `c = [2, 4, 6, 7, 8, 9]`

fn main() {

    // works but it does have `0(n*k * log n*k)`
    // worse time complexity
    let a = vec![6, 8, 9];
    let b = vec![2, 4, 7];

    let mut c = [a, b].concat();
    c.sort();
    // println!("{:#?}", c);

    let a1 = vec![1, 2, 5];
    let a2 = vec![-7, 3, 4, 7];
    let a3 = vec![6, 8, 9];
    dbg!(merge([a1, a3, a2].to_vec()));
}


// A struct that will hold references to the k arrays and their respective index as their iteration state. 
// The index will let us know where in our array we are currently at.
// the instance of item will be inserted in heap
#[derive(Debug, Eq)]
struct Item<'a> {
    arr: &'a Vec<i32>,
    idx: usize
}
// Item does not know how they should be compared
// here we need to impl a few traits and defer the comparision to the elements of the array
//  (PartialEq, PartialOrd, Ord)
impl<'a> PartialEq for Item<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.get_item() == other.get_item()
    }
}

impl<'a> PartialOrd for Item<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_item().partial_cmp(&other.get_item())
    }
}

impl<'a> Ord for Item<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_item().cmp(&other.get_item())
    }    
}

impl<'a> Item<'a> {
    fn new(arr: &'a Vec<i32>, idx: usize) -> Self {
        Self {
            arr,
            idx
        }
    }

    fn get_item(&self) -> i32 {
        self.arr[self.idx]
    }
}

// this method takes in an array of array of singned integers

fn merge(arrays: Vec<Vec<i32>>) -> Vec<i32> {
    // a Vec that will hold our marged item
    let mut sorted = vec![];

    // We use a BinaryHeap instance, we can insert the first k items into the array. 
    // Then we keep popping until our heap is empty and keeping the array indexes.
    let mut heap = BinaryHeap::with_capacity(arrays.len());
    for arr in &arrays {
        let item = Item::new(arr, 0);
        heap.push(Reverse(item));
    }

    // while the heap is not empty.. get the smallest item by heap.pop() and push it 
    // to Vec and increment the item's index by 1. 
    // we still might have items in the heap, we check for that and push to vec at,,,, the last line.
    while !heap.is_empty() {
        let mut it = heap.pop().unwrap();
        sorted.push(it.0.get_item());
        it.0.idx += 1;
        if it.0.idx < it.0.arr.len() {
            heap.push(it)
        }
    }

    sorted
}
// solution our complexity now reduces to `O(n*k * Log(k))
