use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    // Create two basic vectors
    let unsorted_array = Vec::from([9,2,20,15,65,32,11,100,43,5,2,18]);
    let unsorted_array_two = Vec::from([92,20,18,65,32,51,100,43,5,4,18]);

    /**
        Why the HashMaps?  Well the concept for plain Vectors is the same - wrapping them in
        Rc<RefCell<Vec>> (or their Arc/Mutex multithreaded equivalents) is necessary regardless.
        It just so happened that, at the time, I was working on a project that was using HashMaps
        and I left it for posterity
    */

    let test_hash = HashMap::from([
        ("Test", Rc::new(RefCell::new(unsorted_array))),
        ("Test2", Rc::new(RefCell::new(unsorted_array_two)))
    ]);

    /**
        Why do we need Rc and RefCell?  Simply put, the QuickSort algorithm requires TWO distinct
        recursions in each operation - as we know, Rust won't let you have multiple ownership or
        multiple mutable references in a normal environment.  To alleviate this - Rc creates a smart
        pointer.  This pointer, however, is immutable (since, according to Rust's borrow checker,
        we can't have multiple mutable borrows).  (It is important to remember - cloning an Rc will
        yield a smart pointer to the SAME memory location.)  To remedy this, we place a RefCell inside it.
        RefCell's are special because they allow for INTERIOR MUTABILITY (the Rust documentation has
        an entire section on this topic).  They are exempt from the static borrow checker at compile
        time, and instead the borrow checking is done at runtime.  This is slower, but more flexible
        and, in a lot of cases, necessary.  The RefCell itself is essentially another type of smart pointer,
        except this one performs borrow checking at runtime.

        Why then, do we need to wrap a RefCell inside an Rc?  Surely the RefCell is enough?  This
        might trip up new Rustaceans at first - but it's quite simple.  The RefCell points to a section
        of mutable memory.  It is solely responsible for making sure that only one mutable reference
        to that memory exists at any one time.  If you were to clone it and create a new RefCell
        that pointed to that same memory location, neither would know of the other's existence, and you could
        end up with undefined behaviour.  (See: https://manishearth.github.io/blog/2015/05/17/the-problem-with-shared-mutability/
        for a good explanation as to why this is required in a single-threaded environment.  TL;DR:
        most of the time it doesn't matter, but it's also something you don't want to fuck up).  By
        wrapping it in an Rc, we can access the same RefCell each time and simply clone the Rc to
        keep the borrow checker happy.  TL;DR: Cloning Rc creates a new IMMUTABLE reference to the
        underlying memory, cloning RefCell clones the underlying data and creates a new object.

        How do we access the data in a RefCell?  Easy - if you only want an immutable reference, you
        call the `borrow()` method.  If you want an immutable reference, you call `borrow_mut()`.
        If you want to retake ownership and empty the RefCell, you call `take()`.
    */

    // Create a new HashMap to contain the sorted data without being cluttered by RefCells
    let mut final_hash = HashMap::new();

    // For every key (i) and value (j) in the HashMap
    for (i, j) in test_hash {
        // Get the length before wrapping it up
        let length = j.borrow().len() - 1;
        // Start at timer to measure the execution time
        let exec_time = Instant::now();
        // Call `quicksort()` on a clone of `j` (creating a fresh copy of the Rc pointer)
        quicksort(j.clone(), 0, length);
        // Print the execution time for kicks
        println!("Execution Time: {:?}", exec_time.elapsed());
        // Take the now sorted Vector out of the HashMap, and insert it into our clean map
        final_hash.insert(i, j.take());
    }

    // Print the map
    println!("{:?}", final_hash);
}

/**
    Programmer's note: I refer to a "virtual slice" here.  Instead of actually calling quicksort on a
    specific slice of the Vector each time (which might be possible with enough trickery, but would
    ultimately be pointless because you'd have to nest RefCells - and I've never tested it, so I don't
    know if it's even possible) or calling `split()` by index and reassembling the vector each and every time
    (which would involve much more memory allocation and returning data recursively - something you
    could do), I chose to go for the old-school manipulate-in-place algorithm, which is arguably
    what quicksort is all about.  When I refer to a virtual slice then, I am referring to the section
    (slice) of the vector I am accessing by range in each loop.
*/

fn quicksort(array: Rc<RefCell<Vec<u8>>>, low: usize, high: usize) {
    // If the low index is less than the high index (i.e. not out of bounds and our "virtual slice" contains at least one item)
    if low < high {
        // Call partition
        let pi = partition(array.clone(), low, high);

        // In a situation where `pi` is 0, we don't want to attempt to go negative on a usize.  This
        // could happen if the value at the pivot was smaller than all values in the array, and low
        // was equal to 0.  Why usize? Rust only indexes vectors with usize's without extra
        // implementation.  I'm sure someone could work out the tradeoff between running an extra CMP or JZ
        // instruction on each loop vs implementing indexing with signed integers, but this is easier
        if pi > 0 {
            quicksort(array.clone(), low, pi - 1)
        }
        // Call quicksort on all the elements above the partition index.  Again, cloning the Rc pointer
        quicksort(array.clone(), pi + 1, high);
    }
}

fn partition(array: Rc<RefCell<Vec<u8>>>, low: usize, high: usize) -> usize {
    // Index is going to track the last entry that was smaller than our pivot (in this case, the
    // final element in the array)
    let mut index = low;

    // Borrow a mutable reference from our RefCell!
    let mut l_arr = array.borrow_mut();

    // For all the numbers between low and high, including high
    for i in low..=high {
        // If the number stored at array[i] is smaller than our pivot (array[high])
        if l_arr[i].lt(&l_arr[high]) {
            // Swap the item located at our current iteration with that stored at the position last
            // referenced in index
            l_arr.swap(index, i);
            // Increment index
            index += 1;
            // This ensures that every element smaller than our pivot is stored sequentially
        }
    }

    // Swap the value in our pivot index with the last position in our array that contains a value
    // smaller than itself (notice how index is incremented after every swap in the loop, so it always
    // ends up pointing to a value that is greater than or equal to our pivot)
    l_arr.swap(index, high);

    // Return our index - aka the position at which all values to the left will be smaller than it,
    // and all values to the right larger.  If you do this recursively until the vector has been run through
    // quicksort in smaller and smaller slices, the array will be sorted from low to high.
    index
}
