#ifndef FILESYSTEM_HPP
#define FILESYSTEM_HPP

#include <iostream>
#include <string>
#include <ctime>
#include <stdexcept>
#include <boost/filesystem.hpp>
#include <boost/algorithm/string.hpp>

#define NB_UNITS 9

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

    const std::string* ibytes;
    const std::string* bytes;
    const std::string* ibytes_acro;
    const std::string* bytes_acro;

    void init_size_units_str();
    void maketime_readable(bool readable = true);
    void set_size_unit(const std::string & _size_unit);

protected:
    void set_size(const unsigned long long int & size);

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
    long double get_size_human() const;
    long double get_size_peasant() const;
    bool is_dotfile() const;

    virtual void sort_by_name(bool growing=true) =0;
    virtual void sort_by_size(bool growing=true) =0;
    //void shell_sort_by_name(FileSystem** fs, unsigned long long int size, bool direction = true);
};

#endif //FILESYSTEM_HPP
