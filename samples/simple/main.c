int foo(const int* d, int i) {
    if (d[i] == 5) {
        return d[i] + 5;
    } else {
        return d[i+2] + 3;
    }
}

int main() {
    int a = 25;
    int b = 7;
    const int data[5] = {0,1,2,3,4};

    int c = foo(data, 2);
    int d;
    switch (a) {
        case 3: break;
        case 4: break;
        case 5: a = 7; break;
    }

    return 0;
}
