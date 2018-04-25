#include "getoptext.hpp"

namespace getoptext {

template <>
int Option::to<int>()
{
    return std::atoi(value.c_str());
}


template <>
std::string Option::to<std::string>()
{
    return value;
}

}
