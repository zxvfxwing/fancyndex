const home = ".";
const home_index_name = "Home";
const api_index = "http://localhost/api";
const url = "http://localhost/?path=";
var root;
var actual_dir;

$(document).ready(function(){

    var fixed_url = decode_utf8(window.location.href);
    var pos = fixed_url.indexOf(".");
    var path = fixed_url.substr(pos);

    api_list_directory(path);
    on_click();
});

function api_list_directory(path){
    actual_dir = path;

    var api_directory = api_index + "/directory?path=" + path;

    var jqxhr = $.getJSON(api_directory, function(index){
        $("h1").append(index.root_name);
        $("#p_path").append(api_directory);

        $("h1").animateCss("slideInDown");
        $("thead").animateCss("fadeIn");

        for(i in index.directories){
            $("tbody").append("<tr id=\"tr_"+ i +"\" class=\"directory animated fadeIn\"></tr>");
            $("#tr_"+i).append("<td id=\"type\"><span class=\"badge badge-primary\">dir</span></td>");
            $("#tr_"+i).append("<td id=\"name\">" + index.directories[i].name + "</td>");
            $("#tr_"+i).append("<td id=\"date\">" + index.directories[i].date + "</td>");
            $("#tr_"+i).append("<td id=\"size\">" + index.directories[i].size + "</td>");
        }

        var nbd = index.nb_directories;
        for(i in index.files){
            var y = nbd+parseInt(i);
            $("tbody").append("<tr id=\"tr_"+ y +"\" class=\"file animated fadeIn\"></tr>");
            $("#tr_"+y).append("<td id=\"type\"><span class=\"badge badge-secondary\">file</span></td>");
            $("#tr_"+y).append("<td id=\"name\">" + index.files[i].name + "</td>");
            $("#tr_"+y).append("<td id=\"date\">" + index.files[i].date + "</td>");
            $("#tr_"+y).append("<td id=\"size\">" + index.files[i].size + "</td>");
        }
    })

    jqxhr.done(function(){
        update_back_button(path);
        update_nav(path);
    });
}

function on_click(){
    $(document).on("click", ".directory", function(){
        var dir_name = $(this).find("#name").text();
        var clicked_dir = actual_dir + "/" + dir_name;

        clean_all();
        api_list_directory(clicked_dir);

        // History
        history.pushState(null, null, "?path="+clicked_dir);
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

        clean_all();
        api_list_directory(path);
        history.pushState(null, null, "?path="+path);
    });


    $(document).on("click", ".disable button", function(){
        $(this).animateCss("hinge");
    });

    // Téléchargement :
    $(document).on("click", ".file", function(){
        var file_name = $(this).find("#name").text();
        var clicked_file = actual_dir + "/" + file_name;

        console.log(clicked_file);

        $.ajax({
            url: clicked_file,
            type: 'GET',
            success: function() {
                window.location = clicked_file;
            }
        });

        /*$.get(clicked_file, function(data){
            window.location = clicked_file;
        });*/
    });

    $(document).on("click", ".back-button", function(){
        var pos = actual_dir.lastIndexOf("/");
        var back_path = actual_dir.substr(0,pos);

        clean_all();
        api_list_directory(back_path);
        history.pushState(null, null, "?path="+back_path);
    });
}

function update_back_button(path){
    if(path == "."){
        $(".back-button").hide();
    }
    else {
        var back_name = "";
        var arr = path.split("/");
        if( arr.length > 2 ){
            back_name = arr[arr.length-2];
        }
        else{
            back_name = home_index_name;
        }

        $(".btn-back-text").text(back_name);
        $(".back-button").show();
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

    if( arr.length > 1 )
        $(".disable").animateCss("slideInRight");
    else
        $(".disable").animateCss("wobble");
}

function clean_all(){
    $("h1").text('');
    $("#p_path").text('');
    $("tbody > tr").remove();
    $("li").remove();
    $(".nav-img").remove();
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
