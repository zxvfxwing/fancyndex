#ifndef API_HPP
#define API_HPP

#include "json.hpp"
#include "directory.hpp"

using json = nlohmann::json;

class API
{
private:
    Directory* dir;
    unsigned int unit;
    std::string path;
    std::string home;
    json j;

    void setup_d_JSON(bool type=true);
    void setup_f_JSON(bool type=true);

public:
    API();
    ~API();

    int  set_path(std::string path);
    void set_unit(unsigned int _unit);

    void sort_by_name(bool ascending=true);
    void sort_by_size(bool ascending=true);
    void sort_by_date(bool ascending=true);

    void setup_JSON();

    std::string return_answer();
};

#endif //API_HPP
