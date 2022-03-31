#include <stdexcept>

int ithrow() {
    throw "throwing a message";
}

const char* foo() {
    try {
        ithrow();
    } catch (const char* msg) {
        return msg;
    }
    return nullptr;
}

int main() {
    foo();
}