#include <stdio.h>
#include <sys/time.h>
#include <stdlib.h>
#include <assert.h>

long fibo_rec(int n) {
    if (n < 3) {
        return 1;
    }
    return fibo_rec(n - 1) + fibo_rec(n - 2);
}

long fibo_loop(int n) {
    if (n < 3) {
        return 1;
    }
    long curr = 1;
    long prev = 1;
    long temp;
    for(size_t i = 3; i <= n; ++i){
        temp = curr;
        curr = curr + prev;
        prev = temp;
    }
    return curr;
}

int main(int argc, char* argv[]) {
    if (argc < 1) {
        printf("Provide n'th fibonacci number to calculate");
        return 1;
    }
    int n = atoi(argv[1]);
    struct timeval start, end;
    
    gettimeofday(&start, NULL);
    long f_num_rec = fibo_rec(n);
    gettimeofday(&end, NULL);
    printf("Recursive: %ld : %d micro-secs\n", f_num_rec, end.tv_usec - start.tv_usec);
    
    gettimeofday(&start, NULL);
    long f_num_loop = fibo_loop(n);
    gettimeofday(&end, NULL);
    printf("Loop: %ld : %d micro-secs\n", f_num_loop, end.tv_usec - start.tv_usec);
    
    assert (f_num_rec == f_num_loop);
    return 0;
}

