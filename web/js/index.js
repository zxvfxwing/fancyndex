const float_to_fixed = 2;

/* Number of cells */
const cell_name = 1;
const cell_date = 2;
const cell_size = 3;
const cell_unit = 4;

var location_pathname = window.location.pathname;

/* Cut window location pathname after "/home" (5 chars) */
var API_pathname = location_pathname.substring(5);

/* Get URL Params, Queries */
var urlParams = new URLSearchParams(window.location.search);

var _by_ = urlParams.get("by");
var _ascending_ = urlParams.get("ascending");

/* DEBUG *
console.log( location_pathname );
console.log( API_pathname );

console.log(urlParams.get("by"));
console.log(urlParams.get("ascending"));
/* -------- */

/* JSON of the current directory */
var currentJSON = null;

function update_dirs_size(DirJSON){

    test = DirJSON;

    var Directories = document.getElementsByClassName("is-directory");
    for(var i=0; i < Directories.length; ++i){
        var dir = DirJSON.directories[i];

        /* If user wants it to be sorted by size, we have to change also name / datetime place */
        if( _by_ == "size" ){
            Directories[i].cells[cell_name].innerHTML = dir.name;
            Directories[i].cells[cell_date].innerHTML = dir.datetime;
        }

        if( JSON.stringify(dir.hsize).includes(".") ){
            Directories[i].cells[cell_size].innerHTML = dir.hsize.toFixed(float_to_fixed);
        }
        else{
            Directories[i].cells[cell_size].innerHTML = dir.hsize;
        }

        Directories[i].cells[cell_unit].innerHTML = dir.short_unit;
    }
}

function truncate_files_size(fixed_number){
    var Files = document.getElementsByClassName("is-file");
    for(var i=0; i < Files.length; ++i){
        var hsize_str = Files[i].cells[cell_size].innerHTML;
        if( hsize_str.includes(".") ) {
            Files[i].cells[cell_size].innerHTML = Number(hsize_str).toFixed(fixed_number);
        }
    }
}

function API_get_path(path, sort_method, ascending){
    if( sort_method === null ) {
         sort_method = "name";
    }

    if( ascending === null ) {
        ascending = true;
    }

    var r = new XMLHttpRequest();
    r.responseType = "json";

    r.onreadystatechange = function() {
        if (r.readyState != 4 || r.status != 200) return;
        update_dirs_size(r.response);
    };

    var request_url = "/api/path" + path + "?by=" + sort_method + "&ascending=" + ascending;

    r.open("GET", request_url, true);
    r.send();
}

truncate_files_size(float_to_fixed);

nbDir = document.getElementsByClassName("is-directory").length;
if( nbDir > 0 ){
    API_get_path(API_pathname, _by_, _ascending_);
}
