#pragma once

#include "socket.hpp"
#include "i3binds.hpp"
#include "../utils/logging.hpp"

#include <thread>
#include <list>

namespace i3
{

class Client {
public:
    Client(std::string const &socket_path);
    Client(Client const &) = delete;
    ~Client() = default;

    std::string request(RequestType type, std::string const &payload);
    void subscribe(std::string const &payload) = delete;

private:
    std::string receive(ReturnType expected_type);

private:
    Socket socket;
    logging::Logger logger;
};

}
