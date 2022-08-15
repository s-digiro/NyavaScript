int object[3] = {1, 2, 3};
int common;
__thread int tls;

int func(void) {
    return 1;
}

int one(void) {
    return 1;
}

int two(void) {
    return 2;
}

int same(int x) {
    return x;
}

int add(int a, int b) {
    return a + b;
}

void ifunc() __attribute__ ((ifunc ("resolve_ifunc")));

void my_ifunc() {
    ((void)0);
}

void (*resolve_ifunc (void)) (void) {
    return my_ifunc;
}
