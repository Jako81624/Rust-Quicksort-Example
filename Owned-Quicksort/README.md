# Owned (Alloc) Quicksort
This quicksort **doesn't** sort "in-place".  It splits vectors and recombines them recursively.  It's easier to read and understand, but it tests anywhere from 200-3000 times slower than the Rc<RefCell> version.
