#include "desktop.h"

Desktop::Desktop() : m_ipc([this](std::shared_ptr<protos::Desktop> desktop) {
    m_windows.clear();

    std::cout << "[INFO] Received desktop update with " << desktop->active_window().class_() << " clients." << std::endl;
    std::cout << "[INFO] Num clients: " << desktop->clients().clients_size() << std::endl;

    for (const auto& client : desktop->clients().clients())
    {
        std::cout << "[INFO] Adding window with address: " << client.address() << std::endl;
        m_windows.append(new Window(client, this));
    }

    windowsChanged();
}) {}

void Desktop::addWindow(Window* window)
{
    m_windows.append(window);
    windowsChanged();
}

void Desktop::removeWindow(const QString& address)
{
    m_windows.removeIf([&](const Window* w) { return w->getAddress() == address; });
    windowsChanged();
}
