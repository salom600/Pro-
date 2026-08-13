cmake_minimum_required(VERSION 3.20)
project(ProVideoEditor VERSION 0.3.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)

# ── Find Qt6 ──
find_package(Qt6 REQUIRED COMPONENTS Widgets Multimedia)
qt_standard_project_setup()

# ── Sources ──
set(SOURCES
    src/main.cpp
    src/MainWindow.cpp
    src/MediaBinWidget.cpp
    src/TimelineWidget.cpp
    src/PreviewWidget.cpp
    src/PropertiesPanel.cpp
    src/EngineBridge.cpp
)

set(HEADERS
    src/MainWindow.h
    src/MediaBinWidget.h
    src/TimelineWidget.h
    src/PreviewWidget.h
    src/PropertiesPanel.h
    src/EngineBridge.h
)

# ── Rust library ──
# The Rust cdylib is built separately via cargo and linked here.
set(RUST_LIB_DIR ${CMAKE_SOURCE_DIR}/../target/release)
set(RUST_LIB_NAME pro_video_editor)

if(WIN32)
    set(RUST_LIB ${RUST_LIB_DIR}/${RUST_LIB_NAME}.dll.lib)
    set(RUST_DLL ${RUST_LIB_DIR}/${RUST_LIB_NAME}.dll)
elseif(APPLE)
    set(RUST_LIB ${RUST_LIB_DIR}/lib${RUST_LIB_NAME}.dylib)
else()
    set(RUST_LIB ${RUST_LIB_DIR}/lib${RUST_LIB_NAME}.so)
endif()

# ── Include dirs ──
include_directories(
    ${CMAKE_SOURCE_DIR}/../ffi/include
    src
)

# ── Executable ──
qt_add_executable(ProVideoEditor ${SOURCES} ${HEADERS})

target_link_libraries(ProVideoEditor PRIVATE
    Qt6::Widgets
    Qt6::Multimedia
)

# Link Rust library if it exists
if(EXISTS ${RUST_LIB})
    target_link_libraries(ProVideoEditor PRIVATE ${RUST_LIB})
    message(STATUS "Found Rust library: ${RUST_LIB}")
else()
    message(WARNING "Rust library not found at ${RUST_LIB} — build it first with: cargo build --release")
endif()

# ── Install ──
install(TARGETS ProVideoEditor DESTINATION bin)

# Copy Rust DLL next to executable (Windows)
if(WIN32 AND EXISTS ${RUST_DLL})
    add_custom_command(TARGET ProVideoEditor POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_if_different
        ${RUST_DLL} $<TARGET_FILE_DIR:ProVideoEditor>
    )
endif()
