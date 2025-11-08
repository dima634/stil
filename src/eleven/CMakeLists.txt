
#link_directories(/home/dima/repos/stil/plugin/target/debug)

#include_directories(/home/dima/repos/stil/plugin/target/debug/build/plugin-e15cad7f6fa74bba/out/cxxqtbuild/include)


qt_add_qml_module(eleven
    URI eleven
    VERSION 1.0
    SOURCES
        reexport.h
)

target_link_libraries(eleven PRIVATE core)
qt_import_qml_plugins(eleven)


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
