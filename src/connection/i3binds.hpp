#pragma once

extern "C" {
#include <i3/ipc.h>
}
#include <cstddef>

namespace i3
{

enum class RequestType : size_t {
    RUN_COMMAND = I3_IPC_MESSAGE_TYPE_RUN_COMMAND,
    GET_WORKSPACES = I3_IPC_MESSAGE_TYPE_GET_WORKSPACES,
    SUBSCRIBE = I3_IPC_MESSAGE_TYPE_SUBSCRIBE,
    GET_OUTPUTS = I3_IPC_MESSAGE_TYPE_GET_OUTPUTS,
    GET_TREE = I3_IPC_MESSAGE_TYPE_GET_TREE,
    GET_MARKS = I3_IPC_MESSAGE_TYPE_GET_MARKS,
    GET_BAR_CONFIG = I3_IPC_MESSAGE_TYPE_GET_BAR_CONFIG,
    GET_VERSION = I3_IPC_MESSAGE_TYPE_GET_VERSION,
    GET_BINDING_MODES = I3_IPC_MESSAGE_TYPE_GET_BINDING_MODES,
    GET_CONFIG = I3_IPC_MESSAGE_TYPE_GET_CONFIG,
    SEND_TICK = I3_IPC_MESSAGE_TYPE_SEND_TICK
};

enum class ReturnType : size_t {
    COMMAND = I3_IPC_REPLY_TYPE_COMMAND,
    WORKSPACES = I3_IPC_REPLY_TYPE_WORKSPACES,
    SUBSCRIBE = I3_IPC_REPLY_TYPE_SUBSCRIBE,
    OUTPUTS = I3_IPC_REPLY_TYPE_OUTPUTS,
    TREE = I3_IPC_REPLY_TYPE_TREE,
    MARKS = I3_IPC_REPLY_TYPE_MARKS,
    BAR_CONFIG = I3_IPC_REPLY_TYPE_BAR_CONFIG,
    VERSION = I3_IPC_REPLY_TYPE_VERSION,
    BINDING_MODES = I3_IPC_REPLY_TYPE_BINDING_MODES,
    CONFIG = I3_IPC_REPLY_TYPE_CONFIG,
    TICK = I3_IPC_REPLY_TYPE_TICK
};

enum class EventType : size_t {
    WORKSPACE = I3_IPC_EVENT_WORKSPACE,
    OUTPUT = I3_IPC_EVENT_OUTPUT,
    MODE = I3_IPC_EVENT_MODE,
    WINDOW = I3_IPC_EVENT_WINDOW,
    BARCONFIG_UPDATE = I3_IPC_EVENT_BARCONFIG_UPDATE,
    BINDING = I3_IPC_EVENT_BINDING,
    SHUTDOWN = I3_IPC_EVENT_SHUTDOWN,
    TICK = I3_IPC_EVENT_TICK
};

class Header : public i3_ipc_header {
public:
    Header(uint32_t type, uint32_t size);
    Header(RequestType type, uint32_t size);
    Header(ReturnType type, uint32_t size);
    Header(EventType type, uint32_t size);
};

}
