find_package(Qt6 REQUIRED COMPONENTS Core Gui Qml Quick)
qt_standard_project_setup(REQUIRES 6.10)

qt_add_qml_module(eleven
    URI eleven
    VERSION 1.0
    SOURCES
        test.h test.cpp
)
