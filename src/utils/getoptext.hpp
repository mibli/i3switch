#pragma once

#include <stdexcept>
#include <string>
#include <getopt.h>
#include <vector>

namespace getoptext
{

struct Option
{
    std::string shortName;
    std::string longName;
    std::string description;
    std::string value;
    bool exists {false};

    Option(std::string shortName)
        : shortName(shortName) {}
    Option(std::string shortName, std::string description)
        : shortName(shortName), description(description) {}
    Option(std::string shortName, std::string longName, std::string description)
        : shortName(shortName), longName(longName), description(description) {}

    template <typename T> T to();
};

class Parser
{
    Option *current;

public:
    std::vector<Option> opts;

    Parser() = default;
    Parser(std::vector<Option> initlist)
        : opts(std::move(initlist)) {}

    void print_help()
    {
        for (auto &opt : opts)
        {
            printf("    ");
            if (not opt.shortName.empty())
                printf("-%s", opt.shortName.c_str());
            if (not opt.shortName.empty() and
                not opt.longName.empty())
                printf(", ");
            if (not opt.longName.empty())
                printf("--%s\n", opt.longName.c_str());
            if (not opt.description.empty())
                printf("        %s\n", opt.description.c_str());
        }
    }

    void parse(int argc, char const **argv)
    {
        for (size_t i = 1; i < argc; ++i)
        {
            std::string arg = argv[i];
            if(arg[0] == '-')
            {
                if(arg.size() > 1 and arg[1] == '-')
                {
                    std::string needle = std::string(arg.data() + 2, arg.size() - 2);
                    for(auto &opt : opts)
                    {
                        if(opt.longName == needle)
                        {
                            opt.exists = true;
                            current = &opt;
                            break;
                        }
                    }
                }
                else
                {
                    std::string needle = std::string(arg.data() + 1, arg.size() - 1);
                    for(auto &opt : opts)
                    {
                        if(opt.shortName == needle)
                        {
                            opt.exists = true;
                            current = &opt;
                            break;
                        }
                    }
                }
                continue;
            }
            //TODO(mblizniak) append
            //FIXME(error if no opt)
            current->value = arg;
        }
    }

    Option &operator[] (char const *optName)
    {
        for (auto &opt : opts)
            if (opt.shortName == optName ||
                opt.longName == optName)
                return opt;
        throw std::logic_error("Accessing not declared option");
    }
};

}//namespace getopt
