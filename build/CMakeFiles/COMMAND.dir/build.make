# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.9

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:


#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:


# Remove some rules from gmake that .SUFFIXES does not remove.
SUFFIXES =

.SUFFIXES: .hpux_make_needs_suffix_list


# Suppress display of executed commands.
$(VERBOSE).SILENT:


# A target that is always out of date.
cmake_force:

.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/bin/cmake

# The command to remove a file.
RM = /usr/bin/cmake -E remove -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /home/spoken/Git/fancyndex

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /home/spoken/Git/fancyndex/build

# Utility rule file for COMMAND.

# Include the progress variables for this target.
include CMakeFiles/COMMAND.dir/progress.make

CMakeFiles/COMMAND: app
CMakeFiles/COMMAND: ../--docroot
CMakeFiles/COMMAND: ../.
CMakeFiles/COMMAND: ../--http-address
CMakeFiles/COMMAND: ../0.0.0.0
CMakeFiles/COMMAND: ../--http-port
CMakeFiles/COMMAND: ../9099
	run

COMMAND: CMakeFiles/COMMAND
COMMAND: CMakeFiles/COMMAND.dir/build.make

.PHONY : COMMAND

# Rule to build all files generated by this target.
CMakeFiles/COMMAND.dir/build: COMMAND

.PHONY : CMakeFiles/COMMAND.dir/build

CMakeFiles/COMMAND.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/COMMAND.dir/cmake_clean.cmake
.PHONY : CMakeFiles/COMMAND.dir/clean

CMakeFiles/COMMAND.dir/depend:
	cd /home/spoken/Git/fancyndex/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/spoken/Git/fancyndex /home/spoken/Git/fancyndex /home/spoken/Git/fancyndex/build /home/spoken/Git/fancyndex/build /home/spoken/Git/fancyndex/build/CMakeFiles/COMMAND.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : CMakeFiles/COMMAND.dir/depend

