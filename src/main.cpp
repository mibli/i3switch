//#include <nlohmann/json.hpp>
#include "utils/getoptext.hpp"
#include "utils/logging.hpp"
#include "utils/strprintf.hpp"
#include "connection/i3client.hpp"
#include "i3tree.hpp"

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

int constexpr strlength(char const *str)
{
    return *str ? 1 + strlength(str + 1) : 0;
}

int main(int argc, char const **argv)
{
    using nlohmann::json;

    getoptext::Parser parser({
        {"d", "direction", ""},
        {"t", "tab", ""},
        {"p", "parent", ""},
        {"n", "number", ""}
    });

    logging::Logger log;
    log.configure("%s:%s()  ", __FILENAME__, __func__);

    // Verify arguments
    parser.parse(argc, argv);
    if (parser["number"].isSet)
    {
        if (not parser["tab"].isSet)
            print_help_and_die(parser, "Use --number with --tab only");
        if (parser["number"].to<int>() == 0)
            print_help_and_die(parser, "Tab indexes start from 1");
    }
    else if (not parser["direction"].isSet)
    {
        if (parser["tab"].isSet)
            print_help_and_die(parser, "Use --tab only with -d or --direction");
        if (parser["parent"].isSet)
            print_help_and_die(parser, "Use --parent only with -d or --direction");
    }

    // Get socket directory name
    std::string i3_socket_path = command("i3 --get-socketpath");

    // Create socket connection
    i3::Client i3_client(i3_socket_path);

    auto result = i3_client.request(i3::RequestType::GET_TREE, "");
    i3::Tree tree (json::parse(result));
    auto current = tree.find_focused();
    auto parent = tree.find_parent_of(current);

    if (parser["number"].isSet)
    {
        auto tab_number = parser["number"].to<int>();
        if (tab_number > parent["nodes"].size())
        {
            log.error("No tab number %d (only %d tabs)", tab_number, parent["nodes"].size());
            exit(1);
        }
        auto id = parent["nodes"][tab_number - 1]["id"].get<uint64_t>();

        char *request = strprintf("[con_id=%ld] focus", id);
        log.info("request: %s", request);

        auto reply = i3_client.request(i3::RequestType::RUN_COMMAND, request);
        free(request);
        log.info("response: %s", reply.c_str());
    }
    else if (parser["direction"].isSet)
    {
        if (parser["tabs"].isSet)
        {
            // switch to tab (left, right)
        }
        else
        {
            // switch in direction
        }
    }

    return 0;
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
