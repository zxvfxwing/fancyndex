/* --------------------------------------------- */
/* DO NOT INTERACT MANUALLY WITH THIS PART */
const home = ".";
const home_index_name = "Home";
const api_index = "https://api.spokonline.net/fs";
const url = "https://dl.spokonline.net/?path=";

const api_sort_GET = "sort=";
const api_mode_GET = "mode=";
const api_path_GET = "path=";

const api_active_path_GET = "active_path=";
const api_list_GET = "list=";

var GET_sort = 0;
var GET_mode = 1;
/* /!\ DO NOT /!\ */
/* --------------------------------------------- */

var root;
var actual_dir;
var config_fail = false;
var nb_downloads = 0;
var dl_array = [];
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
    /* first, we need to remove and clean previous work */
    clean_all();

    actual_dir = path;
    var api_directory = api_index + "/dir?" + api_mode_GET + GET_mode + "&" + api_sort_GET + GET_sort + "&" + api_path_GET + path;

    var jqxhr = $.getJSON(api_directory, function(index){
        index_json = index; // save json into global variable

        var i; // Loop index
        var y; // tr index for files
        var d; // dir temporary object
        var f; // file temporary object
        var content; // content of tooltip for folders

        for(i=0; i < index.nb_directories; ++i){
            d = index.directories[i];

            $("tbody").append("<tr id=\"tr_"+ i +"\" class=\"directory\"></tr>");
            $("#tr_"+i).append("<td id=\"select\"><input class=\"selection\" type=\"checkbox\"></td>");
            $("#tr_"+i).append("<td id=\"type\"><img src=\"./fancyndex/www/icon/open-iconic/svg/folder.svg\" width=\"12\"></td>");
            $("#tr_"+i).append("<td id=\"name\">" + d.name + "</td>");
            $("#tr_"+i).append("<td id=\"date\">" + d.date + "</td>");
            $("#tr_"+i).append("<td id=\"size\">" + d.size + "</td>");
            $("#tr_"+i).append("<td id=\"unit\">" + d.unit + "</td>");

            content = "";
            if( d.size > 0 ) { content = "Number of elements (" + (d.nb_elements) + ")"; }
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
        for(i=0; i < index.nb_files; ++i){
            y = nbd+i;
            f = index.files[i];

            $("tbody").append("<tr id=\"tr_"+ y +"\" class=\"file\"></tr>");
            $("#tr_"+y).append("<td id=\"select\"><input class=\"selection\" type=\"checkbox\"></td>");
            $("#tr_"+y).append("<td id=\"type\"><img src=\"./fancyndex/www/icon/open-iconic/svg/file.svg\" width=\"12\"></td>");
            $("#tr_"+y).append("<td id=\"name\">" + f.name + "</td>");
            $("#tr_"+y).append("<td id=\"date\">" + f.date + "</td>");
            $("#tr_"+y).append("<td id=\"size\">" + f.size + "</td>");
            $("#tr_"+y).append("<td id=\"unit\">" + f.unit + "</td>");
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

function api_download_archive(){

    /* Add a gif loading animation */
    $(".info").append("<img class=\"loading\" src=\"./fancyndex/www/gif/ajax_load.gif\" alt=\"Loading\" width=\"40\">");

    var api_archive = api_index + "/archive?" + api_active_path_GET + actual_dir + "/&" + api_list_GET;

    for(var i=0; i < nb_downloads; ++i){
        api_archive += dl_array[i][0] + ",";
    }

    /* call API with right url */
    var archive_answer;
    var jqxhr = $.getJSON(api_archive, function(answer){
        archive_answer = answer;
    });

    jqxhr.done(function(){
        /* remove gif */
        $(".loading").remove();

        /* if archiving didn't failed, get the download */
        if( archive_answer.archive_path != "" )
            window.location = archive_answer.archive_path;

        wait_and_deselect(actual_dir);
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
        wait_and_deselect(actual_dir);
    });

    // Back button :
    $(document).on("click", ".back-button", function(){
        var pos = actual_dir.lastIndexOf("/");
        var back_path = actual_dir.substr(0,pos);
        api_list_directory(back_path);
    });

    $(document).on("click", ".selection", function(){
        update_download_array();
    });

    $(document).on("click", ".download-button", function(){
        if( nb_downloads == 0 ){
            $('input[type="checkbox"]').prop("checked", true);
            update_download_array();
            $("#dl_modal").modal('show');
        }

        /* Only one selection, and it's a file */
        else if( nb_downloads ==  1 && dl_array[0][1] == 1 ){
            window.location = actual_dir + "/" + dl_array[0][0];
            wait_and_deselect(actual_dir);
        }

        /* Multiples selections */
        else{
            if( nb_downloads == (index_json.nb_directories + index_json.nb_files) ){
                $("#dl_modal").modal('show');
            }
            else{
                api_download_archive();
            }
        }
    });

    $(document).on("click", ".dl-modal-button", function(){
        api_download_archive();
    });

    $(document).on("click", ".back-modal-button", function(){
        wait_and_deselect(actual_dir);
    });

    $(document).on("click", "th#name", function(){
        if( GET_sort == 0 && GET_mode == 1 )
            GET_mode = 0;
        else {
            GET_sort = 0;
            GET_mode = 1;
        }
        api_list_directory(actual_dir);
    });

    $(document).on("click", "th#size", function(){
        if( GET_sort == 1 && GET_mode == 1 )
            GET_mode = 0;
        else {
            GET_sort = 1;
            GET_mode = 1;
        }
        api_list_directory(actual_dir);
    });

    $(document).on("click", "th#date", function(){
        if( GET_sort == 2 && GET_mode == 1 )
            GET_mode = 0;
        else {
            GET_sort = 2;
            GET_mode = 1;
        }
        api_list_directory(actual_dir);
    });

    $(document).on("click", ".release-note", function(){
        $("#release-note-modal").modal('show');
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

function wait_and_deselect(dir){
    if( dir == actual_dir ){
        setTimeout(function(){
            $('input[type="checkbox"]').prop("checked", false);
            update_download_array();
        }, 2000);
    }
}

function update_chevron_img(){
    var _id;
    var _svgName;

    switch(GET_sort)
    {
        case 0: _id = "#name"; break;
        case 1: _id = "#size"; break;
        case 2: _id = "#date"; break;
    }

    if( GET_mode == 1 ) _svgName = "chevron-bottom";
    else _svgName = "chevron-top";

    $("th#name").find("img").remove();
    $("th#size").find("img").remove();
    $("th#date").find("img").remove();

    $("th"+_id).append(" <img src=\"./fancyndex/www/icon/open-iconic/svg/"+ _svgName +".svg\">")
}

function update_download_array(){
    nb_downloads = 0;
    dl_array = [];
    var thisTR;
    $('input[type=checkbox]').each(function (){
        thisTR = $(this).closest("tr");

        if( this.checked == true ){
            thisTR.addClass("selected");

            dl_array[nb_downloads] = [];

            if( thisTR.hasClass("directory") )  dl_array[nb_downloads][1] = 0;
            else                                dl_array[nb_downloads][1] = 1;

            /* Get the tr_n° */
            var tr_num = thisTR.attr("id").replace(/^\D+/g, '');

            dl_array[nb_downloads++][0] = $("#tr_"+ tr_num).find("#name").text();
        }
        else{
            thisTR.removeClass("selected");
        }
    });
    update_download_button();
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
