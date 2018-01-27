#ifndef __socket_hpp__
#define __socket_hpp__

#include "logging.hpp"

#include <cstdint>
#include <string>

class Socket
{
    int32_t fd {0};
    bool connected;
    logging::Logger log{};

public:
    Socket(std::string const &path);
    ~Socket() = default;

    bool write(std::string const &msg);
    std::string read(size_t size);
};

#endif//__socket_hpp__
