void symex_symbolic(void* ptr, int size);

int foo(int i) {
    if (i < 3) {
        return 1;
    } else {
        return 2;
    }
}

int main() {
    int i = 0;
    symex_symbolic(&i, sizeof(i));
    return foo(i);
}
