let buttons = document.getElementById('buttons')
chrome.storage.sync.get('config', function(data){
    if (!data.config || !data.config.snippets || data.config.snippets.length < 1){
        buttons.innerText = "No snippets set"
        return
    }

    data.config.snippets.map(s => {
        console.log(s)
        buttons.appendChild(buttonTemplate(s))
    })
    let dp = new DOMParser()
    let stopButton = dp.parseFromString(`
    <button></button>
`, 'text/html').querySelector('button')
    stopButton.innerText = "Stop"
    buttons.appendChild(stopButton)
})



buttons.addEventListener('click', proceed)
let timeout
let repeatTimes
let current
let tabId = null
function proceed(e, auto=true){
    localStorage.removeItem("tabId")
    if (e.target.innerText == "Stop"){
        chrome.runtime.sendMessage({theSignal: true})
        return
    }
    timeout = 3//parseInt(prompt("Interval Between Requests(In Seconds)"))
    repeatTimes = 30000//parseInt(prompt("How Many Repititions"))
    current = e.target
    if (!timeout || ! repeatTimes) return
    chrome.runtime.sendMessage(
        {
            theSignal: false,
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

