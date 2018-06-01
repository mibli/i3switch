#pragma once

#include <cstring>
#include <memory>

template<typename ...Args>
char *cstringf(char const *format, Args...args)
{
    size_t const length = snprintf(nullptr, 0, format, args...);
    char *buffer = reinterpret_cast<char *>(malloc(length));
    sprintf(buffer, format, args...);
    return buffer;
}

template<typename ...Args>
std::string stringf(char const *format, Args...args)
{
    size_t const length = snprintf(nullptr, 0, format, args...);
    std::string buffer;
    buffer.reserve(length);
    sprintf(buffer.data(), format, args...);
    return buffer;
}

