// Simple pointers and memory tutorial.

// Advice for the children to run this repo on linux (or blessed WSL):

// 1 - Install compilers for C and C++
// sudo apt-get install gcc 
// sudo apt-get install g++ 

// 2 - Compile code
// gcc hello.c -> creates a.out
// g++ hello.cpp -> creates a.out by default
// note: the a stands for assembled. Its an executable file you can run natively on your OS.

// 3 - Run the code
// ./a.out

// 4 - You can also name the file. Example
// g++ -o hello hello.cpp
// ./hello

#include <iostream>

int main() {
    std::cout << "Hello Celene!\n";

    int marks[10] = {50, 55, 67, 73, 45, 21, 39, 70, 49, 51};
    int i, sum = 0;
    double avg;

    for (i = 0; i <= 9; i++){
        sum += marks[i];
    }

    avg = (double)sum / 10;
    // printf("Average: %f\n", avg); C method. Ignore.
    std::cout << "Average: " << avg << "\n";

    // Hello Celene!
    // Average: 52.000000

    return 0;    
}