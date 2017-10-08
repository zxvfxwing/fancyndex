#include <silicon/api.hh>
#include <silicon/backends/mhd.hh>
#include "symbols.hh"

using namespace sl; // Silicon namespace
using namespace s;  // Symbols namespace

#include "API.hpp"

API fs_api;

#define PORT 9099

void set_headers(mhd_response* r)
{
    r->set_header("Access-Control-Allow-Origin", "*");
    r->set_header("Content-Type", "application/json; charset=UTF-8");
}

void path_error_message(std::string wrong_path)
{
    throw error::unauthorized("The path ", wrong_path, " doesn't exists");
}

auto filesystem_api = http_api(

    /* Sort by name */
    GET / _dir / _by_name * get_parameters(_mode = int(), _path = std::string()) = [] (auto param, mhd_response* r) {
        set_headers(r);
        if( fs_api.set_path(param.path) < 0 ) path_error_message(param.path);
        if( param.mode == 1 ) fs_api.sort_by_name(true);
        else
        if( param.mode == 0 ) fs_api.sort_by_name(false);
        fs_api.setup_JSON();
        return fs_api.return_answer();
    },

    /* Sort by size */
    GET / _dir / _by_size * get_parameters(_mode = int(), _path = std::string()) = [] (auto param, mhd_response* r) {
        set_headers(r);
        if( fs_api.set_path(param.path) < 0 ) path_error_message(param.path);
        if( param.mode == 1 ) fs_api.sort_by_size(true);
        else
        if( param.mode == 0 ) fs_api.sort_by_size(false);
        fs_api.setup_JSON();
        return fs_api.return_answer();
    },

    /* Sort by date */
    GET / _dir / _by_date * get_parameters(_mode = int(), _path = std::string()) = [] (auto param, mhd_response* r) {
        set_headers(r);
        if( fs_api.set_path(param.path) < 0 ) path_error_message(param.path);
        if( param.mode == 1 ) fs_api.sort_by_date(true);
        else
        if( param.mode == 0 ) fs_api.sort_by_date(false);
        fs_api.setup_JSON();
        return fs_api.return_answer();
    }
);

/* Serve filesystem API via microhttpd using the json format */
int main()
{
    mhd_json_serve(filesystem_api, PORT);
}
