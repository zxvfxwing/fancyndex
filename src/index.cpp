#include "index.hpp"
#include "directory.hpp"

Index::Index(const Wt::WEnvironment& env)
    : Wt::WApplication(env)
{
    //static const std::string str_page_title = "FileSystem";
    //static const std::string str_h1_title = "<h1>FileSystem</h1>";
    //static const std::string str_button_title = "Execute";

    //wtext_h1_title = new Wt::WText();
    //wline_text_input = new Wt::WLineEdit();
    //button = new Wt::WPushButton();
    //wtext_answer = new Wt::WText();


    //wtext_h1_title->setText(str_h1_title);
    //wline_text_input->setFocus();
    //button->setText(str_button_title);
    //wtext_answer->setMargin(20, Wt::Left);

    setTheme(new Wt::WBootstrapTheme());
    setTitle("Test");
    root()->addWidget(new Wt::WText("<h1>FileSystem</h1>"));

    //root()->addWidget(wtext_h1_title);

    fs::path p(".");
    Directory* dir = new Directory(p);

    Wt::WContainerWidget* container = new Wt::WContainerWidget();
    container->addWidget(new Wt::WText("<h2>"+dir->get_name()+"</h2>"));

    unsigned long long int i;
    for(i=0; i < dir->get_nb_files(); ++i)
    {
        container->addWidget(new Wt::WText("<p>"+ dir->get_file(i)->get_name() + " -- " + dir->get_file(i)->get_size_str() + "</p>"));

        container->addWidget(new Wt::WText("<a href=\"" + dir->get_file(i)->get_name()+"\"> download </a>"));
    }

    for(i=0; i < dir->get_nb_directories(); ++i)
    {
        container->addWidget(new Wt::WText("<p>"+ dir->get_directory(i)->get_name() + " -- " + dir->get_directory(i)->get_size_str() + "</p>"));
    }

    root()->addWidget(container);

    delete dir;

    //root()->addWidget(wline_text_input);
    //root()->addWidget(button);
    //root()->addWidget(wtext_answer);

    //button->clicked().connect(this, &Index::answer);
    //wline_text_input->enterPressed().connect(this, &Index::answer);
}

Index::~Index()
{
}
