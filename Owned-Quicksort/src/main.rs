use std::time::Instant;

fn main() {
    // Create a vector of randomly ordered integers
    let t_vec = vec![2,12,7,9,8,44,10,3,56,3,1,5,87,5,19];
    let cloned_vec = t_vec.clone();
    // Call quicksort on a cloned copy of the vector (such that the ownership of the original vector is not lost)
    let exec_time = Instant::now();
    let s_vec = quicksort(t_vec);
    let time_elapsed = exec_time.elapsed();

    // Print the original and sorted vectors
    println!("{cloned_vec:?}");
    println!("{s_vec:?}");
    println!("Time elapsed: {:?}", time_elapsed);
}

fn quicksort(mut data: Vec<usize>) -> Vec<usize> {
    // Recursive calls will inevitably result in a zero-length vector.  A length of 1 clearly doesn't require sorting, so
    // we exit prematurely on anything less than that
    if data.len() <= 1 { return data }
    //println!("Called quicksort");

    // Retrieve the index at which all items below are lesser than, and all items above are greater than
    let partition_idx = partition(&mut data);

    // `split_off(index)` creates a new vector containing all the items above the partition index
    let r_vec = data.split_off(partition_idx);
    // Recursively call quicksort on the right vector and assign it to 'right return vector'
    let mut rr_vec = quicksort(r_vec);

    // Declare lr_vec, leave unassigned
    let mut lr_vec;
    // If the left vector contains two items or fewer, we know that the last value is the index and
    // any items below it are smaller; thus we needn't sort it any further
    if data.len() <= 2 {
        lr_vec = data;
    // However, any vector with more than two items requires sorting
    } else {
        // The last item is guaranteed to be the largest, so pop it off the vector to avoid
        // sorting it unnecessarily
        let pivot = data.pop().unwrap();
        // Recursively quicksort the remaining values
        let mut lri_vec = quicksort(data);
        // Push the pivot back on to the vector as the known largest value
        lri_vec.push(pivot);
        // Assign the sorted vector to 'left return vector'
        lr_vec = lri_vec;
    }

    // Append the right return vector to the left return vector
    lr_vec.append(&mut rr_vec);

    lr_vec
}

fn partition(data: &mut Vec<usize>) -> usize {
    let mut index = 0;
    // Use the last item as the pivot, and remove one since vectors are indexed from 0
    let pivot = data.len() - 1;

    // For all items in the vector
    for i in 0..data.len() {
        // If the current item is smaller than the pivot
        if data[i].lt(&data[pivot]) {
            // Swap the current item with the item at the index and increment the index
            data.swap(index, i);
            index += 1
        }
    }
    // Since index lands on a value greater than or equal to the pivot, swap it with the pivot
    data.swap(index, pivot);
    // Increment the index ONLY because when we call `split_off()` in quicksort(), we want to exclude the pivot
    index += 1;
    index
}