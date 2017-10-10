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
    std::string path;
    std::string home;
    std::string home_name;
    unsigned int unit_mode;
    unsigned int unit_type;
    unsigned int port;
    unsigned int sort_kind;
    bool ascending;
    bool show_hidden;
    std::string http_ac;

    bool runAPI;
    json j;

    void setup_d_JSON();
    void setup_f_JSON();

    void sort_by_name();
    void sort_by_size();
    void sort_by_date();

public:
    API();
    ~API();

    int  set_options(std::string _path, unsigned int _sort_kind=0, bool _ascending=true);
    void setup_JSON();
    void clear_JSON();

    unsigned int PORT() const;
    std::string HOME() const;
    std::string HTTP_AccessCHeader() const;

    std::string answer() const;
};

#endif //API_HPP
