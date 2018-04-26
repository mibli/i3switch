//#include <nlohmann/json.hpp>
#include "utils/getoptext.hpp"
#include "utils/logging.hpp"
#include "connection/i3client.hpp"

#include <string>
#include <iostream>
#include <sstream>
#include <cstdio>
#include <thread>
#include <vector>
#include <algorithm>

#include <thread>
#include <chrono>
#include <nlohmann/json.hpp>

std::string command(std::string const &command, bool strip_last = true)
{
    FILE *pipe = popen(command.c_str(), "r");
    if (pipe == nullptr)
    {
        printf("failed to create pipe for '%s'", command.c_str());
        return {};
    }

    std::stringstream stream;
    char buffer[256];

    while(fgets(buffer, sizeof(buffer), pipe) != nullptr)
        stream << buffer;

    auto rc = pclose(pipe);
    if (rc > 0)
        printf("'%s' exited with an error: %d", command.c_str(), rc);

    std::string result = stream.str();
    if (strip_last)
    {
        result.erase(result.size() - 1);
    }

    return result;
}

void print_help_and_die(getoptext::Parser &p, char const *msg)
{
    p.print_help();
    std::cout << msg << std::endl;
    exit(1);
}

using nlohmann::json;

json find_focused(json const &obj)
{
    if (obj["focused"] == true)
    { return obj; }
    for (auto &node : obj["nodes"])
    {
        auto found = find_focused(node);
        if (found != nullptr)
        { return found; }
    }
    return nullptr;
}

int main(int argc, char const **argv)
{
    getoptext::Parser parser({
        {"d", "direction", ""},
        {"t", "tab", ""},
        {"p", "parent", ""},
        {"n", "number", ""}
    });

    logging::Logger log;
    log.configure("%s:%s()  ", __FILENAME__, __func__);

    // Get socket directory name
    std::string i3_socket_path = command("i3 --get-socketpath");

    // Create socket connection
    i3::Client i3_client(i3_socket_path);

    auto result = i3_client.request(i3::RequestType::GET_TREE, "");

    //log.info("Received message:\n%s", result.c_str());
    std::string err;
    auto tree = json::parse(result);
    auto current = find_focused(tree);
    log.info("focused node id: %ld", current["id"].get<uint64_t>());

    exit(1);

    parser.parse(argc, argv);
    auto number = parser["number"].to<int>();
    print_help_and_die(parser, "test");
    //int number = 0;
    //if (parser.exists("number"))
    //    number = parser.retrieve<int>("number");

    //if (not parser.exists("direction"))
    //{
    //    if (parser.exists("tab"))
    //    {
    //        print_help_and_die(parser,
    //                "Use --tab only with -d or --direction");
    //    }
    //    if (parser.exists("parent"))
    //    {
    //        print_help_and_die(parser,
    //                "Use --parent only with -d or --direction");
    //    }
    //}
    //if (parser.exists("number") and number <= 0)
    //{
    //    print_help_and_die(parser, "Tab indexes start from 1");
    //}
    return 0;
}
