## Note
The program should be usable but there's still a lot to improve.

# side-channel-crackme-solver
An automatic multithreaded solver for easy crackmes using various side-channel
attacks. It's not anything serious but may be able to solve easy crackmes in
highschool level capture the flag competitions like in the example below. The
idea is based on [a blog post by Gynvael
Coldwind](https://gynvael.coldwind.pl/?lang=en&id=763) and Julien Voisin's
article "Crackme Solving for the Lazies" from of the magazine "Paged Out! #1".

## Dependencies
The program uses `perf` underneath, so make sure you have it installed and working.

## Usage
### Example Crackme
```c
// Simple program that I use for testing.
// Compile with `gcc test.c`
#include <stdio.h>
#include <unistd.h>
#include <string.h>

int main() {
    char flag[] = "flag{PLEASEWORK}";
    char *line = NULL;
    size_t len;
    size_t ret = getline(&line, &len, stdin);
    line[strcspn(line, "\n")] = '\0';

    if (ret != sizeof flag) {
        return -1;
    }

    // Simulate a bunch of instruction decrypting the flag/character
    for (int i = 0; i < 3000; ++i)
        ;

    for (size_t i = 0; i < strlen(flag); ++i) {
        if (line[i] == flag[i]) {
            // Simulate a bunch of instruction decrypting the flag/character
            for (int j = 0; j < 3000; ++j) 
                ;
        } else {
            return -2;
        }
    }

    puts("correct flag!");
    return 0;
}
```

To run the program on this crackme run:
```
cargo run -- --input-end $'\n' ./a.out
```
