#ifndef SYMLINK_HPP
#define SYMLINK_HPP

#include "filesystem.hpp"
#include "directory.hpp"
#include "file.hpp"

/*
*   Represents a UNIX symbolic link.
*/

class Symlink : public FileSystem
{
private:
    Directory* directory;
    File* file;
    fs::path sym_path;

public:
    Symlink(fs::path);
    ~Symlink();
    fs::path get_path() const;
};

#endif //SYMLINK_HPP
