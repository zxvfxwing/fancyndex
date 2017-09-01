#include "file.hpp"

File::File(fs::path file)
    :FileSystem(file),
    extension(""),
    extension_ok(false)
{
    try
    {
        if(fs::is_regular_file(file))
        {
            if(file.has_extension()){
                extension = file.extension().string();
                extension_ok = true;
            }
            set_size(fs::file_size(file));
        }
        else
        {
            throw std::runtime_error("ERROR: " + file.filename().string() + " is not a regular file.\n");
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

std::string File::get_extension() const
{
    return extension;
}

bool File::is_extension_ok() const
{
    return extension_ok;
}
