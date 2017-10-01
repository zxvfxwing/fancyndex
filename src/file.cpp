#include "file.hpp"

File::File(fs::path file)
    :FileSystem(file),
    extension(""),
    extension_ok(false)
{
    try {
        if(fs::is_regular_file(file)){
            if(file.has_extension()){
                std::string name = get_name();
                /*
                *   If name begin with a '.', "has_extension()" function has mistaken.
                *   Indeed, path.extension().string() == path.filename().string() here.
                *   It's part of our job to fix it :
                */
                if( is_dotfile() ){
                    /*
                    * Find last occurence of dot in the dotfile to get the actual extension
                    * Begin from string last character.
                    */
                    unsigned int found = name.find_last_of(".");

                    /*
                    * If last '.' is == index 0, means that is a dotfile without extension (i.e: ".XResources).
                    * If last '.' is not index 0, means that we have to find the real extension (i.e: .config.js):
                    */
                    if(found != 0){
                        extension = name.substr(found);
                        extension_ok = true;
                    }
                }
                else{
                    extension = file.extension().string();
                    extension_ok = true;
                }
            }

            set_size(fs::file_size(file));
        }
        else{
            throw std::runtime_error("ERROR: " + file.filename().string() + " is not a regular file.\n");
        }
    }
    catch(const fs::filesystem_error& e){
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


/*bool File::_ascending_name(File* f1, File* f2)
{
    return ( boost::to_lower_copy(f1->get_name()) < boost::to_lower_copy(f2->get_name()) );
}

bool File::_decreasing_name(File* f1, File* f2)
{
    return ( boost::to_lower_copy(f1->get_name()) > boost::to_lower_copy(f2->get_name()) );
}
*/
