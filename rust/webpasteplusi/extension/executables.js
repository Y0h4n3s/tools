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

// Hostname Own Links
data = [];
temp = [];
[...document.querySelectorAll("*")].map(n => n.outerHTML.match(/.?(?:(href|src)=['"]?)(?:.?(\/?\/[^\s\n\b"?#<']*)([?#;][^\n\b\s]*)?).?/)).forEach(a => {
    if (a != null) {
       temp.push({ hostname: document.domain || "",protocol: document.location.protocol || "", full_path: a[2] || "", extracted_from: a[1] || "", params: a[3] || ""})
    }
})
exists = false;
for (let p = 0; p < temp.length; p++) {
    for (let m = 0; m < data.length; m++) {
        if (

data[m].full_path == temp[p].full_path && 
data[m].extracted_from == temp[p].extracted_from&& 
data[m].params == temp[p].params) {exists = true;break}
    }
if (!exists) {data.push(temp[p]);}exists = false}
        data

// Much Data

data = [];
temp = [];
port = location.href.match(/.?(?:http?|wss|ssh|ftp)*:\/\/([a-z0-9\-._~%!$&'()*+,;=]+@)?([a-z0-9\-._~%]+|\[[a-z0-9\-._~%!$&'()*+,;=:]+\]):([0-9]+)/);
port = port != null ? port[3] : (window.location.protocol == "http" ? 80 : 443);
[...document.querySelectorAll('*')].map(n => n.outerHTML.match(/.?((?:(https?|wss|ftp|ssh|smtp|rsync|git):?)\/\/([\w\-.]+))(([^\s\n\b"?#<']*)([?#;][^\n\b\s]*)?).?/)).forEach(a => {
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
data[m].port == temp[p].port&&
data[m].path_only == temp[p].path_only&& 
data[m].params == temp[p].params) {exists = true;break}
    }
if (!exists) {data.push(temp[p]);}exists = false}
data