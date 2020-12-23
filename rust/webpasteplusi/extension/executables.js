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
[...document.querySelectorAll("*")].map(n => n.outerHTML.match(/.?(?:(href|src)=['"]?)(?:(?:(?:https?|wss|ftp|ssh|smtp|rsync|git):?))?(?:\/\/github\.com)?(?:.?(\/[^\s\n\b"?#<']*)([?#;][^\n\b\s]*)?).?/)).forEach(a => {
    if (a != null) {
       console.log(a)
    }
})