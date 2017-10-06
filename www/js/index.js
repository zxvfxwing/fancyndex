const home = ".";
const home_index_name = "Home";
const api_index = "https://api.spokonline.net/fs";
const url = "https://dl.spokonline.net/?path=";

const api_by_name = "by_name";
const api_by_size = "by_size";
const api_by_date = "by_date";
const api_mode_GET = "?mode=";
const api_path_GET = "&path=";

var GET_mode = 1;
var api_sort_method = api_by_name;

var root;
var actual_dir;
var config_fail = false;
var nb_downloads = 0;

var index_json;

$(document).ready(function(){
    var fixed_url = decode_utf8(window.location.href);
    var pos = fixed_url.indexOf("=");
    var path;

    if(pos < 0) path = home;
    else{
        path = fixed_url.substr(pos+1);
        if( path == "" || path == "./")
            path = home;
    }

    loading();
    api_list_directory(path);
    on_click();
    on_hover();
});

function api_list_directory(path){
    // First delete all
    clean_all();

    actual_dir = path;
    var api_directory = api_index + "/dir/" + api_sort_method + api_mode_GET + GET_mode + api_path_GET + path;

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
            $("#tr_"+i).append("<td id=\"unit\">" + index.directories[i].unit + "</td>");

            var content = "";
            if( index.directories[i].size > 0 ) { content = "Number of elements (" + (index.directories[i].nb_elements) + ")"; }
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
            $("#tr_"+y).append("<td id=\"unit\">" + index.files[i].unit + "</td>");
        }

        $(".selection").animateCss("fadeIn");
    });

    jqxhr.done(function(){
        update_url(path);
        update_nav(path);
        update_back_button(path);
        update_information(index_json);
        update_download_button();
        update_chevron_img();
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
            $(".main-body").hide();
            $(".alert-div").append("<div class=\"alert alert-danger\" role=\"alert\"><strong>Wrong API configuration !</strong></div>");
            $(".alert-div").append("<div class=\"alert alert-warning\" role=\"alert\"><strong>You may check if the Json API C++ server is running</strong> (fancyndex executable)</div>");
            $(".alert-div").append("<div class=\"alert alert-info\" role=\"alert\">Error mights comes from <strong> url API configuration </strong>(actual one : <a href=\""+ api_index +"\">"+ api_index +"</a>)</div>");
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
            //$(this).parent().animateCss("shake");

            /*var tr_row = $(this).parent();
            setTimeout(function(){ tr_row.removeClass("table-danger"); }, 1000);*/
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
        window.location = clicked_file;
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
        if( nb_downloads == 0 || nb_downloads == (index_json.nb_files + index_json.nb_directories) ){
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

    $(document).on("click", "th#name", function(){
        if( api_sort_method == api_by_name && GET_mode == 1 )
            GET_mode = 0;
        else {
            api_sort_method = api_by_name;
            GET_mode = 1;
        }
        api_list_directory(actual_dir);
    });

    $(document).on("click", "th#size", function(){
        if( api_sort_method == api_by_size && GET_mode == 1 )
            GET_mode = 0;
        else {
            api_sort_method = api_by_size;
            GET_mode = 1;
        }
        api_list_directory(actual_dir);
    });

    $(document).on("click", "th#date", function(){
        if( api_sort_method == api_by_date && GET_mode == 1 )
            GET_mode = 0;
        else {
            api_sort_method = api_by_date;
            GET_mode = 1;
        }
        api_list_directory(actual_dir);
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

function update_chevron_img(){
    var _id;
    var _svgName;

    if( api_sort_method == api_by_name )
        _id = "#name";
    else if( api_sort_method == api_by_size )
        _id = "#size";
    else if ( api_sort_method == api_by_date )
        _id = "#date";

    if( GET_mode == 1 ) _svgName = "chevron-bottom";
    else _svgName = "chevron-top";

    $("th#name").find("img").remove();
    $("th#size").find("img").remove();
    $("th#date").find("img").remove();

    $("th"+_id).append(" <img src=\"./fancyndex/www/icon/open-iconic/svg/"+ _svgName +".svg\">")
}

function update_download_button(){
    $(".badge-nb-dl").text(nb_downloads);
}

function update_information(index){
    $(".info_name").text(index.root_name);
    $(".info_size").text(index.full_size);
    $(".info_files").text(index.nb_files);
    $(".info_dirs").text(index.nb_directories);
    $(".info_el").text(index.nb_elements);
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

function loading(){
    var loading = $("#loading");
    $(document).ajaxStart(function () {
        //loading.show();
    });

    $(document).ajaxStop(function () {
        //loading.hide();
    });
}

function clean_all(){
    $("tbody > tr").remove();
    $("li").remove();
    $(".nav-img").remove();
    $(".back-button").remove();

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
