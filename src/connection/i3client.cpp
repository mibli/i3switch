#include "i3client.hpp"
#include <iostream>

namespace i3
{

std::vector<uint8_t> pack(RequestType type, std::string const &payload)
{
    Header header(type, payload.length());
    std::vector<uint8_t> bytes (sizeof(i3_ipc_header) + header.size);
    auto const header_ptr = reinterpret_cast<uint8_t *>(&header);
    std::copy(header_ptr, header_ptr + sizeof(i3_ipc_header), bytes.begin());
    std::copy(payload.begin(), payload.end(), bytes.end());
    return bytes;
}

Client::Client(std::string const &socket_path)
    : socket(socket_path)
{
    logger.configure("%s:%s()  ", __FILENAME__, __func__);
    logger.info("Connected to socket: %s", socket_path.c_str());
}

std::string Client::request(RequestType type, std::string const &payload)
{
    //1) start receiver thread
    //2) send request
    //3) join receiver thread
    std::string result;
    std::thread receiver_thread{[this, &result, type]() {
        result = this->receive(static_cast<ReturnType>(type));
    }};
    //std::this_thread::sleep_for(std::chrono::seconds(5));
    auto msg = pack(type, payload);
    socket.write(msg);
    logger.info("Sent %dB request", msg.size());
    receiver_thread.join();
    return result;
}

std::string Client::receive(ReturnType expected_type)
{
    logger.debug("Receiveing started for ReturnType(%d)", expected_type);
    std::string payload;
    while (42) //FIXME use atomic
    {
        auto raw_header = socket.read(sizeof(i3_ipc_header));
        auto const header = reinterpret_cast<i3_ipc_header_t *>(raw_header.data());
        auto raw_payload = socket.read(header->size);
        if (static_cast<ReturnType>(header->type) == expected_type)
        {
            std::copy(raw_payload.begin(), raw_payload.end(), std::back_inserter(payload));
            break;
        }
    }
    logger.debug("Receiving finished for ReturnType(%d)", expected_type);
    return payload;
}

}
