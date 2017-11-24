const float_to_fixed = 2;

/* Number of cells */
const cell_name = 1;
const cell_datetime = 2;
const cell_timestamp = 3;
const cell_size = 4;
const cell_byte_size = 5;
const cell_unit = 6;

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

        if( _by_ == "time" ){
            insertion_sort(get_directories(), sort_by_time);
            insertion_sort(get_files(), sort_by_time);
        }
        else
        if( _by_ == "size" ){
            insertion_sort(get_directories(), sort_by_size);
            insertion_sort(get_files(), sort_by_size);
        }
        else {
            insertion_sort(get_directories(), sort_by_name);
            insertion_sort(get_files(), sort_by_name);
            _by_ = "name";
        }
    }
    else {
        reverse_order();
    }

    if( _ascending_ == "true" ) {
        document.getElementById("chevron").src = "/asset/open-iconic-master/svg/chevron-top.svg"
        _ascending_ = "false";
    }
    else {
        document.getElementById("chevron").src = "/asset/open-iconic-master/svg/chevron-bottom.svg"
        _ascending_ = "true";
    }
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

    for(var i=0; i < DirJSON.directories.length; ++i) {
        var dir = DirJSON.directories[i];
        var dir_tr = document.getElementById("dir_"+i);

        /* If user wants it to be sorted by size, we have to change also name / datetime place */
        if( _by_ == "size" ) {
            dir_tr.cells[cell_name].innerHTML = dir.name;
            dir_tr.cells[cell_datetime].innerHTML = dir.datetime;
            dir_tr.cells[cell_timestamp].innerHTML = dir.timestamp;
        }

        if( String(dir.hsize).includes(".") ) {
            dir_tr.cells[cell_size].innerHTML = dir.hsize.toFixed(float_to_fixed);
        }
        else {
            dir_tr.cells[cell_size].innerHTML = dir.hsize;
        }

        dir_tr.cells[cell_byte_size].innerHTML = dir.size;
        dir_tr.cells[cell_unit].innerHTML = dir.short_unit;
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
nbFile = document.getElementsByClassName("is-file").length;

if( nbDir > 0 ) {
    API_get_path(API_pathname, _by_, _ascending_);
}

//reverse_order("dir_", nbDir);

function sort_by_size(el_one, el_two) {
    return el_one.cells[cell_byte_size].innerHTML > el_two.cells[cell_byte_size].innerHTML;
}

function sort_by_time(el_one, el_two) {
    return el_one.cells[cell_timestamp].innerHTML > el_two.cells[cell_timestamp].innerHTML;
}

function sort_by_name(el_one, el_two) {
    return el_one.cells[cell_name].innerHTML.toLowerCase() > el_two.cells[cell_name].innerHTML.toLowerCase();
}

function insertion_sort(arr, sort_func) {
    var i, j, tmp;

    for(i=0; i < arr.length; ++i) {
        for(j=i; j > 0; --j) {
            if( sort_func(arr[j-1], arr[j]) ) {
                swap_elements(arr[j-1], arr[j]);
            }
            else break;
        }
    }
}

function get_directories() {
    return document.getElementsByClassName("is-directory");
}

function get_files() {
    return document.getElementsByClassName("is-file");
}

function reverse_order() {
    var dir_arr = get_directories();
    var file_arr = get_files();

    var start = 0;
    var end = dir_arr.length-1;
    while( (end - start) > 0 ) {
        swap_elements( dir_arr[start++], dir_arr[end--] );
    }

    start = 0;
    end = file_arr.length-1;

    while( (end - start) > 0 ) {
        swap_elements( file_arr[start++], file_arr[end--] );
    }
}

function swap_elements(el_one, el_two) {
    var tmp_tr = el_one.innerHTML;
    var tmp_id = el_one.id;

    el_one.innerHTML = el_two.innerHTML;
    el_one.id = el_two.id;

    el_two.innerHTML = tmp_tr;
    el_two.id = tmp_id;
}
