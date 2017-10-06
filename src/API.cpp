#include "API.hpp"

API::API()
    :dir(NULL), unit(2), path(""), home("")
{
    home = "/var/www/dl/";
}

API::~API()
{
    delete dir;
}

int API::set_path(std::string _path)
{
    path = _path;
    std::string r_path = home + path;
    fs::path p(r_path);

    if(!fs::exists(p)){
        std::cerr << "Path given doesn't exists !" << std::endl;
        return -1;
    }

    dir = new Directory(p);
    return 0;
}

void API::set_unit(unsigned int _unit)
{
    unit = _unit;
}

void API::sort_by_name(bool ascending){
    dir->sort_els_by_name(ascending);
}

void API::sort_by_size(bool ascending){
    dir->sort_els_by_size(ascending);
}

void API::sort_by_date(bool ascending){
    dir->sort_els_by_date(ascending);
}

std::string API::return_answer()
{
    return j.dump();
}

void API::setup_f_JSON(bool type)
{
    unsigned long long int i;
    File* f;

    if( type ){
        for(i=0; i < dir->get_nb_files(); ++i){
            f = dir->get_file(i);

            j["files"][i]["extension"] =    f->get_extension();
            j["files"][i]["date"] =         f->get_date_human();
            j["files"][i]["name"] =         f->get_name();
            j["files"][i]["size"] =         f->get_size_human();
            j["files"][i]["unit"] =         f->get_size_human(unit);
        }
    }
    else{
        for(i=0; i < dir->get_nb_files(); ++i){
            f = dir->get_file(i);

            j["files"][i]["extension"] =    f->get_extension();
            j["files"][i]["date"] =         f->get_date_human();
            j["files"][i]["name"] =         f->get_name();
            j["files"][i]["size"] =         f->get_size_peasant();
            j["files"][i]["unit"] =         f->get_size_peasant(unit);
        }
    }
}

void API::setup_d_JSON(bool type)
{
    unsigned long long int i;
    Directory* d;

    if( type ){
        for(i=0; i < dir->get_nb_directories(); ++i){
            d = dir->get_directory(i);

            j["directories"][i]["date"] =           d->get_date_human();
            j["directories"][i]["name"] =           d->get_name();
            j["directories"][i]["nb_elements"] =    d->get_nb_elements();
            j["directories"][i]["size"] =           d->get_size_human();
            j["directories"][i]["unit"] =           d->get_size_human(unit);
        }
    }
    else{
        for(i=0; i <  dir->get_nb_directories(); ++i){
            d = dir->get_directory(i);

            j["directories"][i]["date"] =           d->get_date_human();
            j["directories"][i]["name"] =           d->get_name();
            j["directories"][i]["nb_elements"] =    d->get_nb_elements();
            j["directories"][i]["size"] =           d->get_size_peasant();
            j["directories"][i]["unit"] =           d->get_size_peasant(unit);
        }
    }
}

void API::setup_JSON()
{
    if( path == "." )
        j["root_name"] = "Home";
    else
        j["root_name"] = dir->get_name();

    j["full_size"] = dir->get_size();
    j["nb_directories"] = dir->get_nb_directories();
    j["nb_files"] = dir->get_nb_files();
    j["nb_elements"] = dir->get_nb_elements();

    setup_d_JSON();
    setup_f_JSON();
}
