find_package(Qt6 REQUIRED COMPONENTS Core Gui Qml Quick QuickControls2)
qt_standard_project_setup(REQUIRES 6.10)

link_directories("${STIL_CORE_LIB_DIR}")
include_directories(../target/cxxbridge)

set(QT_QML_GENERATE_QMLLS_INI ON)
set_source_files_properties(
    qml/Css.qml 
    qml/Theme.qml
    PROPERTIES
    QT_QML_SINGLETON_TYPE TRUE
)

# Skip linting for generated sources
file(GLOB_RECURSE CXXBRIDGE_FILES
    "../target/**/*.cc"
)
set_source_files_properties(
    ${CXXBRIDGE_FILES}
    PROPERTIES
    SKIP_LINTING ON
)

qt_add_qml_module(eleven
    URI eleven
    VERSION 1.0
    SOURCES
        init.h init.cpp
        workspace.h workspace.cpp
        workspaces.h workspaces.cpp
        window.h window.cpp
        app.h app.cpp
        pinned_apps.h pinned_apps.cpp
        windows.h windows.cpp
        wallclock.h wallclock.cpp
        cpu.h cpu.cpp
        memory.h memory.cpp
        system.h system.cpp
        system_events.h system_events.cpp
        icon_lookup.h icon_lookup.cpp
        ../target/cxxbridge/stil_core/src/lib.rs.cc
        ../target/cxxbridge/stil_core/src/system.rs.cc
        ../target/cxxbridge/stil_core/src/system_events.rs.cc
        ../target/cxxbridge/stil_core/src/ffi.rs.cc
        ../target/cxxbridge/stil_core/src/freedesktop/ffi.rs.cc
    QML_FILES
        qml/TaskBar.qml
        qml/TaskbarButton.qml
        qml/WorkspaceList.qml
        qml/WindowList.qml
        qml/PinnedAppsList.qml
        qml/SystemTray.qml
        qml/PowerControlMenu.qml
        qml/Flyout.qml
        qml/ListItem.qml
        qml/Theme.qml
        qml/Css.qml
)

target_link_libraries(eleven PRIVATE libstil_core.a sqlite3)

set(install_script "${CMAKE_CURRENT_BINARY_DIR}/install_plugin.cmake")
file(GENERATE OUTPUT ${install_script} CONTENT "
    include(${QT_DEPLOY_SUPPORT})

    file(GLOB PLUGIN_FILES
        \"${CMAKE_CURRENT_BINARY_DIR}/libeleven.so\"
        \"${CMAKE_CURRENT_BINARY_DIR}/libelevenplugin.so\"
        \"${CMAKE_CURRENT_BINARY_DIR}/qmldir\"
        \"${CMAKE_CURRENT_BINARY_DIR}/eleven.qmltypes\"
        \"${CMAKE_CURRENT_BINARY_DIR}/qml\"
    )

    file(COPY \${PLUGIN_FILES} DESTINATION \"/\${QT_DEPLOY_QML_DIR}/eleven/\")
")

install(SCRIPT ${install_script})
