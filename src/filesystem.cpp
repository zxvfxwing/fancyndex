#include "filesystem.hpp"

FileSystem::FileSystem(fs::path _path)
    :path(_path)
{
    try
    {
        if(fs::exists(path))
        {
            if(path.has_filename())
                name = path.filename().string();
            else
                throw std::runtime_error("No filename found for " + path.root_path().string());

            date_raw = fs::last_write_time(path);
            date_human = this->maketime_readable();
        }
        else
        {
            throw std::runtime_error(path.root_path().string() + " doesn't exists !");
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

unsigned long long int FileSystem::get_size() const
{
    return size;
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
        delete timeinfo;
    }
    catch(std::exception& e)
    {
        std::cerr << "Something went wrong when making the timestamp readable for human : " << e.what() << std::endl;
    }

    return buffer;
}
