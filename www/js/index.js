const home = ".";
const home_index_name = "Home";
const api_index = "http://localhost/api";
const url = "http://localhost/?path=";
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
    update_breadcrumb(path);
    on_click();
});


function api_list_directory(path){
    actual_dir = path;
    var api_directory = api_index + "/directory?path=" + path;
    console.log(api_directory);

    $.getJSON(api_directory, function(index){
        $("h1").append(index.root_name);
        $("#p_path").append(api_directory);

        for(i in index.directories){
            $("tbody").append("<tr id=\"tr_"+ i +"\" class=\"directory\"></tr>");
            $("#tr_"+i).append("<td id=\"type\"><span class=\"badge badge-primary\">dir</span></td>");
            $("#tr_"+i).append("<td id=\"name\">" + index.directories[i].name + "</td>");
            $("#tr_"+i).append("<td id=\"size\">" + index.directories[i].size + "</td>");
            $("#tr_"+i).append("<td id=\"date\">" + index.directories[i].date + "</td>");
        }

        var nbd = index.nb_directories;
        for(i in index.files){
            var y = nbd+parseInt(i);
            $("tbody").append("<tr id=\"tr_"+ y +"\" class=\"file\"></tr>");
            $("#tr_"+y).append("<td id=\"type\"><span class=\"badge badge-secondary\">file</span></td>");
            $("#tr_"+y).append("<td id=\"name\">" + index.files[i].name + "</td>");
            $("#tr_"+y).append("<td id=\"size\">" + index.files[i].size + "</td>");
            $("#tr_"+y).append("<td id=\"size\">" + index.files[i].date + "</td>");
        }
    });
}

function on_click(){
    $(document).on("click", ".directory", function(){
        var dir_name = $(this).find("#name").text();
        var clicked_dir = actual_dir + "/" + dir_name;

        clean_all();
        api_list_directory(clicked_dir);
        update_breadcrumb(clicked_dir);
    });
}

function clean_all(){
    $("h1").text('');
    $("#p_path").text('');
    $("tbody > tr").remove();
    $(".breadcrumb-item").remove();
}

function update_breadcrumb(path){
    var arr = path.split("/");

    for(var i=0; i < arr.length; ++i){
        if(arr[0] === ".")
            arr[0] = home_index_name;

        $(".breadcrumb").append("<li class=\"breadcrumb-item inactive\">" + arr[i] + "</li>");

        if(i == (arr.length-1)){
            $(".breadcrumb .breadcrumb-item:last-child").addClass("active");
            $(".breadcrumb .breadcrumb-item:last-child").removeClass("inactive");
        }
    }

    var chain_dir = url;
    $('.breadcrumb .inactive').each(function(){
        var dir = $(this).text();
        if(dir == "Home")   chain_dir += ".";
        else                chain_dir += "/" + dir;
        $(this).wrapInner("<a href=\""+ chain_dir +"\"/>");
    });
}
