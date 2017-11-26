/*
*
* Use .textContent when you want to access a string / text into an HTML DOM.
* While parse html to string.
* i.e: & -> .innerHTML = &amp; | .textContent = "&"
*
*/

/*
* How many numbers after dot in float number?
* Need to be defined into .toml config file, and reported here with tera template -> TODO
*/
const float_to_fixed = 2;

/*
* Associated number for cells
* TODO
*/
const cell_name = 1;
const cell_datetime = 2;
const cell_timestamp = 3;
const cell_size = 4;
const cell_byte_size = 5;
const cell_unit = 6;
const home = "/home";

var pathname = document.getElementById("api_pathname").textContent;

/* Cut window location pathname after "/home" (5 chars) */
var API_pathname = pathname.substring(home.length);

_by_ = document.getElementById("sort_by").innerHTML;
_ascending_ = ( document.getElementById("sort_ascending").innerHTML === "true" );

function th_click(th_class) {
    const fa_asc = "fa-sort-asc";
    const fa_desc = "fa-sort-desc";
    const fa_sort = "fa-sort"

    if( th_class != _by_ ) {
        var sort_function;

        switch( th_class ){
            case "name": sort_function = sort_by_name; break;
            case "time": sort_function = sort_by_time; break;
            case "size": sort_function = sort_by_size; break;
            default:
                sort_function = sort_by_name;
                th_class = "name";
        }

        var directories = get_directories();
        var files = get_files();

        quick_sort(directories, sort_function, 0, directories.length-1);
        quick_sort(files, sort_function, 0, files.length-1);

        if( _ascending_ == true ){
            document.querySelectorAll("th."+_by_+" > span > i")[0].classList.remove(fa_asc);
        }
        else {
            document.querySelectorAll("th."+_by_+" > span > i")[0].classList.remove(fa_desc);
        }

        document.querySelectorAll("th."+_by_+" > span > i")[0].classList.add(fa_sort);
        document.querySelectorAll("th."+th_class+" > span > i")[0].classList.remove(fa_sort);
        document.querySelectorAll("th."+th_class+" > span > i")[0].classList.add(fa_asc);

        _ascending_ = true;
        _by_ = th_class;
    }
    else {
        reverse_order();
        document.querySelectorAll("th."+_by_+" > span > i")[0].classList.toggle(fa_asc);
        document.querySelectorAll("th."+_by_+" > span > i")[0].classList.toggle(fa_desc);
        _ascending_ = !_ascending_;
    }
    update_breadcumb(pathname, _by_, _ascending_);
    update_queries(pathname, _by_, _ascending_);
}

function dir_click(dir_id) {
    var dir_name = document.getElementById(dir_id).cells[cell_name].textContent;
    var new_location =  pathname + "/" + dir_name + "?by=" + _by_ + "&ascending=" + _ascending_;
    window.location.href = new_location;
}

function update_level(DirJSON) {
    var level_titles = document.querySelectorAll("nav.level>div.level-item>div>p.title");

    level_titles[2].innerHTML = DirJSON.size;
    level_titles[3].innerHTML = DirJSON.elements;
}

function update_breadcumb(pathname, sort_method, ascending) {
    if( undefined ===  sort_method || sort_method === null ) {
         sort_method = "name";
    }

    if( undefined === ascending || ascending === null ) {
        ascending = true;
    }

    if( pathname[0] == "/" )
        pathname = pathname.substring(1);

    var iter = pathname.split("/");
    var bread_list = document.querySelectorAll("ul > li > a ");
    var phref = "";
    var full_phref = "";
    var i;

    for(i=0; i < iter.length; ++i){
        phref += "/" + iter[i];
        full_phref = phref + "?by=" + sort_method + "&ascending=" + ascending;
        bread_list[i].setAttribute("href", full_phref);
    }
}

function update_queries(pathname, sort_method, ascending) {
    if( undefined ===  sort_method || sort_method === null ) {
         sort_method = "name";
    }

    if( undefined === ascending || ascending === null ) {
        ascending = true;
    }

    var uri = pathname + "?by=" + sort_method + "&ascending=" + ascending;
    history.replaceState(null, null, uri);
}

function update_dirs_size(DirJSON) {
    for(var i=0; i < DirJSON.directories.length; ++i) {
        var dir = DirJSON.directories[i];
        var dir_tr = document.getElementById("dir_"+i);

        /* If user wants it to be sorted by size, we have to change also name / datetime */
        if( _by_ == "size" ){
            dir_tr.cells[cell_name].innerHTML = dir.name;
            dir_tr.cells[cell_datetime].innerHTML = dir.datetime;
            dir_tr.cells[cell_timestamp].innerHTML = dir.timestamp;
        }

        dir_tr.cells[cell_size].innerHTML = dir.hsize;
        dir_tr.cells[cell_byte_size].innerHTML = dir.size;
        dir_tr.cells[cell_unit].innerHTML = dir.short_unit;
    }
}

function fixed_size(arr, fixed_number) {
    for(var i=0; i < arr.length; ++i){
        var hsize = arr[i].cells[cell_size];
        if( hsize.innerHTML.includes(".") ){
            hsize.innerHTML = parseFloat(hsize.innerHTML).toFixed(fixed_number);
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
        var answer = r.response;
        update_level(answer);
        update_dirs_size(answer);
        fixed_size(get_directories(), float_to_fixed);
    };

    var request_url = "/api/path" + path + "?by=" + sort_method + "&ascending=" + ascending;

    r.open("GET", request_url, true);
    r.send();
}

fixed_size(get_files(), float_to_fixed);

/* Ajax call only if there is at least one directory */
nbDir = get_directories().length;
nbFile = get_files().length;

if( nbDir > 0 ) {
    API_get_path(API_pathname, _by_, _ascending_);
}

function sort_by_size(el_one, el_two) {
    return parseInt(el_one.cells[cell_byte_size].innerHTML) > parseInt(el_two.cells[cell_byte_size].innerHTML);
}

function sort_by_time(el_one, el_two) {
    return parseInt(el_one.cells[cell_timestamp].innerHTML) > parseInt(el_two.cells[cell_timestamp].innerHTML);
}

function sort_by_name(el_one, el_two) {
    return el_one.cells[cell_name].innerHTML.toLowerCase() > el_two.cells[cell_name].innerHTML.toLowerCase();
}

function partition(arr, sort_func, start, end) {
    var pivot = arr[start];
    var i = start - 1;
    var j = end + 1;

    for(;;){
        do ++i; while ( sort_func(pivot, arr[i]) );
        do --j; while ( sort_func(arr[j], pivot) );
        if ( i >= j ) return j;
        swap_elements(arr[i], arr[j]);
    }
}

function quick_sort(arr, sort_func, start, end) {
    if( start >= end ) return;
    var pivot = partition(arr, sort_func, start, end);
    quick_sort(arr, sort_func, start, pivot);
    quick_sort(arr, sort_func, pivot+1, end);
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
    while( start < end ){
        swap_elements( dir_arr[start++], dir_arr[end--] );
    }

    start = 0;
    end = file_arr.length-1;
    while( start < end ){
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
