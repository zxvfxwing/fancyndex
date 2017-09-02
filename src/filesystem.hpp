#ifndef FILESYSTEM_HPP
#define FILESYSTEM_HPP

#include <iostream>
#include <string>
#include <time.h>
#include <stdexcept>
#include <boost/filesystem.hpp>

namespace fs = boost::filesystem;

class FileSystem
{
private:
    fs::path path;
    std::string name;
    std::string date_human;
    std::time_t date_raw;
    long double size;
    std::string maketime_readable(bool =true);

protected:
    void set_size(const long double &);

public:
    FileSystem(fs::path);
    ~FileSystem();
    std::string get_canonical() const;
    std::string get_name() const;
    std::string get_date_human() const;
    std::time_t get_date_raw() const;
    long double get_size() const;
    std::string get_size_str() const;
};

#endif //FILESYSTEM_HPP
