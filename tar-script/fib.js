function fib(a, b, d) {
    // console.log(a);
    if (d < 10) {
        fib(b, a+b, d+1);
    }
}

let i = 0;
while (i < 10) {
    fib (1, 1, 0);
    i = i + 1;
}