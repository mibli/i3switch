//#include <nlohmann/json.hpp>
#include "getoptext.hpp"
#include "logging.hpp"
#include "socket.hpp"

#include <string>
#include <iostream>
#include <sstream>
#include <cstdio>
#include <thread>

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

int receiver(Socket &socket)
{
    std::cout << "Receiver started" << std::endl;
    while (42)
    {
        auto result = socket.read(1);
        std::cout << "Received " << result << std::endl;
    }
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
    Socket socket {i3_socket_path};
    log.info("Connected %s", i3_socket_path.c_str());

    std::thread receiver_thread {[&socket](){ receiver(socket); }};

    std::this_thread::sleep_for(std::chrono::seconds(5));
    // Create and send a message
    {
        // this is a hassle, let's let i3ipc do it
        struct {
            uint8_t magic [6];
            uint32_t size;
            uint32_t type;
        } __attribute__((packed)) msg;
        std::string request = "";
        socket.write(request);
        log.info("Sent: %s", request.c_str());
    }

    // Create and receive a message
    receiver_thread.join();
    exit(1);
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
