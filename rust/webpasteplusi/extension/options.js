const saveButton = document.querySelector(".save-btn")
const clearButton = document.querySelector(".clear-btn")
const serverInput = document.querySelector(".server-ipt")
const badoNameInput = document.querySelector(".bado-extractor-name-ipt")
const badoCodeArea = document.querySelector("#bado-extractor-code")
const badoNonLiveCodeArea = document.querySelector("#bado-nonelive-code")
const liveCheck = document.querySelector("#live-check")
const badoEnpointPath = document.querySelector(".bado-extractor-endpointpath-ipt")


liveCheck.addEventListener("click", toggleLiveMode);

function toggleLiveMode(e) {
  badoNonLiveCodeArea.disabled = e.target.checked
}


saveButton.addEventListener("click", e => {
  let server = serverInput.value
  let executableName = badoNameInput.value
  let executableCode = badoCodeArea.value
  let endpointPath = badoEnpointPath.value
  let isLiveExecution = liveCheck.checked
  let executableCodeAfter = isLiveExecution ? "" : badoNonLiveCodeArea.value
  chrome.storage.local.get(['configs'], function (result) {
    if (result['configs'].executables.entries().next().done) {
      chrome.storage.local.set(
        {
          configs: {
            server: server,
            executables: [{
              executableId: 0,
              executableName: executableName,
              executableCode: executableCode,
              executableCodeAfter: executableCodeAfter,
              endpointPath: endpointPath,
              isLiveExecution: isLiveExecution
            }]
          }
        },
        function () {
          chrome.storage.local.get(['configs'], result => {
            console.log(result);
            updateDisplay(result)
          })
        })
    } else {

      result['configs']['executables'].push(
        {
          executableId: result['configs']['executables'][result['configs']['executables'].length - 1].executableId + 1,
          executableName: executableName,
          executableCode: executableCode,
          executableCodeAfter: executableCodeAfter,
          endpointPath: endpointPath,
          isLiveExecution: isLiveExecution
        }
      )
      result['configs']['server'] = server
      chrome.storage.local.set(
        result,
        function () {
          chrome.storage.local.get(['configs'], result => {
            console.log(result);
            updateDisplay(result)
          })
        })
    }
  })

})

function updateDisplay(data) {
  let dp = new DOMParser();
  document.querySelectorAll(".executable").forEach(element => {
    element.parentNode.removeChild(element)
  })
  if (!data.configs.executables) {
    return
  }
  data["configs"]["executables"].forEach(element => {
    let extractor = dp.parseFromString(`
    <div class="executable">
      <label for="extractor-name-ipt">Extractor Name: </label>
      <input type="text" class="extractor-name-ipt ipt" name="extractor-name-ipt" value="${element.executableName}" disabled>
      <input type="text" class="extractor-endpointpath-ipt ipt" name="extractor-endpointpath-ipt" value="${element.endpointPath}" disabled>
      <input type="hidden" name="extractor-id-ipt" class="extractor-id-ipt ipt" value="${element.executableId}" disabled>
      <textarea name="extractor-code-textarea textarea" id="extractor-code" cols="30" rows="10" placeholder="${element.executableCode}" disabled></textarea>
      <lable for="live-checker" disabled>Live</lable>
      <input type="checkbox" name="live-checker" id="live-check" class="live-check checkbox" ${element.isLiveExecution ? "checked" : ""} disabled>
      <textarea name="nonlive-code-textarea textarea" id="nonlive-code-textarea" style="display:${element.isLiveExecution ? "none" : "inline-block"}" cols="30" rows="10" placeholder="${element.executableCodeAfter}" disabled></textarea>
      <button class="delete-single-btn btn" value="Delete" >Delete </button>
    </div>
  `, "text/html")
    
    
    extractor.querySelector(".delete-single-btn").addEventListener("click", deleteme)
    document.querySelector(".added-options").appendChild(extractor.querySelector(".executable"))
  });



}

function deleteme(e) {


  let delIndex = [...document.querySelectorAll(".executable")].indexOf(e.path[1])
  chrome.storage.local.get(['configs'], result => {
    result['configs']['executables'].splice(delIndex, 1)
    chrome.storage.local.set(result, function () {
      chrome.storage.local.get(['configs'], result => {
        console.log(result);
        updateDisplay(result)
      })
    })
  })

  let element = e.target.parentNode
  console.log("deleting", element)
  element.parentNode.removeChild(element)
}

clearButton.addEventListener('click', clearStorage)

function clearStorage(e) {
  chrome.storage.local.clear(() => {
    updateDisplay({ configs: {} })
    chrome.storage.local.set({ configs: {} }, () => { })
  })
}