#include <silicon/api.hh>
#include <silicon/backends/mhd.hh>
#include "symbols.hh"

using namespace sl; // Silicon namespace
using namespace s;  // Symbols namespace

#include "API.hpp"

API fs_api;

void set_headers(mhd_response* r, std::string AccessCAOrigin)
{
    r->set_header("Access-Control-Allow-Origin", AccessCAOrigin);
    r->set_header("Content-Type", "application/json; charset=UTF-8");
}

void path_error_message(std::string wrong_path)
{
    throw error::unauthorized("The path ", wrong_path, " doesn't exists");
}

auto filesystem_api = http_api(

    GET / _dir * get_parameters(_mode = int(), _sort = int(), _path = std::string()) = [] (auto param, mhd_response* r)
    {
        set_headers(r, fs_api.HTTP_AccessCHeader());
        int result = fs_api.set_options(param.path, param.sort, bool(param.mode));
        if( result < 0 ) path_error_message(param.path);
        fs_api.setup_JSON();
        return fs_api.return_answer();
    }

);

/* Serve filesystem API via microhttpd using the json format */
int main()
{
    mhd_json_serve(filesystem_api, fs_api.PORT());
}
