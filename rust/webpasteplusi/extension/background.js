

chrome.runtime.onInstalled.addListener(details => {

  chrome.storage.local.set({ registeredTabs: [] }, () => {
    chrome.storage.local.set({ configs: {} }, () => {
      console.log("Installed Succesfully")

    })
  })

})

let stopSignal = false
let liveSignal = false
chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  console.log("Background Recieved Message: ", message)
  if (message.stop) {
    killAThread(message)
    return
  }

  let timeout = message.timeout
  let executableCode = message.executableCode
  let executableName = message.executableName
  let executableId = message.executableId
  let endpointPath = message.endpointPath
  let isLiveExecution = message.isLiveExecution
  let server = message.server
  let reps = message.reps
  registerTab(server, executableCode, executableId, executableName, endpointPath, isLiveExecution, timeout, reps)
    .then(async response => {
      console.log("Added New Tab", response)
      if (reps >= 0 && !isLiveExecution) {
        await runTillReps(response.tabId, response.executableId, reps, timeout)
      }
      else if (isLiveExecution) {
        runLive(response.tabId, response.executableId)
      }
    })
  //runTillStop(server, executable, timeout);
})

chrome.tabs.onUpdated.addListener((tabid, change, tab) => {
  if (change.status = "complete") {
    
    chrome.storage.local.get(['registeredTabs'], result => {
      if (!result.registeredTabs.entries().next().done) {
        result.registeredTabs.forEach(async element => {
          if (element.isLiveExecution && element.isLiveActive && element.tabId == tabid) {
            console.log("Going Live On:", tabid)
            await execute(tabid, element.executableCode, element.server, 0)
          }
        })
      }
    })
  }
})

async function runLive(tabid, executableId) {
  chrome.storage.local.get(['configs'], result => {
    if (!result.configs.executables.entries().next().done)
      result.configs.executables.forEach(element => {
        if (!element.isLiveExecution) {
          console.log("Not A Live Executable")
          return
        } else {
          chrome.storage.local.get(['registeredTabs'], result => {
            if (!result.registeredTabs.entries().next().done) {
              result.registeredTabs.forEach(element => {
                if (element.isLiveExecution && element.tabId == tabid && element.executableId == executableId) {
                  element.isLiveActive = true
                  chrome.storage.local.set(result, () => {
                    console.log("Added One Live Tab")
                    document.location.reload(); 
                  })
                }
                return
              })
            }

          })
        }
      })
  })
  chrome.tabs.get(tabid, tab => {

  })
}

async function killAThread(message) {
  chrome.tabs.query({ active: true, currentWindow: true }, tab => {
    if (tab != undefined) {
      let tabid = tab[0].id
      chrome.storage.local.get(['registeredTabs'], result => {
        if (result.registeredTabs.entries().next().done) {
          return
        }
        result.registeredTabs.forEach(element => {
          if (element.tabId == tabid && element.executableId == message.executableId) {
            if (!element.isLiveExecution)
              element.shouldIStop = true
            else
              element.isLiveActive = false
          }
        });

        chrome.storage.local.set(result, () => console.log("Stopped One Job"))
      })
    }
  })
}

async function stopMe(tabid, executableId) {
  return new Promise(resolve => {
    chrome.storage.local.get(['registeredTabs'], result => {
      result.registeredTabs.forEach(element => {
        if (element.tabId == tabid && element.executableId == executableId && element.shouldIStop) {

          console.log("Removing: ", result.registeredTabs.splice(result.registeredTabs.indexOf(element), 1))
          chrome.storage.local.set({ result }, () => console.log("Finishing Final Request"))
          resolve(true)
        }
      });
      resolve(false)

    })
  })
}
async function runTillReps(tabid, executableId, reps, timeout) {

  chrome.storage.local.get(['configs'], async result => {
    if (result.configs.executables) {
      let executableCode = result.configs.executables.filter(n => n.executableId == executableId)[0].executableCode;
      for (let i = 0; i < reps && !await stopMe(tabid, executableId); i++) {
        console.log("Executing:", executableCode)
        await execute(tabid, executableCode, result.configs.server, timeout).catch(err => console.log(err))
      }
    }
  })

}


async function registerTab(server, executableCode, executableId, executableName, endpointPath, isLiveExecution, sleeptime, reps) {

  return new Promise(resolve => {
    chrome.tabs.query({ active: true, currentWindow: true }, tab => {
      if (tab != undefined) {
        let tabid = tab[0].id
        console.log(tab[0])
        chrome.storage.local.get(['registeredTabs'], result => {
          if (result.registeredTabs.entries().next().done) {
            let newData = {
              postedTo: server,
              executableCode: executableCode,
              executableId: executableId,
              executableName: executableName,
              endpointPath: endpointPath,
              reps: reps,
              sleepTime: sleeptime,
              tabId: tabid,
              shouldIStop: false,
              isLiveExecution: isLiveExecution,
              isLiveActive: false

            }
            chrome.storage.local.set(
              {
                registeredTabs: [ newData ]
              },
              () => {

              }
            )
          }
          else if (result.registeredTabs.filter(navigator => navigator.tabId == tabid && navigator.executableId == executableId).length >= 1) {
            resolve({ tabId: tabid, executableId: executableId })
          } else {
            result.registeredTabs.push(newData)
            chrome.storage.local.set(
              {
                registeredTabs: result.registeredTabs
              }
            ),
              () => {
              }

          }
          resolve({ tabId: tabid, executableId: executableId })
        })
      }
    })
  })
}




async function execute(tabid, executable, server, sleepTime) {
  return new Promise(resolve => {
    chrome.tabs.get(tabid, tab => {
      if (tab != undefined) {
        chrome.tabs.executeScript(tabid, { code: executable }, result => {
          if (result != undefined) {
            try {
              fetch(server, {
                method: "POST",
                mode: "no-cors",
                body: JSON.stringify({
                  "data": result[0]
                })
              }).then(async response => {
                resolve(sleep(sleepTime * 1000))
              }).catch(async err => {
                resolve(sleep(sleepTime * 1000))
              })
            } catch (err) {
              console.log(err)
            }
          }
        })
      }
    })
  })
}

async function sleep(millis) {
  console.log("Sleeping for:", millis)
  return new Promise(r => setTimeout(r, millis ? millis : 7000))
}

async function clearRegisteredTabs(e) {
  return new Promise(resolve => {
    resolve(
      chrome.storage.local.set({ registeredTabs: [] }, () => { })
    )
  })
}