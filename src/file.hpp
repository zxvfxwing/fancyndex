#ifndef FILE_HPP
#define FILE_HPP

#include "filesystem.hpp"

/*
*   File Class
*   Describe a file
*   boost::filesytem only allow file to use 'file_size(path p)',
*   so only File can implements a method to calcul size.
*   Directory will just makes the sum of all files it contains.
*
*   Noticable difference with a Directory : extension.
*   Only file can have an extension.
*/

class File : public FileSystem
{
private:
    std::string extension;
    bool extension_ok;

public:
    File(fs::path);
    ~File();
    std::string get_extension() const;
    bool is_extension_ok() const;
};

#endif //FILE_HPP
