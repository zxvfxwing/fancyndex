#include "directory.hpp"

Directory::Directory(fs::path directory)
    :FileSystem(directory),
    empty(true),
    nb_files(0),
    nb_directories(0),
    files(NULL),
    directories(NULL)
{
    try{
        if(fs::is_directory(directory))
        {
            run_directory(directory);
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

// TODO:
// Créer un nouveau tableau de la bonne taille
// Copier l'actuel dans le nouveau
// Ajouter le nouveau fichier dans le nouveau tableau
// Remplacer le pointeur
// détruire l'ancien tableau

void Directory::add_a_file(fs::path path_file)
{
    File** array = new File* [++nb_files];

    if(nb_files > 1){
        unsigned long long int i;
        for(i=0; i < nb_files-1; ++i)
            array[i] = files[i];
    }

    files = array;
    files[nb_files-1] = new File(path_file);
}

void Directory::run_directory(fs::path dir)
{
    try{
        for(fs::directory_entry& entry: fs::directory_iterator(dir))
        {
            if(fs::is_directory(entry)){ }
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

File** Directory::get_files() const
{
    return files;
}

File* Directory::get_file(unsigned long long int index) const
{
    if( index < 0 || index > nb_files){
        throw std::range_error("Wrong index value, check range.");
    }
    return files[index];
}
