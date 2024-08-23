// spec1_destructors.cpp
#include <cstring> // strlen()
#include <iostream>
using namespace std;

class String
{
    public:
        char* _text {nullptr};

        String(const char* ch)
        {
            size_t sizeOfText = strlen(ch) + 1; // +1 to account for trailing NULL

            // Dynamically allocate the correct amount of memory.
            _text = new char[sizeOfText];

            // If the allocation succeeds, copy the initialization string.
            if (_text)
            {
                strcpy(_text, ch);
            }
        }

        ~String()
        {
            // Deallocate the memory that was previously reserved for the string.
            delete[] _text;
        }

        void print() {
            // so we can check things actually work
            cout << this->_text << endl;
        }
};

int main()
{
    String str("We love C++");
    str.print();
}

// This is RAII
// You use new and delete exclusively in the constructor and destrucotr of classes.
// This means new and delete are used correctly when stuff is on/off the stack. 