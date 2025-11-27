#include "system.h"
#include <stil_core/src/system.rs.h>

bool QSystem::poweroff() const
{
    return core::system::power_off();
}

bool QSystem::reboot() const
{
    return core::system::reboot();
}
