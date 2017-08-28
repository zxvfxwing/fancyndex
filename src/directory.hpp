#ifndef DIRECTORY_HPP
#define DIRECTORY_HPP

#include <boost/filesystem.hpp>
#include "file.hpp"

fs = boost::filesystem;

class Directory :
{
private:
    bool empty;
    Directory* Directories;
    Files* files;

public:
    Directory();
    ~Directory();
    const bool isEmpty() const;
};

#endif //DIRECTORY_HPP
