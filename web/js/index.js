var location_pathname = window.location.pathname;

/* Cut window location pathname after "/home" (5 chars) */
var API_pathname = location_pathname.substring(5);

/* Get URL Params, Queries */
var urlParams = new URLSearchParams(window.location.search);

/* DEBUG */
console.log( location_pathname );
console.log( API_pathname );

console.log(urlParams.get("by"));
console.log(urlParams.get("ascending"));

API_get_path(API_pathname, urlParams.get("by"), urlParams.get("ascending"));

function API_get_path(path, sort_method, ascending) {
    var r = new XMLHttpRequest();

    if( sort_method === undefined ) {
         sort_method = "name";
    }

    if( ascending === undefined ) {
        ascending = true;
    }

    var request_url = "/api/path" + path + "?by=" + sort_method + "&ascending=" + ascending;
    r.open("GET", request_url, true);

    r.onreadystatechange = function () {
        if (r.readyState != 4 || r.status != 200) return;
        console.log(r.responseText);

        /* What to do with responseText */
        /* Here update doc */

        //alert("Success: " + r.responseText);
    };

    r.send();
}
