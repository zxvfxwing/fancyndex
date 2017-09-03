#include "filesystem.hpp"

FileSystem::FileSystem(fs::path _path)
    :path(_path)
{
    try
    {
        if(fs::exists(path))
        {
            if(path.has_filename()){
                name = path.filename().string();
                /*
                * if name == ".", means that we are on the source directory
                * To get his real name, need to call `canonical` function.
                */
                if(name == ".")
                    name = get_canonical();
            }
            else
                throw std::runtime_error("This file or directory has no name !");

            date_raw = fs::last_write_time(path);
            date_human = this->maketime_readable();
        }
        else
        {
            throw std::runtime_error("Path given doesn't exists ...");
        }
    }
    catch(const fs::filesystem_error& ex)
    {
        std::cerr << ex.what() << std::endl;
    }
}

FileSystem::~FileSystem()
{

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

/*
    return size
*/
unsigned long long int FileSystem::get_size() const
{
    return size;
}

/*
    return size
*/
std::string FileSystem::get_size_str() const
{
    return std::to_string(size);
}

void FileSystem::set_size(const unsigned long long int& _size)
{
    size = _size;
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
    }
    catch(std::exception& e)
    {
        std::cerr << "Something went wrong when making the timestamp readable for human : " << e.what() << std::endl;
    }

    return buffer;
}

std::string FileSystem::get_canonical() const
{
    return fs::canonical(path).filename().string();
}