#ifndef INDEX_HPP
#define INDEX_HPP

#include <Wt/WApplication>
#include <Wt/WContainerWidget>
#include <Wt/WText>
#include <Wt/WLineEdit>
#include <Wt/WPushButton>
#include <Wt/WBootstrapTheme>
#include <string>

class Index : public Wt::WApplication
{
private:
    

public:
    Index(const Wt::WEnvironment& );
    ~Index();
    void answer();
};


#endif //INDEX_HPP
