find_package(Qt6 REQUIRED COMPONENTS Core Gui Qml Quick QuickControls2)
qt_standard_project_setup(REQUIRES 6.10)

link_directories(../target/debug)
include_directories(../target/cxxbridge)

qt_add_qml_module(eleven
    URI eleven
    VERSION 1.0
    SOURCES
        workspace.h workspace.cpp
        workspaces.h workspaces.cpp
        client.h client.cpp
        clients.h clients.cpp
        system_events.h system_events.cpp
        ../target/cxxbridge/stil_core/src/ffi/mod.rs.cc
        ../target/cxxbridge/stil_core/src/ffi/system_events.rs.cc
        ../target/cxxbridge/stil_core/src/ffi/hyprland.rs.cc
)

target_link_libraries(eleven PRIVATE libstil_core.a)

set(install_script "${CMAKE_CURRENT_BINARY_DIR}/install_plugin.cmake")
file(GENERATE OUTPUT ${install_script} CONTENT "
    include(${QT_DEPLOY_SUPPORT})

    file(COPY
        \"${CMAKE_CURRENT_BINARY_DIR}/libeleven.so\"
        \"${CMAKE_CURRENT_BINARY_DIR}/libelevenplugin.so\"
        \"${CMAKE_CURRENT_BINARY_DIR}/qmldir\"
        \"${CMAKE_CURRENT_BINARY_DIR}/eleven.qmltypes\"
        DESTINATION \"/\${QT_DEPLOY_QML_DIR}/eleven\"
    )
")

install(SCRIPT ${install_script})
