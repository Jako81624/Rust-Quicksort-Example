# Owned (Alloc) Quicksort
This quicksort **doesn't** sort "in-place".  It splits vectors and recombines them recursively.  It's easier to read and understand, but it tests around half the speed of the Rc<RefCell> version.
