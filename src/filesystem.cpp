#include "filesystem.hpp"

const std::string ibytes[NB_UNITS] =
{
    "Byte(s)",      "KibiByte(s)", "MebiByte(s)",
    "GibiByte(s)",  "TebiByte(s)", "PebiBytes(s)",
    "ExbiByte(s)",  "ZebiByte(s)", "YobiByte(s)"
};

const std::string bytes[NB_UNITS] =
{
    "Byte(s)",      "KiloByte(s)",  "MegaByte(s)",
    "GigaBytes(s)", "TeraByte(s)",  "PetaByte(s)",
    "ExaByte(s)",   "ZettaByte(s)", "YottaByte(s)"
};

const std::string ibytes_acro[NB_UNITS] =
{ "B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB" };

const std::string bytes_acro[NB_UNITS] =
{ "B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB" };


FileSystem::FileSystem(fs::path _path)
    :path(_path),
    name(""),
    date_raw(0),
    date_human(""),
    size(0),
    dotfile(false)
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
{ }

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

std::string FileSystem::get_size_human(bool mode) const
{
    long double dsize = size;
    unsigned int power = 0;
    unsigned int wanted_precision = 3;
    unsigned int precision = 0;

    while( dsize > 1024.0 ){
        ++power;
        dsize /= 1024.0;
    }

    unsigned long long int test_int = dsize;

    if( test_int < dsize )
        precision = wanted_precision;

    std::string str_size = std::to_string(dsize);
    str_size = str_size.substr(0, str_size.find_first_of(".") + precision );

    if( mode ) return str_size + " " + ibytes[power] ;
    return str_size + " " + ibytes_acro[power] ;
}

std::string FileSystem::get_size_peasant(bool mode) const
{
    long double dsize = size;
    unsigned int power = 0;
    unsigned int wanted_precision = 3;
    unsigned int precision = 0;

    while( dsize > 1000.0 ){
        ++power;
        dsize /= 1000.0;
    }

    unsigned long long int test_int = dsize;

    if( test_int < dsize )
        precision = wanted_precision;

    std::string str_size = std::to_string(dsize);
    str_size = str_size.substr(0, str_size.find_first_of(".") + precision );

    if( mode ) return str_size + " " + ibytes[power] ;
    return str_size + " " + ibytes_acro[power] ;
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
