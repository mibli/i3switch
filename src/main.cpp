//#include <nlohmann/json.hpp>
#include "getoptext.hpp"

#include <string>
#include <iostream>
#include <sstream>
#include <zmq.hpp>
#include <cstdio>

#include <thread>
#include <chrono>


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

int main(int argc, char const **argv)
{
    getoptext::Parser parser({
        {"d", "direction", ""},
        {"t", "tab", ""},
        {"p", "parent", ""},
        {"n", "number", ""}
    });

    // Get socket directory name
    std::string i3_socket = "ipc://" + command("i3 --get-socketpath");

    // Create socket connection
    zmq::context_t context {1};
    zmq::socket_t socket {context, ZMQ_REQ};
    //socket.bind(i3_socket);
    socket.connect(i3_socket);
    std::cout << "Connected: " << i3_socket << std::endl;

    // Create and send a message
    {
        std::string request = "Hello\n";
        zmq::message_t msg {request.size()};
        memcpy(msg.data(), request.data(), request.size());
        socket.send(msg);
        std::cout << "Sent: " << request << std::endl;
    }

    // Create and receive a message
    {
        zmq::message_t msg;
        socket.recv(&msg);
        std::string reply(reinterpret_cast<char *>(msg.data()), msg.size());
        std::cout << "Received: " << reply << std::endl;
    }
//    while (42)
//    {
//        zmq::pollitem_t items[] = { { socket, 0, ZMQ_POLLIN, 0 } };
//        zmq::poll(&items[0], 1, 30);
//
//        std::cout << "Sleeping... " << std::endl;
//        std::this_thread::sleep_for(std::chrono::seconds(1));
//
//        if (items[0].revents & ZMQ_POLLIN)
//        {
//            zmq::message_t msg;
//            socket.recv(&msg);
//            std::string reply(reinterpret_cast<char *>(msg.data()), msg.size());
//            std::cout << "Received: " << reply << std::endl;
//            break;
//        }
//    }

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
