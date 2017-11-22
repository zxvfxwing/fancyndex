const float_to_fixed = 2;

/* Number of cells */
const cell_name = 1;
const cell_date = 2;
const cell_size = 3;
const cell_unit = 4;

const home = "/home";

//var location_pathname = decode_utf8( window.location.pathname );

//var the_pathname = document.getElementById("api_pathname").getElementsByTagName("a")[0].attributes[0].value;
var pathname = document.getElementById("api_pathname").innerHTML;
console.log( pathname );

/* Cut window location pathname after "/home" (5 chars) */
var API_pathname = pathname.substring(home.length);

/* Get URL Params, Queries */
var urlParams = new URLSearchParams(window.location.search);

//var _by_ = urlParams.get("by");
//var _ascending_ = urlParams.get("ascending");

_by_ = document.getElementById("sort_by").innerHTML;
_ascending_ = document.getElementById("sort_ascending").innerHTML;

/* DEBUG *
console.log( location_pathname );
console.log( API_pathname );

console.log(urlParams.get("by"));
console.log(urlParams.get("ascending"));
/* -------- */

/* JSON of the current directory */
var currentJSON = null;

function encode_utf8(s) {
  return encodeURIComponent(s);
}

function decode_utf8(s) {
  return decodeURIComponent(s);
}

function th_click(th_class) {

    if( th_class != _by_ ) {
        _by_ = th_class;
        _ascending_ = "true";
    }
    else {
        if( _ascending_ == "true" ) _ascending_ = "false";
        else                        _ascending_ = "true";
    }

    window.location.href = pathname + "?by=" + _by_ + "&ascending=" + _ascending_ ;
}

function dir_click(dir_id) {
    var dir_name = document.getElementById(dir_id).cells[cell_name].innerHTML;
    window.location.href = pathname + "/" + dir_name + "?by=" + _by_ + "&ascending=" + _ascending_ ;
}

function update_breadcumb(pathname, by, ascending) {
    if( by === undefined || by === null ){
        by = "name";
    }

    if( ascending === undefined || ascending === null ){
        ascending = true;
    }

    if( pathname[0] == "/" )
        pathname = pathname.substring(1);

    while( pathname[pathname.length-1] == "/" ){
        pathname = pathname.substring(0,pathname.length-1);
    }

    var iter = pathname.split("/");
    var bread_ul = document.getElementsByClassName("breadcrumb")[0].children[0];
    var phref = "";
    var i;

    for(i=0; i < iter.length-1; ++i) {
        phref += "/" + iter[i];
        bread_ul.innerHTML += "<li><a href =\"" + phref + "?by=" + by + "&ascending=" + ascending + "\">" + iter[i] + "</a></li>";
    }

    phref += "/" + phref[i] + "?by=" + by + "&ascending=" + ascending;
    bread_ul.innerHTML += "<li class=\"is-active\"><a href =\"" + phref + "\" aria-current=\"directory\">" + iter[i] + "</a></li>";
}

function update_dirs_size(DirJSON) {
    test = DirJSON;

    var Directories = document.getElementsByClassName("is-directory");
    for(var i=0; i < Directories.length; ++i) {
        var dir = DirJSON.directories[i];

        /* If user wants it to be sorted by size, we have to change also name / datetime place */
        if( _by_ == "size" ) {
            Directories[i].cells[cell_name].innerHTML = dir.name;
            Directories[i].cells[cell_date].innerHTML = dir.datetime;
        }

        if( String(dir.hsize).includes(".") ) {
            Directories[i].cells[cell_size].innerHTML = dir.hsize.toFixed(float_to_fixed);
        }
        else {
            Directories[i].cells[cell_size].innerHTML = dir.hsize;
        }

        Directories[i].cells[cell_unit].innerHTML = dir.short_unit;
    }
}

function truncate_files_size(fixed_number) {
    var Files = document.getElementsByClassName("is-file");
    for(var i=0; i < Files.length; ++i){
        var hsize_str = Files[i].cells[cell_size].innerHTML;
        if( hsize_str.includes(".") ) {
            Files[i].cells[cell_size].innerHTML = Number(hsize_str).toFixed(fixed_number);
        }
    }
}

function API_get_path(path, sort_method, ascending) {
    if( undefined ===  sort_method || sort_method === null ) {
         sort_method = "name";
    }

    if( undefined === ascending || ascending === null ) {
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

//update_breadcumb(location_pathname, _by_, _ascending_);
truncate_files_size(float_to_fixed);

/* Ajax call only if there is at least one directory */
nbDir = document.getElementsByClassName("is-directory").length;
if( nbDir > 0 ) {
    API_get_path(API_pathname, _by_, _ascending_);
}

quicksort(0, nbDir-1);

/* Try on sort algorithms */
function swap_files(one, two) {
    var file_one = document.getElementById("file_"+one);
    var file_two = document.getElementById("file_"+two);

    for(var i=1; i < 5; ++i) {
        var tmp = file_one.cells[i].innerHTML;
        file_one.cells[i].innerHTML = file_two.cells[i].innerHTML;
        file_two.cells[i].innerHTML = tmp;
    }
}

function swap_directories(one, two) {
    var dir_one = document.getElementById("dir_"+one);
    var dir_two = document.getElementById("dir_"+two);

    console.log( dir_one.cells[1].innerHTML + " -- " + dir_two.cells[1].innerHTML );

    for(var i=1; i < 5; ++i) {
        var tmp = dir_one.cells[i].innerHTML;
        dir_one.cells[i].innerHTML = dir_two.cells[i].innerHTML;
        dir_two.cells[i].innerHTML = tmp;
    }
}

function quicksort(low, high) {
    if( low >= high ) return;

    var p = partition(low, high);
    quicksort(low, p);
    quicksort(p+1, high);
}

function partition(low, high) {
    var pivot = document.getElementById("dir_"+low);

    var i = low - 1;
    var j = high + 1;

    for(;;) {

        do { ++i; }
        while (
            document.getElementById("dir_"+i).cells[cell_name].innerHTML.toLowerCase()
            <
            pivot.cells[cell_name].innerHTML.toLowerCase()
        );

        do { --j; }
        while (
            document.getElementById("dir_"+j).cells[cell_name].innerHTML.toLowerCase()
            >
            pivot.cells[cell_name].innerHTML.toLowerCase()
        );

        if( i >= j ) return j;

        console.log(" i : " + i + " -- j : " + j);
        swap_directories(j, i);
    }
}
