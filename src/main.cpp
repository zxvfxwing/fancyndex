#include <Wt/WApplication>
#include "index.hpp"
#include "directory.hpp"

Wt::WApplication* createApp(const Wt::WEnvironment& env)
{
    return new Index(env);
}

int main(int argc, char** argv){
    fs::path p(".");
    Directory* dir = new Directory(p);

    unsigned long long int i;
    for(i=0; i < dir->get_nb_files(); ++i){

        // TEST AU NIVEAU DES EXTENSIONS
        if(dir->get_file(i)->is_extension_ok())
            std::cout << dir->get_file(i)->get_name() << std::endl;
    }

    delete dir;

    return 0;
    //return Wt::WRun(argc, argv, &createApp);
}
