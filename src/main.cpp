#include <silicon/api.hh>
#include <silicon/backends/mhd.hh>
#include "symbols.hh"

using namespace sl; // Silicon namespace
using namespace s;  // Symbols namespace

#include "API.hpp"

API fs_api;

void set_headers(mhd_response* r, std::string AccessControlAO)
{
    r->set_header("Access-Control-Allow-Origin", AccessControlAO);
    r->set_header("Content-Type", "application/json; charset=UTF-8");
}

void path_error_message(std::string wrong_path)
{
    throw error::unauthorized("The path ", wrong_path, " doesn't exists");
}

auto FileSystemAPI = http_api(

    GET / _dir * get_parameters(_mode = int(), _sort = int(), _path = std::string()) = [] (auto param, mhd_response* r)
    {
        set_headers(r, fs_api.HTTP_AccessCHeader());
        int result = fs_api.set_options(param.path, param.sort, bool(param.mode));
        if( result < 0 ) path_error_message(param.path);
        fs_api.setup_JSON();
        return fs_api.answer();
    },

    GET / _archive * get_parameters(_active_path = std::string(), _list = std::string()) = [] (auto param, mhd_response* r)
    {
        bool good = true;
        std::string archive_time = "";
        std::string archive_path = "";
        std::string archive_absolute_path = "";
        std::string archive_list = "";
        std::string cmd = "";

        set_headers(r, fs_api.HTTP_AccessCHeader());

        if( param.active_path == "." || param.active_path == "./" )
            param.active_path = "";

        /*
        * Verify that files & folders exist on system.
        */
        std::size_t pos;
        std::string name = "";
        std::string full_path = "";

        unsigned long long int i = 0;
        while( i < param.list.length() ){

            /* GET list, separated by commas */
            pos = param.list.find(",", i);
            name = param.list.substr(i, pos-i);

            /* Verification on system */
            full_path = fs_api.HOME() + param.active_path + name;
            fs::path p( full_path );

            /* If not exists, no download here */
            if( !fs::exists(p) || name == "." || name == "./" || name == ".." ){
                good = false;
                break;
            }

            i += name.length()+1;
            archive_list += "\""+ full_path + "\" ";
        }

        /* 7z Archive part */
        if( good ){
            archive_time = std::to_string( time(0) );
            archive_path = "fancyndex/archive/" + archive_time + ".7z";
            archive_absolute_path = fs_api.HOME() + archive_path;
	    

	    /* Create an archive without compression, juste copy documents inside it */ 
	    cmd = "7z a -mx=1 " +  archive_absolute_path + " " + archive_list + " 2>/dev/null 1>/dev/null";
            system(cmd.c_str());
        }

        json j;
        j["ok"] = good;
        j["archive_path"] = archive_path;

        return j.dump();
    }
);

/* Serve filesystem API via microhttpd using the json format */
int main()
{
    mhd_json_serve(FileSystemAPI, fs_api.PORT());
}
