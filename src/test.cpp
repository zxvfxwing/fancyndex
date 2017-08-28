#include <string>
#include <iostream>
#include <boost/filesystem.hpp>
#include <time.h>

namespace fs = boost::filesystem;

/*
int main(){
    fs::path p(".");

    try{
        // Check if path exists
        if(fs::exists(p)){
            std::cout << "Found !" << std::endl;

            // Check if path is a regular file or a directory
            if(fs::is_regular_file(p))
                std::cout << p << " size is " << fs::file_size(p) << std::endl;

            else if(fs::is_directory(p)){
                std::cout << p << " is a fcking directory !" << std::endl;

                // If it's a directory, list every entries
                for(fs::directory_entry& entry: fs::directory_iterator(p)){
                    // Check if entry is a also a directory
                    if(fs::is_directory(entry)){
                        std::cout <<"       "<< entry.path().filename().string() <<" is a directory too" << std::endl;
                        if(fs::is_empty(entry))
                            std::cout <<"       "<< entry.path().filename().string() << " is an empty directory !" << std::endl;
                    }

                    else
                        // Show name of the file, and size
                        std::cout <<"       "<< entry.path().filename().string() << " -- " << fs::file_size(entry) << " bit(s)" << std::endl;

                    //std::cout << fs::last_write_time(entry.path()) << std::endl;

                    // DATE DE DERNIERE ECRITURE
                    // http://www.cplusplus.com/reference/ctime/strftime/
                    std::time_t rawtime = fs::last_write_time(entry.path());
                    struct tm* timeinfo;
                    char buffer [40];
                    timeinfo = localtime (&rawtime);
                    strftime (buffer, 40,"          -> %F %T.",timeinfo);
                    std::cout << buffer << std::endl;
                }
            }
            else
                std::cout << p << " exists but is something else !" << std::endl;
        }
        else
            std::cout << "Not Found !" << std::endl;
    }

    catch(const fs::filesystem_error& ex){
        std::cerr << ex.what() << std::endl;
    }

    return 0;
}

*/
