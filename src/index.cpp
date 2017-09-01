#include "index.hpp"
#include "directory.hpp"

Index::Index(const Wt::WEnvironment& env)
    : Wt::WApplication(env)
{
    static const std::string str_page_title = "Hello World !";
    static const std::string str_h1_title = "<h1>Je m'appelle Marjolaine, et toi ?</h1>";
    static const std::string str_button_title = "Execute";

    wtext_h1_title = new Wt::WText();
    wline_text_input = new Wt::WLineEdit();
    button = new Wt::WPushButton();
    wtext_answer = new Wt::WText();
    bootstrap = new Wt::WBootstrapTheme();

    wtext_h1_title->setText(str_h1_title);
    wline_text_input->setFocus();
    button->setText(str_button_title);
    wtext_answer->setMargin(20, Wt::Left);
    bootstrap->setResponsive(true);

    setTheme(bootstrap);
    setTitle(str_page_title);

    root()->addWidget(wtext_h1_title);
    root()->addWidget(wline_text_input);
    root()->addWidget(button);
    root()->addWidget(wtext_answer);

    button->clicked().connect(this, &Index::answer);
    wline_text_input->enterPressed().connect(this, &Index::answer);
}

Index::~Index()
{
    delete wtext_h1_title;
    delete wline_text_input;
    delete button;
    delete wtext_answer;
    delete bootstrap;
}

void Index::answer()
{
    wtext_answer->setText("Moi c'est " + wline_text_input->text());
}
