let buttons = document.getElementById('buttons')
chrome.storage.sync.get('config', function(data){
    if (!data.config || !data.config.snippets || data.config.snippets.length < 1){
        buttons.innerText = "No snippets set"
        return
    }

    let dp = new DOMParser()
    let liveButton = dp.parseFromString(`
        <button></button>
    `, 'text/html').querySelector('button')
        liveButton.innerText = "Live"
        buttons.appendChild(liveButton)
    let stopButton = dp.parseFromString(`
    <button></button>
`, 'text/html').querySelector('button')
    stopButton.innerText = "Stop"
    buttons.appendChild(stopButton)
    data.config.snippets.map(s => {
        console.log(s)
        if (s.name == "pagegrap") {
            liveButton.value = s.name;
            liveButton.dataset.code = s.code;
            liveButton.dataset.onsuccess = s.onsuccess;
        }
        buttons.appendChild(buttonTemplate(s))
    })
})



buttons.addEventListener('click', proceed)
let timeout
let repeatTimes
let current
let tabId = null
function proceed(e, auto=true){
    localStorage.removeItem("tabId")
    timeout = 7//parseInt(prompt("Interval Between Requests(In Seconds)"))
    repeatTimes = 30000//parseInt(prompt("How Many Repititions"))
    current = e.target
    if (e.target.innerText == "Stop"){
        chrome.runtime.sendMessage({theStopSignal: true})
        return
    }
    if (e.target.innerText == "Live") {
        chrome.runtime.sendMessage({theLiveSignal: true, code: current.dataset.code})
        return
    }
    if (!timeout || ! repeatTimes) return
    chrome.runtime.sendMessage(
        {
            theStopSignal: false,
            theLiveSignal: false,
            timeout: timeout,
            repeatTimes: repeatTimes,
            code: current.dataset.code,
            onsuccess: current.dataset.onsuccess
        })
        return  

}

function buttonTemplate(data){
    let dp = new DOMParser()
    let button = dp.parseFromString(`
        <button></button>
    `, 'text/html').querySelector('button')

    button.innerText = data.name
    button.value = data.name
    button.dataset.code = data.code
    button.dataset.onsuccess = data.onsuccess

    return button
}

