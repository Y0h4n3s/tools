const stopAllButton = document.querySelector('.stop-all-execution-btn')

stopAllButton.addEventListener('click', stopButtonClicked);

chrome.storage.local.get(["configs"], result => {

  if (result.configs.executables) {
    console.log(result)
    let dp = new DOMParser();
    result.configs.executables.forEach(element => {
      let butn = dp.parseFromString(`
      <div class="execute-popup-entry-container">
      <p class="executable-name-text text"></p>
      <button class="execute-btn btn" value="Save">Go</button>
      <button class="stop-btn btn" value="Save">Stop</button>
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
    executableId: e.target.dataset.executableId,
    isLiveExecution: e.target.dataset.isLiveExecution == "true",
    executableName: e.target.value,
    stop: false,
    live: false,
    timeout: 3,
    reps: 200,
  })
  return
}

async function stopButtonClicked(e) {
  if (e.target.innerHTML == "Stop") {
    chrome.runtime.sendMessage({
      stop: true,
      executableId: e.target.dataset.executableId,
      executableCode: e.target.dataset.executableCode,
      endpointPath: e.target.dataset.endpointPath,
      executableId: e.target.dataset.executableId,
      isLiveExecution: e.target.dataset.isLiveExecution == "true",
      executableName: e.target.value,

    })
  }
}