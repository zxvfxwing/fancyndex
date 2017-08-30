#ifndef FILE_HPP
#define FILE_HPP

#include "filesystem.hpp"

class File
{
private:
    std::string extension;

public:
    File(fs::path);
    ~File();
};

#endif //FILE_HPP
