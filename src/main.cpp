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

        std::string home = "/var/www/";
        fs::path p(home + param.path);

        if(!fs::exists(p))
            throw error::unauthorized("The path ", param.path, " doesn't exists");

        Directory* dir = new Directory(p);
        unsigned long long int i;

        json j;

        if( dir->get_absolute() == home+"." )
            j["root_name"] = "Home";
        else
            j["root_name"] = dir->get_name();


        j["full_size"] = dir->get_size();
        j["total_nb_elements"] = dir->get_nb_elements();
        j["nb_files"] = dir->get_nb_files();
        j["nb_directories"] = dir->get_nb_directories();

        for(i=0; i < dir->get_nb_files(); ++i){
            j["files"][i]["name"] = dir->get_file(i)->get_name();
            j["files"][i]["size"] = dir->get_file(i)->get_size();
            j["files"][i]["date"] = dir->get_file(i)->get_date_human();
            j["files"][i]["extension"] = dir->get_file(i)->get_extension();
        }

        for(i=0; i < dir->get_nb_directories(); ++i){
            j["directories"][i]["name"] = dir->get_directory(i)->get_name();
            j["directories"][i]["size"] = dir->get_directory(i)->get_size();
            j["directories"][i]["date"] = dir->get_directory(i)->get_date_human();
            j["directories"][i]["nb_elements"] = dir->get_directory(i)->get_nb_elements();
        }

        delete dir;
        return j.dump();
    }
);

int main()
{
    // Serve hello_api via microhttpd using the json format:
    sl::mhd_json_serve(filesystem_api, 9099);
}
