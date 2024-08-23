

// C++ Program to implement unique_ptr 
#include <iostream> 
#include <memory> 
using namespace std; 
  
class A { 
    public:
    void printA() { cout << "A struct...." << endl; } 
}; 

int main() 
{ 
    unique_ptr<A> p1;
    p1 = std::make_unique<A>(); // Box::new = make_unique<T>

    p1->printA(); 
    
    // displays address of the containing pointer 
    cout << p1.get() << endl; 
  
    // now address stored in p1 shpould get copied to p2 
    unique_ptr<A> p2 = move(p1); 
  
    p2->printA(); 
    cout << p1.get() << endl; // prints 0. dead pointer.
    cout << p2.get() << endl; // prints same address as before.
    return 0; 
}