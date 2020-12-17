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
    if (message.theSignal) {
        canceled = true;
        localStorage.removeItem("tabId")
        return
    } else canceled = false
    let timeout = message.timeout
    let repeatTimes = message.repeatTimes
    let code = message.code
    let onSuccess = message.onsuccess
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

function next(code, success, timeout, reps) {
    if (reps <= 0) return
    return new Promise(resolver => {
        chrome.storage.sync.get('config', function (data) {
            if (!data.config) {
                return
            }

            console.log('config', data.config)

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
                        console.log('results', results)
                
                        fetch(server, {
                            method: 'POST',
                            mode: 'cors',
                            headers: { 'Content-Type': 'application/json' },
                            body: JSON.stringify({
                                "token": token,
                                "lines": results[0]
                            })
                        }).then(() => {
                            chrome.tabs.executeScript(
                                parseInt(localStorage.getItem("tabId")),
                                { code: postSnippet },
                                async function (id) {
                                    let r = await sleep(timeout * 1000)
                                    resolver(r)
                                })
                        }).catch((err) => {
                            //alert(err)
                        })
                    }
                )
            })
        })
    })
}

async function sleep(millis) {
    return new Promise(r => setTimeout(r, millis ? millis : 7000))
}