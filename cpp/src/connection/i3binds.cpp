#include "i3binds.hpp"

#include <cstring>

namespace i3
{

Header::Header(uint32_t type, uint32_t size)
{
    strncpy(magic, I3_IPC_MAGIC, 6);
    this->type = type;
    this->size = size;
}

Header::Header(RequestType type, uint32_t size)
    : Header(static_cast<uint32_t>(type), size)
{}

Header::Header(ReturnType type, uint32_t size)
    : Header(static_cast<uint32_t>(type), size)
{}

Header::Header(EventType type, uint32_t size)
    : Header(static_cast<uint32_t>(type), size)
{}

}
