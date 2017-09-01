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
    unsigned long long int size;
    std::string maketime_readable(bool =true);

protected:
    void set_size(const unsigned long long int &);

public:
    FileSystem(fs::path);
    ~FileSystem();
    std::string get_name() const;
    std::string get_date_human() const;
    std::time_t get_date_raw() const;
    unsigned long long int get_size() const;
};

#endif //FILESYSTEM_HPP
