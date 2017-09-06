#include "symlink.hpp"

Symlink::Symlink(fs::path path)
    :FileSystem(path),
    directory(NULL),
    file(NULL)
{
    try {
        if(fs::is_symlink(path)){
            sym_path = fs::read_symlink(path);

            // Symlink == file
            if(get_type()){
                file = new File(sym_path);
                set_size(file->get_size());
            }
            // Symlink == directory
            else{
                directory = new Directory(sym_path);
                set_size(directory->get_size());
            }
        }
    }
    catch(const fs::filesystem_error& ex){
        std::cerr << ex.what() << std::endl;
    }
}

Symlink::~Symlink()
{
    if( directory != NULL ) delete directory;
    if( file != NULL ) delete file;
}

fs::path Symlink::get_path() const
{
    return sym_path;
}
