#include <cstdlib>
#include <ctime>
#include <thread>
#include <string>
#include <fstream>

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
*/

class API
{
private:
    std::string path;
    unsigned int unit;
    Directory* dir;
    json j;

public:
    API(std::string _path)
        :path(_path), unit(2), dir(NULL)
    {
        std::string home = "/var/www/";
        std::string r_path = home + path;

        fs::path p(r_path);

        if(!fs::exists(p)){
            std::cout << "Path given doesn't exists !" << std::endl;
            throw error::unauthorized("The path ", path, " doesn't exists");
        }

        dir = new Directory(p);
    }

    ~API()
    {
        delete dir;
    }

    void set_unit(unsigned int _unit)
    {
        unit = _unit;
    }

    void set_header(mhd_response* r){
        r->set_header("Access-Control-Allow-Origin", "*");
        r->set_header("Content-Type", "application/json; charset=UTF-8");
    }

    void sort_by_name(bool ascending){
        dir->sort_els_by_name(ascending);
    }

    void sort_by_size(bool ascending){
        dir->sort_els_by_size(ascending);
    }

    void sort_by_date(bool ascending){
        dir->sort_els_by_date(ascending);
    }

    std::string return_answer()
    {
        return j.dump();
    }

    void setup_JSON()
    {
        unsigned long long int nb_dirs;
        unsigned long long int nb_files;
        unsigned long long int i;

        File* f;
        Directory* d;

        nb_dirs = dir->get_nb_directories();
        nb_files = dir->get_nb_files();

        if( path == "." )
            j["root_name"] = "Home";
        else
            j["root_name"] = dir->get_name();

        j["full_size"] = dir->get_size();
        j["nb_directories"] = nb_dirs;
        j["nb_files"] = nb_files;
        j["nb_elements"] = dir->get_nb_elements();

        // FILES
        for(i=0; i < nb_files; ++i){
            f = dir->get_file(i);

            j["files"][i]["extension"] =    f->get_extension();
            j["files"][i]["date"] =         f->get_date_human();
            j["files"][i]["name"] =         f->get_name();
            j["files"][i]["size"] =         f->get_size_human();
            j["files"][i]["unit"] =         f->get_size_human(unit);
        }
        // END FILES

        // DIRECTORIES
        for(i=0; i < nb_dirs; ++i){
            d = dir->get_directory(i);

            j["directories"][i]["date"] =           d->get_date_human();
            j["directories"][i]["name"] =           d->get_name();
            j["directories"][i]["nb_elements"] =    d->get_nb_elements();
            j["directories"][i]["size"] =           d->get_size_human();
            j["directories"][i]["unit"] =           d->get_size_human(unit);
        }
        // END DIRECTORIES
    }
};


// Define the API:
auto filesystem_api = http_api(

    GET / _dir / _by_name * get_parameters(_mode = int(), _path = std::string()) = [] (auto param, mhd_response* r) {
        API api(param.path);
        api.set_header(r);
        if(param.mode == 1) api.sort_by_name(true);
        else if(param.mode == 0) api.sort_by_name(false);
        api.setup_JSON();
        return api.return_answer();
    },

    GET / _dir / _by_size * get_parameters(_mode = int(), _path = std::string()) = [] (auto param, mhd_response* r) {
        API api(param.path);
        api.set_header(r);
        if(param.mode == 1) api.sort_by_size(true);
        else if(param.mode == 0) api.sort_by_size(false);
        api.setup_JSON();
        return api.return_answer();
    },

    GET / _dir / _by_date * get_parameters(_mode = int(), _path = std::string()) = [] (auto param, mhd_response* r) {
        API api(param.path);
        api.set_header(r);
        if(param.mode == 1) api.sort_by_date(true);
        else if(param.mode == 0) api.sort_by_date(false);
        api.setup_JSON();
        return api.return_answer();
    }
);

int main()
{
    // Serve hello_api via microhttpd using the json format:
    sl::mhd_json_serve(filesystem_api, 9099);
}
