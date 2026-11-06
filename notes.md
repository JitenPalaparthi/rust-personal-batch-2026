# Memory Safety

- Null Pointer Dereference -> The pointer does not contain any address yet the program tries to dereference that pointer
- Double Free a Null Pointer -> Try to delete a memory which has not address
- Use After Free -> Try to use the pointer, after it is free
- Dangling Pointer -> Pointers are returned out of their scope and try to be accessed
- Memory Leak -> Memory is allocated on heap, but forget to deallocate, C,C++ are very notorious for this kind of issues. Usually GC handles this.
- Data Races -> Occurs while writing concurrency and multi threading
