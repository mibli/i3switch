#pragma once

#include <string>
#include <array>
#include <cstdio>
#include <cstring>

#define __FILENAME__ (strrchr(__FILE__, '/') ? strrchr(__FILE__, '/') + 1 : __FILE__)

namespace logging {

    enum LogLevel {
        CRITICAL,
        ERROR,
        WARNING,
        INFO,
        DEBUG,
        LEVELS_COUNT
    };

    static const char *names[LEVELS_COUNT] = {
        "(CC)",
        "(EE)",
        "(WW)",
        "(II)",
        "(DD)"
    };
    static LogLevel level {CRITICAL};
    static char const *prefix {"%s "};

    template<typename ... Args>
    void log(LogLevel loglevel, char const *format, Args ... args)
    {
        if (loglevel >= level) {
            printf(prefix, names[loglevel]);
            printf(format, args...);
            printf("\n");
        }
    }

    template<typename ...Args>
    void critical(char const *format, Args...args) {
        logging::log<Args...>(CRITICAL, format, args...);
    }

    template<typename ...Args>
    void error(char const *format, Args...args) {
        logging::log<Args...>(ERROR, format, args...);
    }

    template<typename ...Args>
    void warning(char const *format, Args...args) {
        logging::log<Args...>(WARNING, format, args...);
    }

    template<typename ...Args>
    void info(char const *format, Args...args) {
        logging::log<Args...>(INFO, format, args...);
    }

    template<typename... Args>
    void debug(char const *format, Args ... args) {
        logging::log<Args ...>(DEBUG, format, args...);
    }

    class Logger
    {
    private:
        template<typename ...Args>
        void log(LogLevel loglevel, char const *format, Args...args)
        {
            if (loglevel >= level) {
                printf(logging::prefix, names[loglevel]);
                printf("%s", this->prefix.c_str());
                printf(format, args...);
                printf("\n");
            }
        }

    public:
        std::string prefix;

        Logger(char const *prefix = logging::prefix)
            :prefix(prefix) {}
        ~Logger() = default;

        template<typename ...Args>
        void configure(char const *format, Args...args) {
            char buffer[256];
            sprintf(buffer, format, args...);
            this->prefix = buffer;
        }

        template<typename ...Args>
        inline void critical(char const *format, Args...args)
        {
            this->log<Args...>(CRITICAL, format, args...);
            //TODO EXIT_CODES
            exit(1);
        }

        template<typename ...Args>
        inline void error(char const *format, Args...args)
        {
            this->log<Args...>(ERROR, format, args...);
        }

        template<typename ...Args>
        inline void warning(char const *format, Args...args)
        {
            this->log<Args...>(WARNING, format, args...);
        }

        template<typename ...Args>
        inline void info(char const *format, Args...args)
        {
            this->log<Args...>(INFO, format, args...);
        }

        template<typename ...Args>
        inline void debug(char const *format, Args...args)
        {
#ifdef DEBUG
            this->log<Args...>(DEBUG, format, args...);
#endif
        }
    };
}
