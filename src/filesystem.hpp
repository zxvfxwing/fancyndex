#ifndef FILESYSTEM_HPP
#define FILESYSTEM_HPP

#include <iostream>
#include <fstream>
#include <string>
#include <ctime>
#include <cstdlib>
#include <stdexcept>
#include <algorithm>
#include <vector>
#include <thread>
#include <boost/filesystem.hpp>
#include <boost/algorithm/string.hpp>

namespace fs = boost::filesystem;

class FileSystem
{
private:
    fs::path path;
    unsigned long long int size;
    unsigned int ibyte_pow;
    unsigned int byte_pow;
    std::string str_ib_size;
    std::string str_b_size;

    void compute_unit(unsigned int wanted_precision=3);

    /* "How to sort" functions */
    static bool by_name_ascending(FileSystem* f1, FileSystem* f2);
    static bool by_name_decreasing(FileSystem* f1, FileSystem* f2);
    static bool by_size_ascending(FileSystem* f1, FileSystem* f2);
    static bool by_size_decreasing(FileSystem* f1, FileSystem* f2);
    static bool by_date_ascending(FileSystem* f1, FileSystem* f2);
    static bool by_date_decreasing(FileSystem* f1, FileSystem* f2);

protected:
    void set_size(const unsigned long long int & _size);

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

    std::string get_size_human(unsigned int mode=0) const;
    std::string get_size_peasant(unsigned int mode=0) const;

    bool is_dotfile() const;

    void sort_(FileSystem** fs, unsigned long long int size, unsigned int mode);
};

#endif //FILESYSTEM_HPP
