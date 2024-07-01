#include <iostream>
#include <cmath>
#include <string>
using namespace std;

int main() {
    // This is a comment
    cout << "Hello World!" << "\n\n";
    cout << "I am learning C++" << "\n\n";
    int myNum = 15;  // myNum is 15
    cout << "first: " << myNum << "\n";
    myNum = 10;  // Now myNum is 10
    cout << "now: " << myNum << "\n\n";  // Outputs 10
    float x;
    cout << "Type a number: "; // Type a number and press enter
    cin >> x; // Get user input from the keyboard
    cout << "\n" << "Your number is: " << x << "\n"; // Display the input value
    cout << "Your number rounded: " << round(x) << "\n";

    string firstName;
    cout << "Type your first name: ";
    cin >> firstName; // get user input from the keyboard
    cout << "Your name is: " << firstName << "\n";

    string emp;
    getline(cin, emp);
    return 0;
}
