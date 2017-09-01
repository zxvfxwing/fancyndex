#ifndef FILE_HPP
#define FILE_HPP

#include "filesystem.hpp"

class File : public FileSystem
{
private:
    std::string extension;

public:
    File(fs::path);
    ~File();
    std::string get_extension() const;
};

#endif //FILE_HPP
