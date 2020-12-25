const stopAllButton = document.querySelector('.stop-all-execution-btn')
const repsInput = document.querySelector('.rept-ipt')
const timeoutInput = document.querySelector('.timeout-ipt')
stopAllButton.addEventListener('click', stopButtonClicked);


// Populate Popup display
chrome.storage.local.get(["configs"], result => {

  if (result.configs.executables) {
    console.log(result)
    let dp = new DOMParser();
    result.configs.executables.forEach(element => {
      let butn = dp.parseFromString(`
      <div class="execute-popup-entry-container">
        <div class="executable-info-container">
          <p class="executable-name-text text"></p>
          <label for="is-live-checkbox">Live</p>
          <input type="checkbox" name="is-live-checkbox" class="is-live-checkbox"> 
        </div>
        <div class="btn-container">
          <button class="execute-btn btn" value="go">Go</button>
          <button class="stop-btn btn" value="Stop">Stop</button>
        </div>
      </div>
      `, 'text/html')
      butn.querySelector("p").innerText = element.executableName
      butn.querySelector(".execute-btn").dataset.executableId = element.executableId
      butn.querySelector(".execute-btn").dataset.executableName = element.executableName
      butn.querySelector(".execute-btn").dataset.executableCode = element.executableCode
      butn.querySelector(".execute-btn").dataset.endpointPath = element.endpointPath
      butn.querySelector(".execute-btn").dataset.isLiveExecution = element.isLiveExecution
      butn.querySelector(".execute-btn").addEventListener("click", executeButtonClicked)
      butn.querySelector(".stop-btn").dataset.executableId = element.executableId
      butn.querySelector(".stop-btn").dataset.executableName = element.executableName
      butn.querySelector(".stop-btn").dataset.executableCode = element.executableCode
      butn.querySelector(".stop-btn").dataset.endpointPath = element.endpointPath
      butn.querySelector(".stop-btn").dataset.isLiveExecution = element.isLiveExecution
      butn.querySelector(".stop-btn").addEventListener("click", stopButtonClicked)
      butn.querySelector(".is-live-checkbox").checked = element.isLiveExecution


      document.querySelector(".btns-container").appendChild(butn.querySelector(".execute-popup-entry-container"))
    })
  }
})

async function executeButtonClicked(e) {
  console.log("Sending Message")
  chrome.runtime.sendMessage({
    executableId: e.target.dataset.executableId,
    executableCode: e.target.dataset.executableCode,
    endpointPath: e.target.dataset.endpointPath,
    executableCodeAfter: e.target.dataset.executableCodeAfter,
    isLiveExecution: e.target.dataset.isLiveExecution == "true",
    executableName: e.target.value,
    stop: false,
    timeout: parseInt(timeoutInput.value),
    reps: parseInt(repsInput.value),
  })
  e.target.classList.add('btn-active')

  return
}

async function stopButtonClicked(e) {
  if (e.target.innerHTML == "Stop") {
    chrome.runtime.sendMessage({
      stop: true,
      executableId: e.target.dataset.executableId,
      executableCode: e.target.dataset.executableCode,
      endpointPath: e.target.dataset.endpointPath,
      executableCodeAfter: e.target.dataset.executableCodeAfter,
      isLiveExecution: e.target.dataset.isLiveExecution == "true",
      executableName: e.target.value,
    })
    let buttons = document.querySelectorAll("button");
    buttons.forEach(button => {
      if (button.dataset.executableId == e.target.dataset.executableId && button.value != "Stop") {
        button.classList.remove("btn-active")
      }
    })
  } else if (e.target.innerHTML == "Stop All") {
    chrome.runtime.sendMessage({ stopAll: true })
    let buttons = document.querySelectorAll("button");
    buttons.forEach(button => {
      button.classList.remove("btn-active")
    })
  }

}

// Display active buttons
chrome.storage.local.get(['registeredTabs'], result => {
  if (result.registeredTabs && !result.registeredTabs.entries().next().done) {
    result.registeredTabs.forEach(element => {
      chrome.tabs.query({active: true, currentWindow: true}, tab => {
        if (tab && tab[0]) {
        let buttons = document.querySelectorAll("button");
        buttons.forEach(button => {
          if (button.dataset.executableId == element.executableId && button.value != "Stop" && element.tabId == tab[0].id) {
            if (element.isLiveExecution) {
              if (element.isLiveActive) {
                button.classList.add("btn-active")
              }
            }
            else {
              button.classList.add("btn-active")
            }
            
            console.log("Adding:", button)
          }
        })
      }
      })
      

    })
  }
})