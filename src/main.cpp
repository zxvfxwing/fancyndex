#include <silicon/api.hh>
#include <silicon/backends/mhd.hh>
#include "symbols.hh"

#include "json.hpp"
#include "directory.hpp"

using namespace sl; // Silicon namespace
using namespace s; // Symbols namespace

// for convenience
using json = nlohmann::json;

// Define the API:
auto filesystem_api = http_api(

    GET / _directory * get_parameters(_path = std::string())   = [] (auto param) {

        if(param.path == "__home__")
            param.path = ".";

        std::cout << param.path << std::endl;

        fs::path p(param.path);

        if(!fs::exists(p))
            throw error::unauthorized("The path ", param.path, " doesn't exists");

        Directory* dir = new Directory(p);
        unsigned long long int i;

        json j;

        j["root_name"] = dir->get_name();
        j["full_size"] = dir->get_size();

        for(i=0; i < dir->get_nb_files(); ++i){
            j["files"][i]["name"] = dir->get_file(i)->get_name();
            j["files"][i]["size"] = dir->get_file(i)->get_size();
        }

        for(i=0; i < dir->get_nb_directories(); ++i){
            j["directories"][i]["name"] = dir->get_directory(i)->get_name();
            j["directories"][i]["size"] = dir->get_directory(i)->get_size();
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
