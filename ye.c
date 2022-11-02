#define _GNU_SOURCE
#define __need_IOV_MAX
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <fcntl.h>
#include <limits.h>

#define LEN 2
#define TOTAL (1024 * 1024)
#define IOVECS IOV_MAX

int main() {
    unsigned pipesize = 16 * 1024 * 1024;
    if (pipesize != fcntl(1, F_SETPIPE_SZ, pipesize))
        perror("Warning: could not set pipe size. Maybe you're not piping? | fcntl F_SETPIPE_SZ"), exit(1);

    char yes[LEN] = {'y', '\n'};
    char* buf = malloc(TOTAL);
    for (int bufused = 0; bufused < TOTAL; bufused += LEN)
        memcpy(buf + bufused, yes, LEN);

    struct iovec iov[IOVECS];
    for (int i = 0; i < IOVECS; i++) {
        iov[i].iov_base = buf;
        iov[i].iov_len = TOTAL;
    }

    while(vmsplice(1, iov, IOVECS, SPLICE_F_GIFT));

    return 1;
}
