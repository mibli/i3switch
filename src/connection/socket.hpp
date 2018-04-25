#pragma once

#include "../utils/logging.hpp"

#include <cstdint>
#include <string>
#include <vector>

class Socket
{
    int32_t fd {0};
    bool connected;
    logging::Logger log{};

public:
    Socket(std::string const &path);
    ~Socket() = default;

    bool write(std::vector<uint8_t> const &msg);
    std::vector<uint8_t> read(size_t size);
};
