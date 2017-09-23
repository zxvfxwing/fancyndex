#include "filesystem.hpp"

std::string ibytes [] = {
    "Byte(s)",
    "KibiByte(s)",
    "MebiByte(s)",
    "GigiByte(s)",
    "TebiByte(s)",
    "ExiByte(s)",
    "ZebiByte(s)",
    "YobiByte(s)"
};

std::string peasant_bytes [] = {
    "Byte(s)",
    "KiloByte(s)",
    "MegaByte(s)",
    "GigaBytes(s)",
    "TebaByte(s)",
    "ExaByte(s)",
    "ZettaByte(s)",
    "YottaByte(s)"
};

FileSystem::FileSystem(fs::path _path)
    :path(_path),
    name(""),
    date_raw(0),
    date_human(""),
    size(0),
    size_unit(ibytes[0]),
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

                if( name[0] == '.' ){
                    dotfile = true;
                }
            }
            else
                throw std::runtime_error("This file or directory has no name !");

            date_raw = fs::last_write_time(path);
            maketime_readable();
        }
        else{
            if(fs::is_symlink(path)){
                throw std::runtime_error("Wrong symbolic link, check Documentation (git).");
            }
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
    return size as str
*/
std::string FileSystem::get_size_str() const
{
    return std::to_string(size);
}

long double FileSystem::get_size_human()
{
    long double size_human = size;
    unsigned int power = 1;

    while ( size_human > 1024.0 )
    {
        size_human /= 1024.0;
        size_unit = ibytes[power++];
    }

    return size_human;
}

long double FileSystem::get_size_peasant()
{
    long double size_human = size;
    unsigned int power = 1;

    while ( size_human > 1000.0 )
    {
        size_human /= 1000.0;
        size_unit = peasant_bytes[power++];
    }

    return size_human;
}

void FileSystem::set_size(const unsigned long long int& _size)
{
    size = _size;
}

void FileSystem::set_size_unit(const std::string & _size_unit)
{
    size_unit = _size_unit;
}

std::string FileSystem::get_size_unit() const
{
    return size_unit;
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

void FileSystem::shell_sort_by_name(FileSystem** fs, unsigned long long int size, bool direction)
{
    unsigned long long int* gaps = new unsigned long long int [size];
    unsigned long long int i, y, k;
    unsigned long long int gaps_size = 0;
    unsigned long long int gap = 1;
    FileSystem* tmp;

    do{
        gaps[gaps_size++] = gap;
        gap = gap*3 + 1;
    } while( gap < size );

    while( gaps_size > 0 ){
        gap = gaps[gaps_size-1];
        for(i=0; i < gap; ++i){
            for(y=i; y < size ; y+=gap){
                for(k=y; k > i; k-=gap){
                    if( direction && boost::to_lower_copy(fs[k]->get_name()) < boost::to_lower_copy(fs[k-gap]->get_name()) ){
                        tmp = fs[k];
                        fs[k] = fs[k-gap];
                        fs[k-gap] = tmp;
                    }
                    else if( !direction && boost::to_lower_copy(fs[k]->get_name()) > boost::to_lower_copy(fs[k-gap]->get_name()) )
                    {
                        tmp = fs[k];
                        fs[k] = fs[k-gap];
                        fs[k-gap] = tmp;
                    }
                    else break;
                }
            }
        }
        --gaps_size;
    }

    delete [] gaps;
}
