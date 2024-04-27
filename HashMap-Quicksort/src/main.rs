use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let unsorted_array = Vec::from([9,2,20,15,65,32,11,100,43,5,2,18]);
    let unsorted_array_two = Vec::from([92,20,18,65,32,51,100,43,5,4,18]);

    let test_hash = HashMap::from([
        ("Test", Rc::new(RefCell::new(unsorted_array))),
        ("Test2", Rc::new(RefCell::new(unsorted_array_two)))
    ]);

    let mut final_hash = HashMap::new();

    for (i, j) in &test_hash {
        let length = j.borrow().len() - 1;
        let exec_time = Instant::now();
        quicksort(j.clone(), 0, length);
        println!("Execution Time: {:?}", exec_time.elapsed());
        final_hash.insert(i, j.take());
    }

    println!("{:?}", final_hash);
}

fn quicksort(array: Rc<RefCell<Vec<u8>>>, low: usize, high: usize) {
    if low < high {
        let pi = partition(array.clone(), low, high);

        if pi > low {
            quicksort(array.clone(), low, pi - 1)
        }

        quicksort(array.clone(), pi + 1, high);
    }
}

fn partition(array: Rc<RefCell<Vec<u8>>>, low: usize, high: usize) -> usize {
    let mut index = low;

    let mut l_arr = array.borrow_mut();

    for i in low..=high {
        if l_arr[i].lt(&l_arr[high]) {
            l_arr.swap(index, i);
            index += 1;
        }
    }

    l_arr.swap(index, high);

    index
}
