#include "file.hpp"

File::File(fs::path _file)
    :file(_file)
{
    try
    {
        if(fs::exists(file))
        {
            if(fs::is_regular_file(file))
            {
                if(file.has_filename())
                    filename = file.filename().string();
                else
                    throw std::runtime_error("ERROR: file doesn't even has a name !");

                if(file.has_extension())
                    extension = file.extension().string();
                else
                    extension = "";

                size = fs::file_size(file);
                date_raw = fs::last_write_time(file);
                date_human = maketime_readable(date_raw);
            }
            else
            {
                throw std::runtime_error("ERROR: " + file.filename().string() + " is either a directory or not a regular file.\nThis is an instance of the File class !");
            }
        }
        else
        {
            throw std::runtime_error("ERROR: " + file.filename().string() + " doesn't even exists !");
        }
    }
    catch(const fs::filesystem_error& e)
    {
        std::cerr << e.what() << std::endl;
    }
}

File::~File()
{

}


std::string File::maketime_readable(std::time_t raw, bool use_localtime)
{
    struct tm* timeinfo;
    char buffer[80];

    try
    {
        if(use_localtime)   timeinfo = localtime(&raw);
        else                timeinfo = gmtime(&raw);
        strftime(buffer, 80, "%F %T", timeinfo);
        delete timeinfo;
    }
    catch(std::exception& e)
    {
        std::cerr << "Something went wrong during maketime_readable(std::time_t raw) :" << e.what() << std::endl;
    }

    return buffer;
}

std::string File::get_name() const
{
    return filename;
}

std::string File::get_extension() const
{
    return extension;
}

std::string File::get_date_human() const
{
    return date_human;
}

std::time_t File::get_date_raw() const
{
    return date_raw;
}

unsigned long long int File::get_size() const
{
    return size;
}
