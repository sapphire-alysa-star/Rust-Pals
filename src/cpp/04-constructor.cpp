// C++ program to illustrate free()
// and delete keyword in C++

#include <iostream>
using namespace std;
 
// Class A
class A {
 
public:
    int val;

    // Constructor of class A
    A()
    {
        cout << "Constructor was Called!"
             << endl;
    }
 
    // Destructor of class A
    ~A()
    {
        cout << "Destructor was Called!"
             << endl;
    }
};

void stack_frame() {
    A my_a;

    my_a.val = 0;

    // both the constructor and de-structor will get called. Since my_a is stack allocated.

}
 
// Driver Code
int main()
{
 
    stack_frame();
    cout << endl;

    stack_frame();
    cout << endl;

    stack_frame();


    // Constructor was Called!
    // Destructor was Called!

    // Constructor was Called!
    // Destructor was Called!

    // Constructor was Called!
    // Destructor was Called!
 
    return 0;
}