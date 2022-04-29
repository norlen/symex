int test(int high) {
    for (int i = 0; i < high; ++i) {
        if (i == 4) {
            return 1;
        }
    }
    return 0;
}


int main() {
    return test(5);
}
