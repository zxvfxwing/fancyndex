#include "directory.hpp"

Directory::Directory(fs::path _directory)
    :directory(_directory)
{
    // List all files & directories

}

Directory::~Directory()
{
    delete [] directories;
    delete [] files;
}
