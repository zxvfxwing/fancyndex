#include "directory.hpp"

/* Directory::Directory(fs::path _directory)
    :directory(_directory)
{
    try{
        if(fs::exists(directory))
        {
            if(fs::is_directory(directory))
            {
                if(directory.has_filename())
                    name = directory.filename().string();
                else
                    throw std::runtime_error("ERROR: directory doesn't even has a name !");

                nb_files = 0;
                nb_directories = 0;
                empty = true;

                files = new File* [nb_files];
                directories = new Directory* [nb_directories];

                this->run_directory();
                this->total_size();
            }
            else
            {
                throw std::runtime_error("ERROR: " + directory.filename().string() + " is not a directory !\nThis is an instance of the Directory class !");
            }
        }
        else
        {
            throw std::runtime_error("ERROR: " + directory.filename().string() + " doesn't even exists !");
        }
    }
    catch(const fs::filesystem_error& e)
    {
        std::cerr << e.what() << std::endl;
    }
}

Directory::~Directory()
{
    this->delete_files();
    this->delete_directories();
}

void Directory::add_file(fs::path newFile)
{
    File** tmp_tab = new File* [nb_files];

    unsigned long long int i;
    for(i=0; i < nb_files; ++i)
    {
        tmp_tab[i] = files[i];
    }

    ++nb_files;
    files = new File* [nb_files];

    for(i=0; i < nb_files-1; ++i)
    {
        files[i] = tmp_tab[i];
        delete tmp_tab[i];
    }
    delete [] tmp_tab;

    File* _file = new File(newFile);
    files[nb_files] = _file;
}

void Directory::add_directory(fs::path newDir)
{
    Directory** tmp_tab = new Directory* [nb_directories];

    unsigned long long int i;
    for(i=0; i < nb_directories; ++i)
    {
        tmp_tab[i] = directories[i];
    }

    ++nb_directories;
    directories = new Directory* [nb_directories];

    for(i=0; i < nb_directories-1; ++i)
    {
        directories[i] = tmp_tab[i];
        delete tmp_tab[i];
    }
    delete [] tmp_tab;

    Directory* _directory = new Directory(newDir);
    directories[nb_directories] = _directory;
}

void Directory::delete_files()
{
    unsigned long long int i;
    for(i=0; i < nb_files; ++i)
    {
        delete files[i];
    }
    delete [] files;
}

void Directory::delete_directories()
{
    unsigned long long int i;
    for(i=0; i < nb_directories; ++i)
    {
        delete directories[i];
    }
    delete [] directories;
}

bool Directory::is_empty() const
{
    return empty;
}

void Directory::run_directory()
{
    try
    {
        for(fs::directory_entry& entry: fs::directory_iterator(directory))
        {
            if(fs::is_directory(entry))
            {
                if(!fs::is_empty(entry))
                {
                    if(this->is_empty())
                    {
                        this->empty = false;
                    }
                    this->add_directory(entry);
                }
            }
            else if(fs::is_regular_file(entry))
            {
                if(this->is_empty())
                {
                    this->empty = false;
                }
                this->add_file(entry);
            }
        }
    }
    catch(const fs::filesystem_error& e)
    {
        std::cerr << e.what() << std::endl;
    }
}

void Directory::total_size()
{
    unsigned long long int i;
    for(i=0; i < nb_directories; ++i){
        size += directories[i]->get_size();
    }

    for(i=0; i < nb_files; ++i)
    {
        size += files[i]->get_size();
    }
}

unsigned long long int Directory::get_size() const
{
    return size;
}

*/
