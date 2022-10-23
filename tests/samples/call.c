void symex_symbolic(void* ptr, int size);

int foo(int a) {
    if (a < 0) {
        return 10;
    } else {
        return 1;
    }
}

int main() {
    int a = 0;
    symex_symbolic(&a, sizeof(a));
    return foo(a);
}
