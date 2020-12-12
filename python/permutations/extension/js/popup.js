
const links = document.querySelector('.links')
const submitBtn = document.querySelector('.submit-links-btn')
const nextBtn = document.querySelector('.next-btn')
let tabId = null

submitBtn.addEventListener('click', function (e) {
  console.log(links)
  if (!isValidLinks(links.value)) {
    console.log("no work")
    return
  }
  storeLinks(links.value)
  goToNext()

})

nextBtn.addEventListener('click', e => goToNext(e))

function goToNext(e=null){
  let index = window.localStorage.getItem('indexAt')
  let link = window.localStorage.getItem(index)
  if (!tabId) {
    //let tab = chrome.tabs.create({'active':true, 'index':0, 'url':link})
    chrome.runtime.sendMessage({type: 'getTabId'}, res => {
      tabId = res.tabId
    })
  }
  window.localStorage.setItem('indexAt', parseInt(index) + 1)

}


function storeLinks(params) {
  let values = params.split('\n')
  let temp = {}
  values.forEach(link => {
    window.localStorage.setItem([values.indexOf(link)] ,link);
  });
  window.localStorage.setItem('indexAt', 0)
}

const isValidLinks = data => {
  if (data == "") return false;
  return true;
}


const start = () => {
  console.log(links)
}

