// How to get my stack size limit in kilobytes
// ulimit -s
// 8192

// Most machines have 1K to 8K kb of stack memory. Thats 250K to 2million 32-bit integers.

#include <iostream>
#include <stack>

char* pointy() {
    char* p = new char[6];

    return p;
}

int main() {
    const int num_bytes = 8192*1024; 
    // const int num_bytes = 8192*1024; // This version Always triggers a seg fault memory error.

    // char chars[num_bytes]; // The max num_bytes I can use is not consistent on my machine. -- SEG FAULT HERE

    // char* chars  = NULL;         // Pointer initialized with null
    char* chars = new char[num_bytes];       // Request memory for the variable

    std::fill(chars, chars + num_bytes, '0'); // You dont actually need this line. But wanted to show they are filled.

    return 0;
}
