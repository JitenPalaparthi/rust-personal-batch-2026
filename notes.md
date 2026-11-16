# Memory Safety

- Null Pointer Dereference -> The pointer does not contain any address yet the program tries to dereference that pointer
- Double Free a Null Pointer -> Try to delete a memory which has not address
- Use After Free -> Try to use the pointer, after it is free
- Dangling Pointer -> Pointers are returned out of their scope and try to be accessed
- Memory Leak -> Memory is allocated on heap, but forget to deallocate, C,C++ are very notorious for this kind of issues. Usually GC handles this.
- Data Races -> Occurs while writing concurrency and multi threading

------------------------

Compile Time -->

Text and Data Segment

Runtime -->

Stack and Heap

-----------------------
 - In Python all data is stored in heap and their address are stored in stack
 - In JavaScript, engines choose how to represent the values. Most probably on Heap and addresses on Stack
 - In Java/C# basic/primitive variables are stored in stack and objects are stored in heap
 - In C, variables are stored in stack and memory can be allocared on heap using malloc
 - In C++, new function/keyword allocates on heap, normal variables are allocated on stack
 - in Go, data is stored in heap or stack is decided by go's escape analaysis during compilation (New approach)
 - Rust, basic data is stored on stack, some kind of data(some variables) are stored on heap. Developer can decide to store the data either on stack or on heap
