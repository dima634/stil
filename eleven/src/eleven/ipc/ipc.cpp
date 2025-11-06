#include "ipc.h"
#include <bit>
#include <sys/socket.h>
#include <sys/un.h>

const char* socketPath = "/tmp/stil.sock";

Ipc::Ipc(Listener listener) : m_listener(std::move(listener))
{
    m_listenerFuture = std::async(std::launch::async, &Ipc::listen, this);
}

void Ipc::listen()
{
    auto sock = socket(AF_UNIX, SOCK_STREAM, 0);

    if (sock == -1)
    {
        std::cerr << "[ERROR] Failed to create socket" << std::endl;
        return;
    }

    sockaddr_un socketAddress;
    socketAddress.sun_family = AF_UNIX;
    strcpy(socketAddress.sun_path, socketPath);

    if (connect(sock, (struct sockaddr*)&socketAddress, sizeof(socketAddress)) != 0)
    {
        std::cerr << "[ERROR] Failed to connect to socket: " << socketPath << std::endl;
        return;
    }

    std::cout << "[INFO] Listening on socket: " << socketPath << std::endl;

    std::vector<std::uint8_t> buffer(1024, 0);
    std::vector<std::uint8_t> rawMessage;
    std::optional<std::uint64_t> messageSize;

    while (true)
    {
        auto readResult = read(sock, buffer.data(), buffer.size());

        if (readResult == -1)
        {
            std::cerr << "[ERROR] Failed to read from socket" << std::endl;
            return;
        }

        if (readResult == 0)
        {
            std::cout << "[INFO] Socket reached EOF" << std::endl;
            return;
        }

        std::cout << "[DEBUG] Read " << readResult << " bytes from socket" << std::endl;

        if (messageSize.has_value())
        {
            rawMessage.insert(rawMessage.end(), buffer.begin(), buffer.begin() + readResult);
        }
        else
        {
            std::array<std::uint8_t, sizeof(std::uint64_t)> sizeBuffer;
            std::copy(buffer.begin(), buffer.begin() + sizeof(std::uint64_t), sizeBuffer.begin());
            messageSize = std::bit_cast<std::uint64_t>(sizeBuffer);
            rawMessage.insert(rawMessage.end(), buffer.begin() + sizeof(std::uint64_t), buffer.begin() + readResult);
        }

        messageSize = processRawMessage(rawMessage, *messageSize);
    }
}


std::optional<std::uint64_t> Ipc::processRawMessage(std::vector<std::uint8_t>& rawMessage, std::uint64_t messageSize)
{
    std::cout << "[DEBUG] Processing raw message of size " << rawMessage.size() << " bytes, expected size: " << messageSize << " bytes" <<  std::endl;

    if (rawMessage.size() < messageSize)
    {
        return messageSize;
    }

    // Complete message received
    std::shared_ptr<protos::Desktop> desktop = std::make_shared<protos::Desktop>();
    if (desktop->ParseFromArray(rawMessage.data(), messageSize))
    {
        std::cout << "[DEBUG] Parsed message successfully" << std::endl;
        m_listener(desktop);
    }
    else
    {
        std::cerr << "[ERROR] Failed to parse message" << std::endl;
    }
    
    std::uint64_t extraDataSize = rawMessage.size() - messageSize;

    if (extraDataSize > 0)
    {
        std::uint64_t nextMessageSize = std::bit_cast<std::uint64_t>(rawMessage.data() + messageSize);
        rawMessage.erase(rawMessage.begin(), rawMessage.begin() + messageSize + 1);

        if (rawMessage.size() >= nextMessageSize)
        {
            return processRawMessage(rawMessage, nextMessageSize);
        }
        else
        {
            return nextMessageSize;
        }
    }
    else
    {
        rawMessage.clear();
        return std::nullopt;
    }
}
