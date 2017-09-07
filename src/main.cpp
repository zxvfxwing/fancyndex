#include <silicon/api.hh>
#include <silicon/backends/mhd.hh>
#include "symbols.hh"

#include "json.hpp"
#include "directory.hpp"

using namespace sl; // Silicon namespace
using namespace s; // Symbols namespace

// for convenience
using json = nlohmann::json;

/*
* Make an API classt to serve only json.dump() here
* API :
*   - calcul All without dotfile ;
*   - get human size ;
*   - sort by Name ;
*   - sort by Size ;
*   -
*
*/

// Define the API:
auto filesystem_api = http_api(

    /*
    *   GET:
    *   https://your.domain.name/directory?path=the/path/you/want/to/be/found
    */
    GET / _directory * get_parameters(_path = std::string()) = [] (auto param) {

        std::string home = "/var/www/fancyndex/www";
        std::string r_path = home + param.path;

        fs::path p(r_path);

        if(!fs::exists(p)){
            std::cout << "FAIL" << std::endl;
            throw error::unauthorized("The path ", param.path, " doesn't exists");
        }

        Directory* dir = new Directory(p);
        unsigned long long int i;

        // std::cout << dir->get_absolute() << std::endl;
        json j;

        /*
        * JSON in alphabetic order :
        */

        // DIRECTORY HIMSELF
        j["full_size"] = dir->get_size();
        j["nb_directories"] = dir->get_nb_directories();
        j["nb_files"] = dir->get_nb_files();

        if( dir->get_absolute() == home+"." )
            j["root_name"] = "Home";
        else
            j["root_name"] = dir->get_name();

        j["total_nb_elements"] = dir->get_nb_elements();
        // END DIRECTORY HIMSELF

        // FILES
        for(i=0; i < dir->get_nb_files(); ++i){
            j["files"][i]["extension"] = dir->get_file(i)->get_extension();
            j["files"][i]["date"] = dir->get_file(i)->get_date_human();
            j["files"][i]["name"] = dir->get_file(i)->get_name();
            j["files"][i]["size"] = dir->get_file(i)->get_size();
        }
        // END FILES

        // DIRECTORIES
        for(i=0; i < dir->get_nb_directories(); ++i){
            j["directories"][i]["date"] = dir->get_directory(i)->get_date_human();
            j["directories"][i]["name"] = dir->get_directory(i)->get_name();
            j["directories"][i]["nb_elements"] = dir->get_directory(i)->get_nb_elements();
            j["directories"][i]["size"] = dir->get_directory(i)->get_size();
        }
        // END DIRECTORIES

        // delete Directory instance :
        delete dir;

        // Parse JSON into std::string and return it
        return j.dump();
    },

    GET / _root = [] () {
        json j;
        j["root"] = "/home/spoken/Git/fancyndex/www";
        return j.dump();
    }
);

int main()
{
    // Serve hello_api via microhttpd using the json format:
    sl::mhd_json_serve(filesystem_api, 9099);
}
