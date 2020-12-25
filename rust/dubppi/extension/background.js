

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
  if (message.stopAll) {
    killAllThreads()
    return
  }

  let timeout = message.timeout
  let executableCode = message.executableCode
  let executableName = message.executableName
  let executableId = message.executableId
  let endpointPath = message.endpointPath
  let executableCodeAfter = message.executableCodeAfter
  let isLiveExecution = message.isLiveExecution
  let server = message.server
  let reps = message.reps
  registerTab(server, executableCode, executableCodeAfter, executableId, executableName, endpointPath, isLiveExecution, timeout, reps)
    .then(async response => {
      console.log("Added New Tab", response)
      if (reps >= 0 && !isLiveExecution) {
        await runTillReps(response.tabId, response.executableId, endpointPath, reps, timeout)
      }
      else if (isLiveExecution) {
        runLive(response.tabId, response.executableId)
      }
    })
})

chrome.tabs.onUpdated.addListener((tabid, change, tab) => {
  if (change.status = "complete") {

    chrome.storage.local.get(['registeredTabs'], result => {

      if (result.registeredTabs && !result.registeredTabs.entries().next().done) {
        result.registeredTabs.forEach(async element => {
          if (element.isLiveExecution && element.isLiveActive && element.tabId == tabid) {
            await execute(tabid, element.executableCode, element.server, element.endpointPath, 0)
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

async function killAllThreads() {
  chrome.storage.local.get(['registeredTabs'], result => {
    if (result.registeredTabs && !result.registeredTabs.entries().next().done) {
      result.registeredTabs.forEach(tab => {
        tab.shouldIStop = true
      })
      chrome.storage.local.set(result, () => console.log("Killing Workers..."))
    }
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
            if (!element.isLiveExecution) {
              element.shouldIStop = true
            }
            else
              element.isLiveActive = false
            unregisterTab(element)
          }
        });

        chrome.storage.local.set(result, () => console.log("Stopping One Job"))
      })
    }
  })
}

function unregisterTab(element) {
  chrome.storage.local.get(['registeredTabs'], result => {
    if (result.registeredTabs && !result.registeredTabs.entries().next().done) {
      let tempTabs = [];
      result.registeredTabs.forEach(single => {
        if (!element.isLiveActive &&
          !single.isLiveActive &&
          single.tabId != element.tabid &&
          single.executableId != element.executableId) {
          tempTabs.push(single)
        }
      })
      clearRegisteredTabs()
      chrome.storage.local.set({ registeredTabs: tempTabs }, () => console.log("Removed:", element))
    }
  })
}

async function stopMe(tabid, executableId) {
  return new Promise(resolve => {
    chrome.storage.local.get(['registeredTabs'], result => {
      if (!result.registeredTabs || result.registeredTabs.entries().next().done) resolve(true)
      result.registeredTabs.forEach(element => {
        if (element.tabId == tabid && element.executableId == executableId && element.shouldIStop) {
          unregisterTab(element)
          resolve(true)
        }
      });
      resolve(false)

    })
  })
}

async function jumpMe(tabid, executableId) {
  return new Promise(resolve => {
    chrome.storage.local.get(['registeredTabs'], result => {
      if (!result.registeredTabs || result.registeredTabs.entries().next().done) resolve(false)
      result.registeredTabs.forEach(element => {
        if (element.tabId == tabid && element.executableId == executableId && element.previousState.wasSuccess && element.previousState.previousHref == element.previousState.href) {
          resolve(true)
        }
      });
      resolve(false)
    })
  })
}
async function runTillReps(tabid, executableId, endpointPath, reps, timeout) {

  chrome.storage.local.get(['configs'], async result => {
    if (result.configs.executables) {
      let executableCode = result.configs.executables.filter(n => n.executableId == executableId)[0].executableCode;
      let executableCodeAfter = result.configs.executables.filter(n => n.executableId == executableId)[0].executableCodeAfter;
      let server = result.configs.server
      try {
        for (let i = 0; i < reps && !await stopMe(tabid, executableId); i++) {
          await looper(tabid, executableCode,executableId, server, endpointPath, timeout, executableCodeAfter)
        }
      } catch (error) {
        console.log(error)
      }
      unregisterTab({tabid: tabid, executableId: executableId, isLiveActive: false})
    }
  })
}
async function looper(tabid, executableCode, executableId, server, endpointPath, timeout, executableCodeAfter) {
  return new Promise(async resolve => {
    await jumpMe(tabid, executableId).then(async jump => {
      if (jump) {
        await updateState(tabid, executableId, jump).then(async r => {
        await sleep(timeout * 1000)
          resolve(false)
        })
      } else {
        await execute(tabid, executableCode, server, endpointPath, timeout, executableCodeAfter).then(async success => {
          await updateState(tabid, executableId, success).then(r => {
            resolve(true)
          })
        })
      }
    })
  })
}
async function updateState(tabid, executableId, success) {
  return new Promise(resolve => {
    chrome.storage.local.get(['registeredTabs'], async result => {
      if (result.registeredTabs && !result.registeredTabs.entries().next().done) {
        result.registeredTabs.forEach(async element => {
          if (element.tabId == tabid && element.executableId == executableId) {
            chrome.tabs.executeScript(tabid, { code: "[document.location.href]" }, resultt => {
              element.previousState = { previousHref: element.previousState.href, href: resultt[0][0], wasSuccess: success }
              chrome.storage.local.set({ registeredTabs: result.registeredTabs }, () => console.log("State Updated"))
              resolve(true)
            })

          } 
          

        })

      } else {
        resolve(false)
      }
    })
  })
}

async function registerTab(server, executableCode, executableCodeAfter, executableId, executableName, endpointPath, isLiveExecution, sleeptime, reps) {

  return new Promise(resolve => {
    chrome.tabs.query({ active: true, currentWindow: true }, tab => {
      if (tab != undefined) {
        let tabid = tab[0].id
        console.log(tab[0])
        chrome.storage.local.get(['registeredTabs'], result => {
          let newData = {
            postedTo: server,
            executableCode: executableCode,
            executableId: executableId,
            executableName: executableName,
            endpointPath: endpointPath,
            executableCodeAfter: executableCodeAfter,
            previousState: { previousHref: "", href: "", wasSuccess: false },
            reps: reps,
            sleepTime: sleeptime,
            tabId: tabid,
            shouldIStop: false,
            isLiveExecution: isLiveExecution,
            isLiveActive: false

          }
          if (result.registeredTabs && result.registeredTabs.entries().next().done) {

            chrome.storage.local.set(
              {
                registeredTabs: [newData]
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




async function execute(tabid, executable, server, path, sleepTime, nextExecutable = null) {
  return new Promise(resolve => {
    chrome.tabs.get(tabid, tab => {
      if (tab != undefined) {
        console.log("Executing: ", executable)
        chrome.tabs.executeScript(tabid, { code: executable }, result => {
          if (result != undefined) {
            console.log("Found Result: ", result)
            try {

              fetch(server + path, {
                method: "POST",
                mode: "cors",
                headers: {
                  "Content-Type": "application/json"
                },
                body: JSON.stringify({
                  "data": result[0],
                  "endpoint_id": btoa(path)
                })
              }).then(async response => {
                if (nextExecutable == null) {
                  await sleep(sleepTime * 1000)
                  resolve(true)
                }
                else {
                  chrome.tabs.executeScript(tabid, { code: nextExecutable }, async result => {
                    await sleep(sleepTime * 1000)
                    resolve(true)
                  })
                }
              }).catch(async err => {
                await sleep(sleepTime * 1000)
                resolve(false)
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

async function clearRegisteredTabs(e = null) {
  return new Promise(resolve => {
    resolve(
      chrome.storage.local.set({ registeredTabs: [] }, () => { })
    )
  })
}


// Shortcut Listener

chrome.commands.onCommand.addListener((command) => {
  chrome.tabs.query({ active: true, currentWindow: true }, tab => {
    if (tab != undefined) {
      chrome.storage.local.get(['configs'], result => {
        if (command == ",") {
          if (!result.configs.executables || result['configs'].executables.entries().next().done) {
            return
          } else {
            result.configs.executables.forEach(element => {
              execute(tab[0].id, element.executableCode, result.configs.server, element.endpointPath, 0)
            })
          }
        } else if (
          command == "1" ||
          command == "2" ||
          command == "3"
        ) {
          result.configs.executables.forEach(element => {
            if (element.executableId == command) {
              execute(tab[0].id, element.executableCode, result.configs.server, element.endpointPath, 0)
            }
          })
        }
      })
    }
  })

})