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
    Wt::WText* wtext_h1_title;
    Wt::WText* wtext_answer;
    Wt::WLineEdit* wline_text_input;
    Wt::WPushButton* button;
    Wt::WBootstrapTheme* bootstrap;

public:
    Index(const Wt::WEnvironment& );
    ~Index();
    void answer();
};


#endif //INDEX_HPP
