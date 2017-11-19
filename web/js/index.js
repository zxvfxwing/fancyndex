const float_to_fixed = 2;
const cell_hsize = 3;
const cell_unit = 4;

var location_pathname = window.location.pathname;

/* Cut window location pathname after "/home" (5 chars) */
var API_pathname = location_pathname.substring(5);

/* Get URL Params, Queries */
var urlParams = new URLSearchParams(window.location.search);

/* DEBUG *
console.log( location_pathname );
console.log( API_pathname );

console.log(urlParams.get("by"));
console.log(urlParams.get("ascending"));
/* -------- */

function update_dir_sizes(DirJSON){
    var Directories = document.getElementsByClassName("is-directory");
    for(var i=0; i < Directories.length; ++i){
        Directories[i].cells[cell_hsize].innerHTML = DirJSON.directories[i].hsize;
        Directories[i].cells[cell_unit].innerHTML = DirJSON.directories[i].short_unit;
    }
    truncate_hsize(float_to_fixed);
}

function truncate_hsize(fixed_number){
    var HumanSizes = document.getElementsByClassName("size");
    for(var i=0; i < HumanSizes.length; ++i){
        var hsize_str = HumanSizes[i].innerHTML;
        if( hsize_str.includes(".") ) {
            HumanSizes[i].innerHTML = Number(hsize_str).toFixed(fixed_number);
        }
    }
}

function API_get_path(path, sort_method, ascending){
    if( sort_method === undefined ) {
         sort_method = "name";
    }

    if( ascending === undefined ) {
        ascending = true;
    }

    var r = new XMLHttpRequest();
    r.responseType = "json";

    r.onreadystatechange = function() {
        if (r.readyState != 4 || r.status != 200) return;
        update_dir_sizes(r.response);
    };

    var request_url = "/api/path" + path + "?by=" + sort_method + "&ascending=" + ascending;

    r.open("GET", request_url, true);
    r.send();
}


truncate_hsize(float_to_fixed);
API_get_path(API_pathname, urlParams.get("by"), urlParams.get("ascending"));
