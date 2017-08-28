#ifndef DIRECTORY_HPP
#define DIRECTORY_HPP

#include <boost/filesystem.hpp>
#include "file.hpp"

namespace fs = boost::filesystem;

class Directory
{
private:
    fs::path directory;
    bool empty;
    Directory* directories;
    File* files;

public:
    Directory(fs::path);
    ~Directory();
    bool isEmpty() const;
};

#endif //DIRECTORY_HPP
