#pragma once

#include <cstring>
#include <memory>

template<typename ...Args>
char *strprintf(char const *format, Args...args)
{
    size_t const length = snprintf(nullptr, 0, format, args...);
    char *buffer = reinterpret_cast<char *>(malloc(length));
    sprintf(buffer, format, args...);
    return buffer;
}

