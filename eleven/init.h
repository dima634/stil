#pragma once

#include <stil_core/src/lib.rs.h>

// This runs when the plugin shared library is loaded
__attribute__((constructor)) static void init_stil_core()
{
    core::init();
}
