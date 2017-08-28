#ifndef FILE_HPP
#define FILE_HPP

#include <iostream>
#include <stdexcept>
#include <time.h>
#include <string>
#include <boost/filesystem.hpp>

namespace fs = boost::filesystem;

class File
{
private:
    fs::path file;
    std::string filename;
    std::string extension;
    std::string date_human;
    std::time_t date_raw;
    unsigned long long int size;

public:
    File(fs::path);
    ~File();
    std::string get_name() const;
    std::string get_extension() const;
    std::string get_date_human() const;
    std::time_t get_date_raw() const;
    unsigned long long int get_size() const;
    std::string maketime_readable(std::time_t, bool =true);
};

#endif //FILE_HPP
