find_package(protobuf REQUIRED)
find_package(Qt6 REQUIRED COMPONENTS Core Gui Qml Quick)
qt_standard_project_setup(REQUIRES 6.10)

qt_add_qml_module(eleven
    URI eleven
    VERSION 1.0
    SOURCES
        desktop.h desktop.cpp
        window.h window.cpp
        ipc/ipc.h ipc/ipc.cpp
        proto/desktop.pb.cc proto/desktop.pb.h
        proto/hyprland.pb.cc proto/hyprland.pb.h
)
target_link_libraries(eleven PRIVATE protobuf)

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
