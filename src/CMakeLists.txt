find_package(protobuf REQUIRED)
find_package(Qt6 REQUIRED COMPONENTS Core Gui Qml Quick QuickControls2)
qt_standard_project_setup(REQUIRES 6.10)

find_package(CxxQt QUIET)

if(NOT CxxQt_FOUND)
    include(FetchContent)
    FetchContent_Declare(
        CxxQt
        GIT_REPOSITORY https://github.com/kdab/cxx-qt-cmake.git
        GIT_TAG 0.7.2
    )
    
    FetchContent_MakeAvailable(CxxQt)
endif()
    
cxx_qt_import_crate(
    MANIFEST_PATH plugin/Cargo.toml 
    CRATES plugin 
    QT_MODULES Qt::Core Qt::Gui Qt::Qml Qt::QuickControls2
)

cxx_qt_import_qml_module(qml_minimal_qml_module
    URI "myplugin"
    SOURCE_CRATE plugin)

add_subdirectory(eleven)
