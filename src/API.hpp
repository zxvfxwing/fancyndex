#ifndef API_HPP
#define API_HPP

#include "yaml-cpp/yaml.h"
#include "json.hpp"
#include "directory.hpp"

using json = nlohmann::json;

class API
{
private:
    Directory* dir;
    unsigned int unit_mode;
    unsigned int unit_type;
    std::string path;
    std::string home;
    std::string home_name;
    json j;

    bool runAPI;

    void setup_d_JSON();
    void setup_f_JSON();

public:
    API();
    ~API();

    int  set_path(std::string path);

    void sort_by_name(bool ascending=true);
    void sort_by_size(bool ascending=true);
    void sort_by_date(bool ascending=true);

    void setup_JSON();
    void clear_JSON();

    std::string return_answer();
};

#endif //API_HPP
