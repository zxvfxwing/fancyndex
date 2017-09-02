#include <cppcms/application.h>
#include <cppcms/applications_pool.h>
#include <cppcms/service.h>
#include <cppcms/http_response.h>
#include <iostream>
#include "directory.hpp"

class hello : public cppcms::application {
public:
    hello(cppcms::service &srv) :
        cppcms::application(srv)
    {}
    virtual void main(std::string url);
};

void hello::main(std::string )
{
    fs::path p(".");
    Directory* dir = new Directory(p);

    response().out() <<
        "<html>\n"
        "<body>\n"
        "  <h1>Hello World</h1>\n";

    unsigned long long int i;
    for(i=0; i < dir->get_nb_files(); ++i){
        response().out() <<
            "<p>"
            + dir->get_file(i)->get_name()
            + " ---- "
            + dir->get_file(i)->get_date_human()
            + " ----- "
            + std::to_string(dir->get_file(i)->get_size());
    }

    response().out() <<
        "</body>\n"
        "</html>\n";


    delete dir;
}

int main(int argc,char ** argv)
{
    try {
        cppcms::service srv(argc, argv);
        srv.applications_pool().mount(
            cppcms::applications_factory<hello>()
        );
        srv.run();
    }
    catch(std::exception const &e) {
        std::cerr << e.what() << std::endl;
    }
}
