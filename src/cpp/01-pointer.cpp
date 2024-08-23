#include <iostream>
using namespace std;

int main() {
    int x; 
    int *p; // note its genuinely int *p not int* p;

    x = 5;
    p = &x; // p points to x

    cout << "variable x: " << x << endl;

    cout << "address of x: " << p << endl;  

    return 0;
}

// printf formatters: 
// %p - pointer, %d/%i - int, %s - string, %u - unsigned, %f - float, %c - character