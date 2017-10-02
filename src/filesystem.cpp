#include "filesystem.hpp"

FileSystem::FileSystem(fs::path _path)
    :path(_path),
    name(""),
    date_raw(0),
    date_human(""),
    size(0),
    dotfile(false),
    ibytes(NULL),
    bytes(NULL),
    ibytes_acro(NULL),
    bytes_acro(NULL)
{
    try {
        if(fs::exists(path)){
            if(path.has_filename()){
                name = path.filename().string();
                /*
                * if name == ".", means that we are on the source directory
                * To get his real name, need to call `canonical` function.
                */
                if(name == ".")
                    name = get_canonical_name();

                if( name[0] == '.' )
                    dotfile = true;
            }
            else
                throw std::runtime_error("This file or directory has no name !");

            date_raw = fs::last_write_time(path);
            maketime_readable();
        }
        else{
            if(fs::is_symlink(path))
                throw std::runtime_error("Wrong symbolic link, check Documentation (git).");
            else
                throw std::runtime_error("Path given doesn't exists ...");
        }
    }
    catch(const fs::filesystem_error& ex){
        std::cerr << ex.what() << std::endl;
    }
}

FileSystem::~FileSystem()
{
    delete [] ibytes;   delete [] ibytes_acro;
    delete [] bytes;    delete [] bytes_acro;
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

long double FileSystem::get_size_human() const
{
    long double size_human = size;
    unsigned int power = 1;

    while ( size_human > 1024.0 )
    {
        size_human /= 1024.0;
        //size_unit = ibytes[power++];
    }

    return size_human;
}

long double FileSystem::get_size_peasant() const
{
    long double size_human = size;
    unsigned int power = 1;

    while ( size_human > 1000.0 )
    {
        size_human /= 1000.0;
        //size_unit = peasant_bytes[power++];
    }

    return size_human;
}

void FileSystem::set_size(const unsigned long long int& _size)
{
    size = _size;
}

void FileSystem::maketime_readable(bool use_localtime)
{
    struct tm* timeinfo;
    char buffer[80];

    try{
        if(use_localtime)   timeinfo = localtime(&date_raw);
        else                timeinfo = gmtime(&date_raw);
        strftime(buffer, 80, "%F %T", timeinfo);
    }
    catch(std::exception& e){
        std::cerr << "Something went wrong when making the timestamp readable for human : " << e.what() << std::endl;
    }

    date_human = buffer;
}

std::string FileSystem::get_canonical() const
{
    return fs::canonical(path).string();
}

std::string FileSystem::get_canonical_name() const
{
    return fs::canonical(path).filename().string();
}

std::string FileSystem::get_absolute() const
{
    return fs::absolute(path).string();
}

bool FileSystem::is_dotfile() const
{
    return dotfile;
}

void FileSystem::init_size_units_str()
{
    ibytes = new const std::string[NB_UNITS]
    {
        "Byte(s)",      "KibiByte(s)", "MebiByte(s)",
        "GibiByte(s)",  "TebiByte(s)", "PebiBytes(s)",
        "ExbiByte(s)",  "ZebiByte(s)", "YobiByte(s)"
    };

    bytes = new const std::string[NB_UNITS]
    {
        "Byte(s)",      "KiloByte(s)",  "MegaByte(s)",
        "GigaBytes(s)", "TeraByte(s)",  "PetaByte(s)",
        "ExaByte(s)",   "ZettaByte(s)", "YottaByte(s)"
    };

    ibytes_acro = new const std::string[NB_UNITS]
    { "B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB" };

    bytes_acro = new const std::string[NB_UNITS]
    { "B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB" };
}


bool FileSystem::by_name_ascending(FileSystem* f1, FileSystem* f2)
{
    return ( boost::to_lower_copy(f1->get_name()) < boost::to_lower_copy(f2->get_name()) );
}

bool FileSystem::by_name_decreasing(FileSystem* f1, FileSystem* f2)
{
    return ( boost::to_lower_copy(f1->get_name()) > boost::to_lower_copy(f2->get_name()) );
}

bool FileSystem::by_size_ascending(FileSystem* f1, FileSystem* f2)
{
    return ( f1->get_size() < f2->get_size() );
}

bool FileSystem::by_size_decreasing(FileSystem* f1, FileSystem* f2)
{
    return ( f1->get_size() > f2->get_size() );
}

void FileSystem::sort_(FileSystem** fs_array, unsigned long long int size, unsigned int mode)
{
    switch(mode){
        case 0: std::sort(fs_array, fs_array + size, by_name_ascending);    break;
        case 1: std::sort(fs_array, fs_array + size, by_name_decreasing);   break;
        case 2: std::sort(fs_array, fs_array + size, by_size_ascending);    break;
        case 3: std::sort(fs_array, fs_array + size, by_size_decreasing);   break;
    }
}
