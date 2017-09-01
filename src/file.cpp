#include "file.hpp"

File::File(fs::path file)
    :FileSystem(file)
{
    try
    {
        if(fs::is_regular_file(file))
        {
            if(file.has_extension())
                extension = file.extension().string();
            else
                extension = "";

            FileSystem::set_size( fs::file_size(file) );
        }
        else
        {
            throw std::runtime_error("ERROR: " + file.filename().string() + " is either a directory or not a regular file.\nThis is an instance of the File class !");
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
