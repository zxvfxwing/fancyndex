#ifndef FILESYSTEM_HPP
#define FILESYSTEM_HPP

#include <iostream>
#include <string>
#include <ctime>
#include <stdexcept>
#include <boost/filesystem.hpp>

namespace fs = boost::filesystem;

class FileSystem
{
private:
    fs::path path;
    std::string name;
    std::time_t date_raw;
    std::string date_human;
    unsigned long long int size;
    bool dotfile;

    void maketime_readable(bool =true);

protected:
    void set_size(const unsigned long long int &);

public:
    FileSystem(fs::path);
    ~FileSystem();
    std::string get_canonical_name() const;
    std::string get_canonical() const;
    std::string get_absolute() const;
    std::string get_name() const;
    std::string get_date_human() const;
    std::time_t get_date_raw() const;
    unsigned long long int get_size() const;
    std::string get_size_str() const;
    bool is_dotfile() const;
};

#endif //FILESYSTEM_HPP
