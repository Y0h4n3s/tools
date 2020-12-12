
const links = document.querySelector('.links')
const submitBtn = document.querySelector('.submit-links-btn')
const nextBtn = document.querySelector('.next-btn')
const prevBtn = document.querySelector('.prev-btn')
const currentLink = document.querySelector('.current-link')
let link = localStorage.getItem("currentLink")
let prevLink = localStorage.getItem("previousLink")
let nextLink = localStorage.getItem("nextLink")
let linkss = localStorage.getItem("allLinks")
currentLink.value = link
links.value = linkss
submitBtn.addEventListener('click', function (e) {
  console.log(links)
  if (!isValidLinks(links.value)) {
    console.log("no work")
    return
  }
  localStorage.setItem("allLinks", links.value)
  storeLinks(links.value)
  goToNext()

})

nextBtn.addEventListener('click', goToNext)

prevBtn.addEventListener('click', e => goToNext(e,false))
function goToNext(e=null,advance=true){
  let index = parseInt(window.localStorage.getItem('indexAt'))
  let toBePrevLink = link;
  link = window.localStorage.getItem(index)
  
  
  let prevLink = localStorage.getItem(index <  1? 0 : index - 2)
  localStorage.setItem("previousLink", prevLink)
  let nexLink = localStorage.getItem(index == 0 ? 0 : index+1)
  localStorage.setItem("nextLink", nexLink)
  localStorage.setItem("currentLink", advance ? link : prevLink)
  chrome.tabs.query({url: toBePrevLink, currentWindow: true}, function(tab) {    
    if(tab == undefined || tab[0] == undefined) {
      chrome.tabs.create({active: true, url: link})
    } else
    if (!link) {
      alert("Done!")
      clearStorage()
      return
    }
    chrome.tabs.update(tab[0].id,{active: true, selected: true, url: advance ? link : prevLink}, t => {})
  })
  if (!advance && index < 1) return localStorage.setItem('indexAt', 0)
  window.localStorage.setItem('indexAt', index + (advance? 1 : -2)) 

}


function clearStorage() {
  window.localStorage.clear()
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


