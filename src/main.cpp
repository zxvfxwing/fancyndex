#include <Wt/WApplication>
#include "index.hpp"

Wt::WApplication* createApp(const Wt::WEnvironment& env)
{
    return new Index(env);
}

int main(int argc, char** argv){
    return Wt::WRun(argc, argv, &createApp);
}
