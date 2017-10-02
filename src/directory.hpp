#ifndef DIRECTORY_HPP
#define DIRECTORY_HPP

#include "filesystem.hpp"
#include "file.hpp"

class Directory : public FileSystem
{
private:
    bool empty;
    unsigned long long int nb_files;
    unsigned long long int nb_directories;
    unsigned long long int nb_elements;
    File** files;
    Directory** directories;

    void destructor_files();
    void destructor_directories();
    void add_a_file(fs::path);
    void add_a_directory(fs::path);
    void run_directory(fs::path);
    unsigned long long int sum_size();
    void sum_elements();

public:
    Directory(fs::path);
    ~Directory();
    bool is_empty() const;
    unsigned long long int get_nb_files() const;
    unsigned long long int get_nb_directories() const;
    unsigned long long int get_nb_elements() const;
    File** get_files() const;
    File* get_file(unsigned long long int) const;
    Directory** get_directories() const;
    Directory* get_directory(unsigned long long int) const;
    void sort_directories(unsigned int mode);
    void sort_files(unsigned int mode);

    void sort_dirs_by_name(bool ascending=true);
    void sort_dirs_by_size(bool ascending=true);
    void sort_files_by_name(bool ascending=true);
    void sort_files_by_size(bool ascending=true);
    void sort_els_by_name(bool ascending=true);
    void sort_els_by_size(bool ascending=true);
};

#endif //DIRECTORY_HPP
