const home = ".";
const home_index_name = "Home";
const api_index = "http://localhost/api";
const url = "http://localhost/?path=";
var root;
var actual_dir;

$(document).ready(function(){
    /*
    * GET parameters
    * regex rule
    */
    var path = document.URL.match(/path=([A-Za-z0-9_./]+)/);

    if(path === undefined || path === null)
        path = home;
    else
        path = path[1];

    api_list_directory(path);
    on_click();
});


function api_root_path(){
    $.getJSON(api_index + "/root", function(path){
        root = path.root;
    });
}

function api_list_directory(path){
    update_nav(path);

    actual_dir = path;
    var api_directory = api_index + "/directory?path=" + path;

    $.getJSON(api_directory, function(index){
        $("h1").append(index.root_name);
        $("#p_path").append(api_directory);

        for(i in index.directories){
            $("tbody").append("<tr id=\"tr_"+ i +"\" class=\"directory animated fadeIn\"></tr>");
            $("#tr_"+i).append("<td id=\"type\"><span class=\"badge badge-primary\">dir</span></td>");
            $("#tr_"+i).append("<td id=\"name\">" + index.directories[i].name + "</td>");
            $("#tr_"+i).append("<td id=\"size\">" + index.directories[i].size + "</td>");
            $("#tr_"+i).append("<td id=\"date\">" + index.directories[i].date + "</td>");
        }

        var nbd = index.nb_directories;
        for(i in index.files){
            var y = nbd+parseInt(i);
            $("tbody").append("<tr id=\"tr_"+ y +"\" class=\"file animated fadeIn\"></tr>");
            $("#tr_"+y).append("<td id=\"type\"><span class=\"badge badge-secondary\">file</span></td>");
            $("#tr_"+y).append("<td id=\"name\">" + index.files[i].name + "</td>");
            $("#tr_"+y).append("<td id=\"size\">" + index.files[i].size + "</td>");
            $("#tr_"+y).append("<td id=\"date\">" + index.files[i].date + "</td>");
        }
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

        $("button").each(function(){
            $(this).addClass("animated bounceOut");
        });

        clean_all();
        api_list_directory(path);
        history.pushState(null, null, "?path="+path);
    });


    $(document).on("click", ".disable button", function(){
        $(this).animateCss("wobble");
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

        //document.location.assign(clicked_file);
    });
}

function clean_all(){
    $("h1").text('');
    $("#p_path").text('');
    $("tbody > tr").remove();
    $(".breadcrumb-item").remove();
    $("li").remove();
}

function update_nav(path){
    var arr = path.split("/");

    for(var i=0; i < arr.length; ++i){
        if(arr[0] === ".")
            arr[0] = home_index_name;

        $(".nav").append("<li class=\"nav-item active\"><button type=\"button\" class=\"btn btn-outline-primary\">"+ arr[i] +"</button></li>");
        //$(".nav").append("<li class=\"nav-item\">/</li>");

        //$(".breadcrumb").append("<li class=\"breadcrumb-item inactive\">" + arr[i] + "</li>");

        if(i == (arr.length-1)){
            $("li:last-child").toggleClass("active disable");
            $(".disable > .btn").toggleClass("btn-outline-primary btn-outline-secondary");
            //$(".breadcrumb .breadcrumb-item:last-child").toggleClass("inactive active");
            //$(".breadcrumb .breadcrumb-item:last-child").removeClass("inactive");
        }
    }
}

$.fn.extend({
    animateCss: function (animationName) {
        var animationEnd = 'webkitAnimationEnd mozAnimationEnd MSAnimationEnd oanimationend animationend';
        this.addClass('animated ' + animationName).one(animationEnd, function() {
            $(this).removeClass('animated ' + animationName);
        });
        return this;
    }
});
