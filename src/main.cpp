// os checks
#ifndef __linux__
    #error "only linux supported"
#endif

#include <unistd.h>
#include <errno.h>

#include <filesystem>
#include <cstring>

using std::filesystem::directory_iterator;

static char* XTND_PREFIX = "/.xtnd";

static void init(void) __attribute__((constructor));

static void wrerr(const char *p) {
    const char *q;
    int saved_errno;

    if (!p) return;

    q = p;
    while (*q) q++;

    if (q == p) return;

    saved_errno = errno;

    while (p < q) {
        ssize_t n = write(STDERR_FILENO, p, (size_t)(q - p));
        if (n > 0) p += n; else if (n != (ssize_t)-1 || errno != EINTR) break;
    }
    errno = saved_errno;
}

static void init(void) {
    wrerr("Loading XTND...\n");

    wrerr("Discovering plugins...\n");
    const char* home_dir = getenv("HOME");
    char* prvw_dir;
    prvw_dir = (char*)malloc(strlen(home_dir)+1+4);
    strcpy(prvw_dir, home_dir);
    strcat(prvw_dir, XTND_PREFIX);
    for (const auto &file : directory_iterator(prvw_dir)) {
        wrerr("Found ");
        wrerr(file.path().c_str());
        wrerr("\n");
    }

    wrerr("Initializing plugins...\n");
    // TODO actually make this

    wrerr("Hooking...\n");
    // TODO actually make this

    wrerr("XTND loaded!\n");
}