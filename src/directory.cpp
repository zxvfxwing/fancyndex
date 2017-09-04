#include "directory.hpp"

Directory::Directory(fs::path directory)
    :FileSystem(directory),
    empty(true),
    nb_files(0),
    nb_directories(0),
    nb_elements(0),
    files(NULL),
    directories(NULL)
{
    try
    {
        if(fs::is_directory(directory))
        {
            run_directory(directory);
            set_size(sum_size());
            sum_elements();
        }
        else
        {
            throw std::runtime_error("ERROR: " + directory.filename().string() + " is not a directory.\n");
        }
    }
    catch(const fs::filesystem_error& e)
    {
        std::cerr << e.what() << std::endl;
    }
}

Directory::~Directory()
{
    destructor_files();
    destructor_directories();
}

bool Directory::is_empty() const
{
    return empty;
}

void Directory::destructor_files()
{
    if(files != NULL){
        unsigned long long int i;
        for(i=0; i < nb_files; ++i)
        {
            if(files[i] != NULL)
                delete files[i];
        }
        delete [] files;
    }
}

void Directory::destructor_directories()
{
    if(directories != NULL){
        unsigned long long int i;
        for(i=0; i < nb_directories; ++i)
        {
            if(directories[i] != NULL)
                delete directories[i];
        }
        delete [] directories;
    }
}

void Directory::add_a_file(fs::path path_file)
{
    File** array = new File* [++nb_files];

    if(nb_files > 1)
    {
        unsigned long long int i;
        for(i=0; i < nb_files-1; ++i)
            array[i] = files[i];
    }

    files = array;
    files[nb_files-1] = new File(path_file);
}

void Directory::add_a_directory(fs::path path_dir)
{
    Directory** array = new Directory* [++nb_directories];

    if(nb_directories > 1)
    {
        unsigned long long int i;
        for(i=0; i < nb_directories-1; ++i)
            array[i] = directories[i];
    }

    directories = array;
    directories[nb_directories-1] = new Directory(path_dir);
}

void Directory::run_directory(fs::path dir)
{
    try{
        for(fs::directory_entry& entry: fs::directory_iterator(dir))
        {
            if(fs::is_directory(entry))
                add_a_directory(entry);
            else
                add_a_file(entry);
        }
    }
    catch(const fs::filesystem_error& e)
    {
        std::cerr << e.what() << std::endl;
    }
}

unsigned long long int Directory::get_nb_files() const
{
    return nb_files;
}

unsigned long long int Directory::get_nb_directories() const
{
    return nb_directories;
}

unsigned long long int Directory::get_nb_elements() const
{
    return nb_elements;
}

File** Directory::get_files() const
{
    return files;
}

File* Directory::get_file(unsigned long long int index) const
{
    assert(index < nb_files);
    return files[index];
}

Directory** Directory::get_directories() const
{
    return directories;
}

Directory* Directory::get_directory(unsigned long long int index) const
{
    assert(index < nb_directories);
    return directories[index];
}

unsigned long long int Directory::sum_size()
{
    unsigned long long int sum = 0;

    unsigned long long int i;
    for(i=0; i < nb_files; ++i)
        sum += files[i]->get_size();

    for(i=0; i < nb_directories; ++i)
        sum += directories[i]->get_size();

    if(sum > 0) empty = false;

    return sum;
}

/*
* Calcul the total number of files (empty directory count as +1 element)
* in this directory and all his sub-directory
*/
void Directory::sum_elements()
{
    unsigned long long int i;
    for(i=0; i < nb_directories; ++i){
        nb_elements += directories[i]->get_nb_elements();
    }

    nb_elements += nb_files; // each files of a directory == one element.
    ++nb_elements; // the directory himself count as one element (works with recursion)
}
