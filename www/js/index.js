const home = ".";
const home_index_name = "Home";
const api_index = "http://127.0.0.1:9099";
const url = "http://127.0.0.1/?path=";

var root;
var actual_dir;
var config_fail = false;
var nb_downloads = 0;

var index_json;
var full_size;
var nb_directories;
var nb_files;
var root_name;
var total_nb_elements;

$(document).ready(function(){
    var fixed_url = decode_utf8(window.location.href);
    var pos = fixed_url.indexOf("=");
    var path;

    if(pos < 0) path = home;
    else{
        path = fixed_url.substr(pos+1);
        if( path == "" || path == "./")
            path = home;
    }

    api_list_directory(path);
    on_click();
    on_hover();
});

function api_list_directory(path){
    // First delete all
    clean_all();

    actual_dir = path;
    var api_directory = api_index + "/directory?path=" + path;

    var jqxhr = $.getJSON(api_directory, function(index){

        // save json
        index_json = index;

        for(i in index.directories){
            $("tbody").append("<tr id=\"tr_"+ i +"\" class=\"directory\"></tr>");
            $("#tr_"+i).append("<td id=\"select\"><input class=\"selection\" type=\"checkbox\"></td>");
            $("#tr_"+i).append("<td id=\"type\"><img src=\"./fancyndex/www/icon/open-iconic/svg/folder.svg\" width=\"12\"></td>");
            $("#tr_"+i).append("<td id=\"name\">" + index.directories[i].name + "</td>");
            $("#tr_"+i).append("<td id=\"date\">" + index.directories[i].date + "</td>");
            $("#tr_"+i).append("<td id=\"size\">" + index.directories[i].size + "</td>");

            var content = "";
            if( index.directories[i].size > 0 ) { content = "nb elements (" + (index.directories[i].nb_elements) + ")"; }
            else                                { content = "empty directory"; }

            $("#tr_"+i).tooltip(
                {
                    title: content,
                    placement: "auto",
                    animation: true,
                    trigger: "hover",
                    delay: {
                        "show": 200,
                        "hide": 0
                    }
                }
            );
        }

        var nbd = index.nb_directories;
        for(i in index.files){
            var y = nbd+parseInt(i);
            $("tbody").append("<tr id=\"tr_"+ y +"\" class=\"file\"></tr>");
            $("#tr_"+y).append("<td id=\"select\"><input class=\"selection\" type=\"checkbox\"></td>");
            $("#tr_"+y).append("<td id=\"type\"><img src=\"./fancyndex/www/icon/open-iconic/svg/file.svg\" width=\"12\"></td>");
            $("#tr_"+y).append("<td id=\"name\">" + index.files[i].name + "</td>");
            $("#tr_"+y).append("<td id=\"date\">" + index.files[i].date + "</td>");
            $("#tr_"+y).append("<td id=\"size\">" + index.files[i].size + "</td>");
        }

        $(".selection").animateCss("fadeIn");
    });

    jqxhr.done(function(){
        update_url(path);
        update_nav(path);
        update_back_button(path);
        update_information(index_json);
        update_download_button();
        config_fail = false;
    });

    jqxhr.fail(function(){
        if( config_fail === false ){
            api_list_directory(home);
            config_fail = true;
        }
        else {
            /*
                If Ajax fails two time in a row, means that there is a mistake somewhere.
                Display error / information messages :
            */
            $("table").hide();
            $("body").append("<div class=\"alert alert-danger\" role=\"alert\"><strong>Wrong API configuration !</strong></div>");
            $("body").append("<div class=\"alert alert-warning\" role=\"alert\"><strong>You may check if the Json API C++ server is running</strong> (fancyndex executable)</div>");
            $("body").append("<div class=\"alert alert-info\" role=\"alert\">Error might comes from <strong> url API configuration </strong>(actual one : <a href=\""+ api_index +"\">"+ api_index +"</a>)</div>");
        }
    });
}

function on_click(){

    // Directory
    $(document).on("click", ".directory #type, .directory #name, .directory #date, .directory #size", function(){
        var size = $(this).parent().find("#size").text();
        var dir_name = $(this).parent().find("#name").text();

        if( size > 0 ){
            $(this).parent().tooltip("hide");
            var clicked_dir = actual_dir + "/" + dir_name;
            api_list_directory(clicked_dir);
        }
        else{
            $(this).parent().addClass("table-danger");
            $(this).parent().animateCss("shake");

            var tr_row = $(this).parent();
            setTimeout(function(){ tr_row.removeClass("table-danger"); }, 1000);
        }
    });

    // nav bar
    $(document).on("click", ".active button", function(){
        var dir_name = $(this).text();
        if(dir_name === home_index_name)
            dir_name = home;

        var arr = actual_dir.split("/");

        var path = "";
        for(var i=0; i < arr.length-1; ++i){
            path += arr[i];
            if(dir_name == arr[i])
                break;
            path += "/";
        }

        api_list_directory(path);
    });

    // When clicking on Home button
    $(document).on("click", ".disable button", function(){
        $(this).animateCss("hinge");
    });

    // Download file :
    $(document).on("click", ".file #type, .file #name, .file #date, .file #size", function(){
        var file_name = $(this).parent().find("#name").text();
        var clicked_file = actual_dir + "/" + file_name;

        var jqxhr = $.get(clicked_file, function(){
            window.location = clicked_file;
        });

        // If GET file fails :
        jqxhr.fail(function(){
            // MODAL POP UP
        });
    });

    // Back button :
    $(document).on("click", ".back-button", function(){
        var pos = actual_dir.lastIndexOf("/");
        var back_path = actual_dir.substr(0,pos);

        api_list_directory(back_path);
    });

    $(document).on("click", ".selection", function(){
        if( ! $(this).closest("tr").hasClass("selected") ){
            $(this).closest("tr").addClass("selected");
            ++nb_downloads;
            update_download_button();
        }
        else{
            $(this).closest("tr").removeClass("selected");
            --nb_downloads;
            update_download_button();
        }
    });


    $(document).on("click", ".download-button", function(){
        if( nb_downloads == 0 ){
            $('input[type="checkbox"]').prop("checked", true);
            $("#dl_modal").modal('show');
            nb_downloads = (index_json.nb_files + index_json.nb_directories);
            update_download_button();
        }
    });


    $(document).on("click", ".back-modal-button", function(){
        nb_downloads = 0;
        update_download_button();
        $('input[type="checkbox"]').prop("checked", false);
    });
}

/*
* Function hover on attribute populate by JS
*/
function on_hover(){

    // mouseenter on directory
    $(document).on("mouseenter", ".directory, .file", function(){
        //$(this).addClass("table-primary");
    });

    $(document).on("mouseleave", ".directory, .file", function(){
        //$(this).removeClass("table-primary");
    });

}

function update_download_button(){
    $(".badge-nb-dl").text(nb_downloads);
}

function update_information(index){
    $(".info_name").append(index.root_name);
    $(".info_size").append(index.full_size);
    $(".info_files").append(index.nb_files);
    $(".info_dirs").append(index.nb_directories);
    $(".info_el").append(index.nb_elements);
}

function update_back_button(path){
    if( path != home ){
        var back_name = "";
        var arr = path.split("/");
        if( arr.length > 2 ){ back_name = arr[arr.length-2]; }
        else                { back_name = home_index_name; }

        $(".info").prepend("<button type=\"button\" class=\"btn btn-warning btn-sm back-button\"><img src=\"./fancyndex/www/icon/open-iconic/svg/arrow-thick-left.svg\" width=\"15\"> "+ back_name +" </button>");
        $(".back-button").animateCss("slideInLeft");
    }
}

function update_nav(path){
    var arr = path.split("/");

    for(var i=0; i < arr.length; ++i){
        if(arr[0] === ".")
            arr[0] = home_index_name;

        if(i == (arr.length-1)){
            $(".nav").append("<li class=\"nav-item disable\"><button type=\"button\" class=\"btn btn-primary\">"+ arr[i] +"</button></li>");
        }
        else {
            $(".nav").append("<li class=\"nav-item active\"><button type=\"button\" class=\"btn btn-outline-primary\">"+ arr[i] +"</button></li>");
            $(".nav").append("<img src=\"./fancyndex/www/icon/open-iconic/svg/chevron-right.svg\" class=\"nav-img\">");
        }
    }

    if( arr.length > 1 )    $(".disable").animateCss("slideInRight");
    else                    $(".disable").animateCss("wobble");
}

function update_url(path){
    history.pushState(null, null, "?path="+path);
}

function clean_all(){
    $("tbody > tr").remove();
    $("li").remove();
    $(".nav-img").remove();
    $(".back-button").remove();
    $("dd").text("");

    nb_downloads=0;
}

/* Function for animate CSS */
$.fn.extend({
    animateCss: function (animationName) {
        var animationEnd = 'webkitAnimationEnd mozAnimationEnd MSAnimationEnd oanimationend animationend';
        this.addClass('animated ' + animationName).one(animationEnd, function() {
            $(this).removeClass('animated ' + animationName);
        });
        return this;
    }
});

function encode_utf8(s) { return encodeURIComponent(s); }
function decode_utf8(s) { return decodeURIComponent(s); }
