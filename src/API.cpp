#include "API.hpp"

API::API()
    :dir(NULL)
{
    YAML::Node config = YAML::LoadFile("/var/www/fancyndex/conf/config.yaml");

    /* root_path */
    if( config["exe"]["root_path"] ) home = config["exe"]["root_path"].as<std::string>();
    else throw std::runtime_error("There is no default option for `root_path` in config.yaml\n   Please modify your config file.");

    /* root_name */
    if( config["exe"]["root_name"] ) home_name = config["exe"]["root_name"].as<std::string>();
    else home_name = "Home";

    /* mode used to display size unit */
    if( config["exe"]["unit_mode"] ) unit_mode = config["exe"]["unit_mode"].as<unsigned int>();
    else unit_mode = 1;

    /* Acronym or fullsize when displaying size unit */
    if( config["exe"]["unit_type"] ) unit_type = config["exe"]["unit_type"].as<unsigned int>();
    else unit_type = 2;
    if( unit_type == 0 ) unit_type = 2;
}

API::~API()
{
    delete dir;
}

int API::set_path(std::string _path)
{
    /*
    * Check if it's just a page refresh of the API with the same path.
    * If it's true, we are just going to dump() once again json without sorting and parsing.
    */
    if( _path == path ){
        runAPI = false;
        return 1;
    }

    if( dir != NULL ){
        clear_JSON();
        delete dir;
        dir = NULL;
    }

    path = _path;
    std::string r_path = home + path;
    fs::path p(r_path);

    if(!fs::exists(p)){
        std::cerr << "Path given doesn't exists !" << std::endl;
        return -1;
    }

    dir = new Directory(p);
    runAPI = true;
    return 0;
}

void API::sort_by_name(bool ascending){
    if( runAPI ) dir->sort_els_by_name(ascending);
}

void API::sort_by_size(bool ascending){
    if( runAPI ) dir->sort_els_by_size(ascending);
}

void API::sort_by_date(bool ascending){
    if( runAPI ) dir->sort_els_by_date(ascending);
}

std::string API::return_answer()
{
    return j.dump();
}

void API::clear_JSON(){
    j.clear();
}

void API::setup_f_JSON()
{
    unsigned long long int i;
    File* f;

    for(i=0; i < dir->get_nb_files(); ++i){
        f = dir->get_file(i);

        j["files"][i]["extension"] =    f->get_extension();
        j["files"][i]["date"] =         f->get_date_human();
        j["files"][i]["name"] =         f->get_name();
        if( unit_mode == 1 ){
            j["files"][i]["size"] =     f->get_size_human();
            j["files"][i]["unit"] =     f->get_size_human(unit_type);
        }
        else{
            j["files"][i]["size"] =     f->get_size_peasant();
            j["files"][i]["unit"] =     f->get_size_peasant(unit_type);
        }
    }
}

void API::setup_d_JSON()
{
    unsigned long long int i;
    Directory* d;

    for(i=0; i < dir->get_nb_directories(); ++i){
        d = dir->get_directory(i);

        j["directories"][i]["date"] =           d->get_date_human();
        j["directories"][i]["name"] =           d->get_name();
        j["directories"][i]["nb_elements"] =    d->get_nb_elements();
        if( unit_mode == 1 ){
            j["directories"][i]["size"] =       d->get_size_human();
            j["directories"][i]["unit"] =       d->get_size_human(unit_type);
        }
        else{
            j["directories"][i]["size"] =       d->get_size_peasant();
            j["directories"][i]["unit"] =       d->get_size_peasant(unit_type);
        }
    }
}

void API::setup_JSON()
{
    if( runAPI ){
        if( path == "." )
            j["root_name"] = home_name;
        else
            j["root_name"] = dir->get_name();

        j["full_size"] = dir->get_size();
        j["nb_directories"] = dir->get_nb_directories();
        j["nb_files"] = dir->get_nb_files();
        j["nb_elements"] = dir->get_nb_elements();

        setup_d_JSON();
        setup_f_JSON();
    }
}
