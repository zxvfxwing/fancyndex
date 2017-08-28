#ifndef DIRECTORY_HPP
#define DIRECTORY_HPP

#include <boost/filesystem.hpp>
#include <string>
#include "file.hpp"

namespace fs = boost::filesystem;

class Directory
{
private:
    fs::path directory;
    std::string name;
    bool empty;
    Directory** directories;
    File** files;
    unsigned long long int nb_files;
    unsigned long long int nb_directories;
    unsigned long long int total_size;

    void run_directory();
    void delete_directories();
    void delete_files();

public:
    Directory(fs::path);
    ~Directory();
    bool is_empty() const;
    void add_directory(fs::path);
    void add_file(fs::path);
    unsigned long long int get_size() const;

    Directory* list_directory();
};

#endif //DIRECTORY_HPP
