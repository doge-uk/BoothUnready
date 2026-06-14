const { contextBridge, ipcRenderer } = require('electron');
contextBridge.exposeInMainWorld('boothunready', {
  selectFolder: () => ipcRenderer.invoke('select-folder'),
  selectXml: () => ipcRenderer.invoke('select-xml'),
  getDevices: () => ipcRenderer.invoke('get-devices'),
  runScan: (folderPath, deviceName) => ipcRenderer.invoke('run-scan', folderPath, deviceName),
  runXmlScan: (xmlPath, deviceName) => ipcRenderer.invoke('run-xml-scan', xmlPath, deviceName),
  minimizeWindow: () => ipcRenderer.send('minimize-window'),
  closeWindow: () => ipcRenderer.send('close-window'),
  maximizeWindow: () => ipcRenderer.send('maximize-window')
});