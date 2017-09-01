#ifndef DIRECTORY_HPP
#define DIRECTORY_HPP

#include "filesystem.hpp"
#include "file.hpp"

namespace fs = boost::filesystem;

class Directory : public FileSystem
{
private:
    bool empty;
    unsigned long long int nb_files;
    unsigned long long int nb_directories;
    File** files;
    Directory** directories;

    void destructor_files();
    void destructor_directories();
    void add_a_file(fs::path);
    void add_a_directory(fs::path);
    void run_directory(fs::path);

public:
    Directory(fs::path);
    ~Directory();
    bool is_empty() const;
    unsigned long long int get_nb_files() const;
    unsigned long long int get_nb_directories() const;
    File** get_files() const;
    File* get_file(unsigned long long int) const;
    Directory** get_directories() const;
    Directory* get_directory(unsigned long long int) const;
    unsigned long long int sum_size();
};

#endif //DIRECTORY_HPP
