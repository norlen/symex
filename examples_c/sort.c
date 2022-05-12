void assume(int condition);
void symbolic(void* ptr, int size);

// Advanced sorting algorithm.
void sort(int arr[], int size) {
    for (int i = 0; i < size-1; ++i) {
        for (int j = i+1; j < size; ++j) {
            if (arr[i] > arr[j]) {
                int tmp = arr[i];
                arr[i] = arr[j];
                arr[j] = tmp;
            }
        }
    }
}

// Check that it works for any number.
void check() {
    int arr[10]; // Can contain anything.
    int n = 10;
    for (int i = 0; i < n; ++i) {
        symbolic(&arr[i], 4);
    }

    sort(&arr[0], n);
    for (int i = 1; i < n; ++i) {
        // Ensure the result is sorted.
        assume(arr[i-1] <= arr[i]);
    }
}

int main() {
    int arr[10] = {753, 532, 864, 349, 1425, 34, 65000, 64, 1034, 985};
    
    // Sorted: {34, 64, 349, 532, 753, 864, 985, 1034, 1425, 65000}
    sort(&arr[0], 10);

    for (int i = 1; i < 10; ++i) {
        // Ensure the result is sorted.
        assume(arr[i-1] <= arr[i]);
    }
    return arr[1]; // Should return 64 = 0x40
}
