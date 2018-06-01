//#include <nlohmann/json.hpp>
#include "utils/getoptext.hpp"
#include "utils/logging.hpp"
#include "utils/stringf.hpp"
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

    json current = tree.get_focused_child(tree.root);
    json parent = tree.find_tabbed(current);
    tree.print_node(parent);
    for (json &id : parent["focus"])
    {
        printf("Focus:");
        printf("%ld\n", id.get<uint64_t>());
    }

    if (parser["number"].isSet)
    {
        auto tab_number = parser["number"].to<int>();
        json nodes = parent["nodes"];
        if (tab_number > nodes.size())
        {
            log.error("No tab number %d (only %d tabs)", tab_number, nodes.size());
            exit(1);
        }
        uint64_t id = nodes[tab_number - 1]["id"].get<uint64_t>();

        std::string request = stringf("[con_id=%ld] focus", id);
        log.info("request: %s", request.c_str());

        auto reply = i3_client.request(i3::RequestType::RUN_COMMAND, request);
        log.info("response: %s", reply.c_str());
    }
    else if (parser["direction"].isSet)
    {
        if (parser["tab"].isSet)
        {
            // switch to tab (left, right)
            json nodes = parent["nodes"];
            json target = nullptr;
            std::string direction = parser["direction"].to<std::string>();
            json &focus_id = parent["focus"][0];
            if (direction == "left")
            {
                for (auto itr = nodes.begin(); itr != nodes.end(); itr++)
                {
                    if ((*itr)["id"] == focus_id)
                        break;
                    target = (*itr);
                }
            }
            else if (direction == "right")
            {
                for (auto itr = nodes.rbegin(); itr != nodes.rend(); itr++)
                {
                    if ((*itr)["id"] == focus_id)
                        break;
                    target = (*itr);
                }
            }
            else
            {
                log.error("Can't switch to %s tab, don't know where it is", direction.c_str());
                exit(1);
            }
            if (target == nullptr)
            {
                log.error("Can't switch to %s tab, tab not found", direction.c_str());
                exit(1);
            }

            auto id = target["id"].get<uint64_t>();

            std::string request = stringf("[con_id=%ld] focus", id);
            log.info("request: %s", request.c_str());
            auto reply = i3_client.request(i3::RequestType::RUN_COMMAND, request);
            log.info("response: %s", reply.c_str());
        }
        else
        {
            // switch in direction
        }
    }

    return 0;
}
