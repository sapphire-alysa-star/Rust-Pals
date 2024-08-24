struct MyString {
    length: usize,
    bytes: [u8]
}


fn main() {

}



// C++ Code:

// class String
// {
//     public:
//         char* _text {nullptr};

//         String(const char* ch)
//         {
//             size_t sizeOfText = strlen(ch) + 1; // +1 to account for trailing NULL

//             // Dynamically allocate the correct amount of memory.
//             _text = new char[sizeOfText];

//             // If the allocation succeeds, copy the initialization string.
//             if (_text)
//             {
//                 strcpy(_text, ch);
//             }
//         }

//         ~String()
//         {
//             // Deallocate the memory that was previously reserved for the string.
//             delete[] _text;
//         }

//         void print() {
//             // so we can check things actually work
//             cout << this->_text << endl;
//         }
// };





