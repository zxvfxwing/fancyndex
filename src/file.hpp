#ifndef FILE_HPP
#define FILE_HPP

#include "filesystem.hpp"

class File : public FileSystem
{
private:
    std::string extension;
    bool extension_ok;
    bool dotfile;

public:
    File(fs::path);
    ~File();
    std::string get_extension() const;
    bool is_extension_ok() const;
    bool is_dotfile() const;
};

#endif //FILE_HPP
