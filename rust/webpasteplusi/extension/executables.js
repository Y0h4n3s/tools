// Hostname Protocol Pair
i = [];t = [];
[...document.querySelectorAll('*')].map(n => n.outerHTML.match(/.?(https?):\/\/([^\/=><?)(*#]+)/)).filter(n => n != null).map(n =>  n.splice(1,2).reverse()).
forEach(n => {i.push({hostname: n[0], protocol: n[1]})})
exists = false
for (let p = 0; p < i.length; p++) {
    for (let m = 0; m < t.length; m++) {
        if (t[m].hostname == i[p].hostname && t[m].protocol == i[p].protocol) {exists = true;break}
    }
if (!exists) {t.push(i[p]);}exists = false}
t

// ========================================================= Own Links =============================================================
temp = [];data = [];
domain = document.domain;
port = location.href.match(/.?(?:http?|wss|ssh|ftp|file)*:\/\/([a-z0-9\-._~%!$&'()*+,;=]+@)?([a-z0-9\-._~%]+|\[[a-z0-9\-._~%!$&'()*+,;=:]+\]):([0-9]+)/);
port = port != null ? port[3] : (window.location.protocol == "http" ? 80 : 443);
links = [...document.querySelectorAll("a[href]")].map(n => n.href);
links.forEach(link => {
    result = link.match(/.?[a-z][a-z0-9+\-.]*:\/\/([a-z0-9\-._~%!$&'()*+,;=]+@)?([a-z0-9\-._~%]+|\[[a-z0-9\-._~%!$&'()*+,;=:]+\])/)
if (result[2] == domain) {
    path_only = link.match(/.?([a-z][a-z0-9+\-.]*:(\/\/[^/?#]+)?)?(\/?[a-z0-9\-._~%!$&'()*+,;=@]+(\/[a-z0-9\-._~%!$&'()*+,;=:@]+)*\/?|\/)([#?]|$)/)[3] || "";   
    param = link.match(/^[^?#]+\?([^#]+)/)? link.match(/^[^?#]+\?([^#]+)/)[1] : "";

temp.push({port: port, protocol: document.location.protocol, hostname: document.domain, path_only: path_only, params: param, full_link: result.input, extracted_from: document.location.href});
}
});
[...document.querySelectorAll('*')].map(n => n.outerHTML.match(/.?(?:\b)(href|src)=(["'`](([/.]([^?'`"]*)\?)([^"'`]*))).?/)).forEach(a => {
    if (a != null) {
       temp.push({port: port, protocol: document.location.protocol, hostname: document.domain, path_only: "/" + (a[5]? a[5]: ""), params: a[6] ? a[6] : "", full_link: document.location.origin + "/" + (a[3]? a[3] : ""), extracted_from: document.location.href})

    }
});

links = [...document.querySelectorAll("script[src]")].map(n => n.src);
links.forEach(link => {
if (link != null ) {
    result = link.match(/.?[a-z][a-z0-9+\-.]*:\/\/([a-z0-9\-._~%!$&'()*+,;=]+@)?([a-z0-9\-._~%]+|\[[a-z0-9\-._~%!$&'()*+,;=:]+\])/)
if (result[2] == domain) {
    path_only = link.match(/.?([a-z][a-z0-9+\-.]*:(\/\/[^/?#]+)?)?(\/?[a-z0-9\-._~%!$&'()*+,;=@]+(\/[a-z0-9\-._~%!$&'()*+,;=:@]+)*\/?|\/)([#?]|$)/)? link.match(/.?([a-z][a-z0-9+\-.]*:(\/\/[^/?#]+)?)?(\/?[a-z0-9\-._~%!$&'()*+,;=@]+(\/[a-z0-9\-._~%!$&'()*+,;=:@]+)*\/?|\/)([#?]|$)/)[3] : "";
    param = link.match(/^[^?#]+\?([^#]+)/)? link.match(/^[^?#]+\?([^#]+)/)[1] : "";

temp.push({port: port, protocol: document.location.protocol, hostname: document.domain, path_only: path_only, params: param, full_link: result[2], extracted_from: document.location.href});
}}
});
exists = false;
for (let p = 0; p < temp.length; p++) {
    for (let m = 0; m < data.length; m++) {
        if (data[m].hostname == temp[p].hostname &&
data[m].protocol == temp[p].protocol &&
data[m].full_link == temp[p].full_link &&
data[m].path_only == temp[p].path_only&&
data[m].params == temp[p].params) {exists = true;break}
    }
if (!exists) {data.push(temp[p]);}exists = false;}
data

//  ==================================================Too Much Data ========================================================
data = [];
temp = [];
port = location.href.match(/.?(?:http?|wss|ssh|ftp|file)*:\/\/([a-z0-9\-._~%!$&'()*+,;=]+@)?([a-z0-9\-._~%]+|\[[a-z0-9\-._~%!$&'()*+,;=:]+\]):([0-9]+)/);
port = port != null ? port[3] : (window.location.protocol == "http" ? 80 : 443);
[...document.querySelectorAll('*')].map(n => n.outerHTML.match(/.?((?:(https?|wss|ftp|ssh|smtp|rsync|git|file):?)\/\/([\w\-.]+))(([^\s\n\b"?#<']*)([?#;][^\n\b\s]*)?).?/)).forEach(a => {
    if (a != null) {
       temp.push({full_link: a[0], link_only: a[1], protocol: a[2] || "",port: port, hostname: a[3] || "", full_path: a[4] || "", path_only: a[5] || "", params: a[6] || "", page_from: document.location.href})
    }
})

exists = false;
for (let p = 0; p < temp.length; p++) {
    for (let m = 0; m < data.length; m++) {
        if (data[m].hostname == temp[p].hostname && 
data[m].protocol == temp[p].protocol && 
data[m].full_link == temp[p].full_link && 
data[m].link_only == temp[p].link_only &&
data[m].full_path == temp[p].full_path && 
data[m].path_only == temp[p].path_only&& 
data[m].params == temp[p].params) {exists = true;break}
    }
if (!exists) {data.push(temp[p]);}exists = false}
data

