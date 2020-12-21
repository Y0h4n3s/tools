chrome.runtime.onInstalled.addListener(function () {

    chrome.declarativeContent.onPageChanged.removeRules(undefined, function () {
        chrome.declarativeContent.onPageChanged.addRules([{
            conditions: [new chrome.declarativeContent.PageStateMatcher({
                //pageUrl: {hostEquals: 'github.com'}
            })],
            actions: [new chrome.declarativeContent.ShowPageAction()]
        }])
    })
})

let canceled = false
chrome.runtime.onMessage.addListener(async function (message, sender, sendResponse) {
    if (message.theStopSignal) {
        canceled = true;
        localStorage.removeItem("tabId")
        return
    } else canceled = false
    let timeout = message.timeout
    let repeatTimes = message.repeatTimes
    let code = message.code
    let onSuccess = message.onsuccess
    if (message.theLiveSignal) {
    try{
        goLive(code);
    } catch(error) {
        goLive(code);
    }
    }
    for (let i = 0; i < repeatTimes; i++) {
        if(canceled)break
        try {
        await next(code, onSuccess, timeout, repeatTimes).then(v => { })
        } catch(error) {
            continue
        }
    }
    localStorage.removeItem("tabId")
    return
})

async function goLive(code) {
    localStorage.setItem("liveCode", code);
    chrome.tabs.query({active: true, currentWindow: true}, (tab) => {
        if (tab[0] != null) {
        let liveTabs = (localStorage.getItem("liveTabs")? localStorage.getItem("liveTabs") : "")  + String(tab[0].id)+ ",";
        localStorage.setItem("liveTabs", liveTabs)
        chrome.tabs.onUpdated.addListener((tabid, info, tab) => {
            let tabs = liveTabs.split(",");
            let snippet = localStorage.getItem("liveCode");
            let server = "http://" + "127.0.0.1:8889" || 'localhost'
            let token = "iloveweb"
            if (tabs.find(n => n == String(tabid))) {
                chrome.tabs.executeScript(
                    tabid,
                    { code: snippet },

                    async function (results) {
                    //console.log("results:", results[0])
                        var dat = JSON.stringify({

                                                    "token": token,
                                                    "everything_else": results[0]

                                            })
                        const response = await fetch(server, {
                        method: "POST",
                        mode: "cors",
                        headers: {
                            "Content-Type": "application/json"
                        },
                        body: dat
                        })

                })
            }
        })
    }
    })
}

function next(code, success, timeout, reps) {
    if (reps <= 0) return
    return new Promise(resolver => {
        chrome.storage.sync.get('config', function (data) {
            if (!data.config) {
                return
            }

            //console.log('config', data.config)

            let server = "http://" + data.config.server || 'localhost'
            let token = data.config.token || 'notoken'
            let snippet = code
            let postSnippet = success
            chrome.tabs.query({ active: true, currentWindow: true }, function (tab) {
                if (!localStorage.getItem("tabId")) {
                    localStorage.setItem("tabId", tab[0].id)
                }
                chrome.tabs.executeScript(
                    parseInt(localStorage.getItem("tabId")),
                    { code: snippet },

                    function (results) {
                    //console.log("results:", results[0])
                        var dat = JSON.stringify({

                                                    "token": token,
                                                    "everything_else": results[0]

                                            })
                        var xhr = new XMLHttpRequest();
                        xhr.open("POST",server,true)
                        xhr.setRequestHeader("Content-Type", "application/json")
                        xhr.onreadystatechange = function(){
                            if (xhr.readyState = XMLHttpRequest.DONE) {
                            chrome.tabs.executeScript(
                                parseInt(localStorage.getItem("tabId")),
                                { code: postSnippet },
                                async function (id) {
                                    let r = await sleep(timeout * 1000)
                                    resolver(r)
                                })
                            }
                        }
                        xhr.send(dat)
                        //console.log(dat)

                        })
                    })
            })
        })
}

async function sleep(millis) {
    return new Promise(r => setTimeout(r, millis ? millis : 7000))
}