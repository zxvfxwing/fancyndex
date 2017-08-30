#include "filesystem.hpp"

FileSystem::FileSystem(std::string path_name)
{
    path = fs::path(path_name);

    try
    {
        if(fs::exists(path))
        {
            name = path.filename().string();
            date_raw = fs::last_write_time(path);
            date_human = this->maketime_readable();

            nb_entries = 0;
            entries = new FileSystem* [nb_entries];

            

            //TODO
            // - Calculate size :
            // size = ?
            // -
        }
        else
        {
            throw std::runtime_error(path_name + " doesn't exists !");
        }
    }
    catch(const fs::filesystem_error& ex)
    {
        std::cerr << ex.what() << std::endl;
    }
}

FileSystem::~FileSystem()
{
    unsigned long long int i;
    for(i=0; i < nb_entries; ++i)
    {
        delete entries[i];
    }
    delete [] entries;
}

std::string FileSystem::get_name() const
{
    return name;
}

std::string FileSystem::get_date_human() const
{
    return date_human;
}

std::time_t FileSystem::get_date_raw() const
{
    return date_raw;
}

unsigned long long int FileSystem::get_size() const
{
    return size;
}

unsigned long long int FileSystem::get_nb_entries() const
{
    return nb_entries;
}

std::string FileSystem::maketime_readable(bool use_localtime)
{
    struct tm* timeinfo;
    char buffer[80];

    try
    {
        if(use_localtime)   timeinfo = localtime(&date_raw);
        else                timeinfo = gmtime(&date_raw);
        strftime(buffer, 80, "%F %T", timeinfo);
        delete timeinfo;
    }
    catch(std::exception& e)
    {
        std::cerr << "Something went wrong when making the timestamp readable for human : " << e.what() << std::endl;
    }

    return buffer;
}
