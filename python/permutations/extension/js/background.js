'use strict';

chrome.runtime.onInstalled.addListener(function() {
  
});

chrome.runtime.onMessage.addListener((message,sender,sendResponse) => {
  if (message.type == 'getTabId') {
    sendResponse({'tabId': sender.tab.id});
  }
})