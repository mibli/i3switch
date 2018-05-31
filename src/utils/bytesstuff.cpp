#include "bytesstuff.hpp"

#include <iostream>

void bytedump(std::vector<uint8_t> const &bytes)
{
    for (int i = 0; i < 8; i++)
    {
        printf("  %01d ", i);
    }
    printf("\n");

    int i = 1;
    for (uint8_t byte: bytes)
    {
        printf("%03hhd ", byte);
        if (i++ % 8 == 0)
            printf("\n");
    }
    printf("\n");
    fflush(stdout);
}

