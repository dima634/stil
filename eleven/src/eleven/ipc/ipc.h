#pragma once

#include <proto/desktop.pb.h>
#include <vector>
#include <memory>
#include <future>

class Ipc
{
    public:
    using Listener = std::function<void(std::shared_ptr<protos::Desktop>)>;

    Ipc() = delete;
    Ipc(const Ipc&) = delete;
    Ipc& operator=(const Ipc&) = delete;

    explicit Ipc(Listener listener);
    void listen();

    private:
    Listener m_listener;
    std::future<void> m_listenerFuture;

    std::optional<std::uint64_t> processRawMessage(std::vector<std::uint8_t>& rawMessage, std::uint64_t messageSize);
};
