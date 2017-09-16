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

unsigned int delay = 180;
time_t timer = time(NULL);
time_t next_timer = timer;

void make_speedtest(){
    timer = time(NULL);

    if( timer >= next_timer ){
        next_timer = timer + delay;

        unsigned int i;
        for(i=0; i < 10; ++i){
            // count how many iperf3 command are running, 1 is default (one line counted with `wc -l`, the title one) -> 0 running.
            system("ps -C iperf3 | wc -l > /home/spoken/Git/fancyndex/conf/iperf3_running.txt");

            int is_running;
            std::ifstream iperf3_file;
            iperf3_file.open("/home/spoken/Git/fancyndex/conf/iperf3_running.txt");
            iperf3_file >> is_running;
            iperf3_file.close();

            // upload to ping.online.net (ipv4) during 20 seconds, get output in json format :
            if( is_running == 1 ){
                std::string command = "iperf3 --client ping.online.net --port 520"+std::to_string(i)+" --time 20 --json > /home/spoken/Git/fancyndex/conf/speedtest.js &";
                system(command.c_str());
                break;
            }
        }
    }
}


/*
* Make an API classt to serve only json.dump() here
* API :
*   - calcul All without dotfile ;
*   - get human size ;
*   - sort by Name ;
*   - sort by Size ;
*   -
*/

// Define the API:
auto filesystem_api = http_api(

    /*
    *   GET:
    *   https://your.domain.name/directory?path=the/path/you/want/to/be/found
    */
    GET / _directory * get_parameters(_path = std::string()) = [] (auto param, mhd_response* r) {

        std::string home = "/var/www/";
        std::string r_path = home + param.path;

        fs::path p(r_path);

        if(!fs::exists(p)){
            std::cout << "Path given doesn't exists !" << std::endl;
            throw error::unauthorized("The path ", param.path, " doesn't exists");
        }

        std::thread t_one(make_speedtest);
        Directory* dir = new Directory(p);
        t_one.join();

        unsigned long long int i;
        json j;

        /*
        * JSON in alphabetic order :
        */

        // DIRECTORY HIMSELF
        j["full_size"] = dir->get_size();
        j["nb_directories"] = dir->get_nb_directories();
        j["nb_files"] = dir->get_nb_files();

        if(  param.path == "." )
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

        // Needed HTTP header :
        r->set_header("Access-Control-Allow-Origin", "*");
        r->set_header("Content-Type", "application/json; charset=UTF-8");

        // Parse JSON into std::string and return it
        return j.dump();
    }
);

int main()
{
    // Serve hello_api via microhttpd using the json format:
    sl::mhd_json_serve(filesystem_api, 9099);
}
