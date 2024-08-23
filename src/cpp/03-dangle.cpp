// spec1_destructors.cpp
#include <cstring> // strlen()
#include <iostream>
using namespace std;

int* dangle() {
    int x = 5;
    int* p;

    p = &x;

    cout << "variable x has address: " << p << endl;

    return p;
}


int main() {

    int* p = dangle();

    cout << "dangling pointer address: " << p << endl;
    cout << "dereference p to get: " << *p << endl;

    // What will happen is undefined. On my first run I got:
    // variable x has address: 0x7ffff48d2dcc
    // dangling pointer address: 0x7ffff48d2dcc
    // dereference p to get: 32619


    return 0;
}


// some notes on variable addresses and function calls:

// This is normal, especially if you call the function from the same caller many times, e.g.

// read_set("foo");
// read_set("bar");
// read_set("baz");
// or

// for (i = 0; i < 5; ++i) {
//   read_set(filenames[i]);
// }
// The stack contains the return address, the frame pointer, the local variables, the parameter values and the temporary values of each active function call (starting from main). It's quite common that all these have a fixed size for each function, so if function A() needs 40 bytes on the stack, function B() needs 20 bytes and function C() needs 1000 bytes, then in the following two situations the local variables (e.g. &ch) of read_set are usually on the same address:

// main() calls C(), C() calls B(), B() calls B() again, the inner B() calls read_set().

// main() calls C(), C() calls A(), A() calls read_set().

// Please note that depending on the compiler and on the architecture you may get different addresses. But getting the same address is not unusual.
