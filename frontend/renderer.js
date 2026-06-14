// No require statement needed - using window.boothunready

const state = {
  devices: [],
  results: [],
  selectedIndex: -1,
  scanState: 'idle',
  filterPass: false,
  filterFail: false,
  selectedDevices: [],
  searchQuery: '' // Add search query to state
};

const elements = {
  fileCountLabel: document.getElementById('fileCountLabel'),
  deviceList: document.getElementById('deviceList'),
  runScan: document.getElementById('runScan'),
  statusText: document.getElementById('statusText'),
  fileList: document.getElementById('fileList'),
  detailsView: document.getElementById('detailsView'),
  filterPass: document.getElementById('filterPass'),
  filterFail: document.getElementById('filterFail'),
  scanLabel: document.querySelector('.scan-label'),
  scanStatus: document.getElementById('scanStatus'),
  fileSearch: document.getElementById('fileSearch'),
  scanFolderBtn: document.getElementById('scanFolderBtn'),
  parseXmlBtn: document.getElementById('parseXmlBtn'),
  folderModal: document.getElementById('folderModal'),
  modalFolderPath: document.getElementById('modalFolderPath'),
  modalChooseFolder: document.getElementById('modalChooseFolder'),
  confirmFolderBtn: document.getElementById('confirmFolderBtn'),
  cancelFolderBtn: document.getElementById('cancelFolderBtn')
};

let selectedFolderPath = '';

// Modal functions
function openFolderModal() {
  if (elements.folderModal) {
    elements.folderModal.style.display = 'flex';
  }
}

function closeFolderModal() {
  if (elements.folderModal) {
    elements.folderModal.style.display = 'none';
  }
}

function formatValue(value) {
  if (value === null || value === undefined || value === '') {
    return 'Unknown';
  }

  if (Array.isArray(value)) {
    return value.join(', ');
  }

  return String(value);
}

function compatibleLabel(fileResult) {
  const entries = Object.values(fileResult.compatibility || {});
  if (entries.length === 0) {
    return { text: 'No devices', tone: 'warn' };
  }

  const passing = entries.filter((entry) => entry.compatible).length;
  if (passing === entries.length) {
    return { text: 'All pass', tone: 'good' };
  }

  if (passing === 0) {
    return { text: 'Blocked', tone: 'bad' };
  }

  return { text: `${passing}/${entries.length} pass`, tone: 'warn' };
}

function escapeHtml(value) {
  return String(value)
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/\"/g, '&quot;')
    .replace(/'/g, '&#39;');
}

function setScanState(stateName, label) {
  state.scanState = stateName;

  if (stateName === 'running') {
    if (elements.scanLabel) elements.scanLabel.style.display = 'none';
    if (elements.scanStatus) elements.scanStatus.style.display = 'inline';
    if (elements.runScan) elements.runScan.disabled = true;
  } else {
    if (elements.scanLabel) elements.scanLabel.style.display = 'inline';
    if (elements.scanStatus) elements.scanStatus.style.display = 'none';
    if (elements.runScan) elements.runScan.disabled = false;
  }
}

function populateDeviceSelect() {
  if (!elements.deviceList) return;
  elements.deviceList.innerHTML = '';
  state.devices.forEach((deviceName) => {
    const checkboxDiv = document.createElement('label');
    checkboxDiv.className = 'device-checkbox';
    
    const checkbox = document.createElement('input');
    checkbox.type = 'checkbox';
    checkbox.value = deviceName;
    checkbox.addEventListener('change', () => {
      if (checkbox.checked) {
        if (!state.selectedDevices.includes(deviceName)) {
          state.selectedDevices.push(deviceName);
        }
      } else {
        state.selectedDevices = state.selectedDevices.filter(d => d !== deviceName);
      }
      renderFiles();
    });
    
    const label = document.createElement('label');
    label.textContent = deviceName;
    label.style.cursor = 'pointer';
    
    checkboxDiv.appendChild(checkbox);
    checkboxDiv.appendChild(label);
    elements.deviceList.appendChild(checkboxDiv);
  });
}

function renderFiles() {
  let files = state.results || [];
  
  // Apply search filter
  if (state.searchQuery) {
    files = files.filter(fileResult => {
      const fileName = fileResult.path.split(/[\\/]/).pop().toLowerCase();
      return fileName.includes(state.searchQuery);
    });
  }
  
  // Apply device/filter filters ONLY after scan is complete
  if (state.scanState !== 'running') {
    if (state.filterPass || state.filterFail) {
      files = files.filter(fileResult => {
        const compatibility = fileResult.compatibility || {};
        const devicesToCheck = state.selectedDevices.length > 0 ? state.selectedDevices : Object.keys(compatibility);
        
        if (state.filterPass) {
          return devicesToCheck.every(device => compatibility[device]?.compatible);
        }
        
        if (state.filterFail) {
          return devicesToCheck.some(device => !compatibility[device]?.compatible);
        }
        
        return true;
      });
    }
  }

  if (elements.fileCountLabel) {
    elements.fileCountLabel.textContent = `${files.length} file${files.length === 1 ? '' : 's'} Pass`;
  }

  if (files.length === 0) {
    if (elements.fileList) {
      elements.fileList.className = 'file-list empty-state';
      if (state.results.length === 0) {
        elements.fileList.textContent = 'No scan has been run yet.';
      } else if (state.searchQuery) {
        elements.fileList.textContent = `No files match "${state.searchQuery}".`;
      } else {
        elements.fileList.textContent = 'No files match the current filters.';
      }
    }
    if (elements.detailsView) {
      elements.detailsView.className = 'details empty-state';
      elements.detailsView.textContent = 'Select a file to see format details.';
    }
    state.selectedIndex = -1;
    return;
  }

  if (elements.fileList) {
    elements.fileList.className = 'file-list';
    elements.fileList.innerHTML = '';

    files.forEach((fileResult, displayIndex) => {
      const originalIndex = state.results.indexOf(fileResult);
      
      const compatibility = fileResult.compatibility || {};
      const devicesToCheck = state.selectedDevices.length > 0 ? state.selectedDevices : Object.keys(compatibility);
      const passing = devicesToCheck.filter(device => compatibility[device]?.compatible).length;
      const total = devicesToCheck.length;
      
      let summary = { text: 'No devices', tone: 'warn' };
      if (passing === total) {
        summary = { text: 'All pass', tone: 'good' };
      } else if (passing === 0) {
        summary = { text: 'Blocked', tone: 'bad' };
      } else {
        summary = { text: `${passing}/${total} pass`, tone: 'warn' };
      }
      
      const card = document.createElement('button');
      card.type = 'button';
      card.className = `file-card ${originalIndex === state.selectedIndex ? 'active' : ''}`;
      
      const fileName = fileResult.path.split(/[\\/]/).pop();
      
      let displayName = escapeHtml(fileName);
      if (state.searchQuery) {
        const regex = new RegExp(`(${escapeHtml(state.searchQuery)})`, 'gi');
        displayName = displayName.replace(regex, '<mark>$1</mark>');
      }
      
      card.innerHTML = `
        <div class="file-topline">
          <div class="file-name">${displayName}</div>
          <span class="pill ${summary.tone}">${summary.text}</span>
        </div>
        <div class="file-meta">
          <span>${formatValue(fileResult.format)}</span>
        </div>
      `;

      card.addEventListener('click', () => {
        state.selectedIndex = originalIndex;
        renderFiles();
        renderDetails();
      });

      elements.fileList.appendChild(card);
    });
  }
}

function renderDetails() {
  const fileResult = state.results[state.selectedIndex];
  if (!fileResult) {
    if (elements.detailsView) {
      elements.detailsView.className = 'details empty-state';
      elements.detailsView.textContent = 'Select a file to see format details.';
    }
    return;
  }

  if (elements.detailsView) {
    elements.detailsView.className = 'details';

    const compatibility = fileResult.compatibility || {};
    const deviceEntries = Object.entries(compatibility);

    elements.detailsView.innerHTML = `
      <div class="detail-block">
        <div class="detail-grid">
          <div class="detail-row"><span>Format</span><strong>${formatValue(fileResult.format)}</strong></div>
          <div class="detail-row"><span>Bit depth</span><strong>${formatValue(fileResult.bit_depth)} bit</strong></div>
          <div class="detail-row"><span>Sample rate</span><strong>${formatValue(fileResult.sample_rate)} Hz</strong></div>
          <div class="detail-row"><span>Bitrate</span><strong>${formatValue(fileResult.bitrate)} bps</strong></div>
          <div class="detail-row"><span>Encoding</span><strong>${formatValue(fileResult.encoding)}</strong></div>
          <div class="detail-row"><span>Devices checked</span><strong>${deviceEntries.length}</strong></div>
        </div>
      </div>
      <div class="detail-block">
        <strong>Per-device compatibility</strong>
        ${deviceEntries.map(([deviceName, result]) => {
          const tone = result.compatible ? 'good' : 'bad';
          const label = result.compatible ? 'Compatible' : 'Incompatible';
          const reasons = result.reasons && result.reasons.length > 0
            ? `<ul class="reason-list">${result.reasons.map((reason) => `<li>${escapeHtml(reason)}</li>`).join('')}</ul>`
            : '<p class="status">No failures reported.</p>';

          return `
            <div class="device-result">
              <div class="file-topline">
                <strong>${escapeHtml(deviceName)}</strong>
                <span class="pill ${tone}">${label}</span>
              </div>
              ${reasons}
            </div>
          `;
        }).join('')}
      </div>
    `;
  }
}

async function bootstrap() {
  state.devices = await window.boothunready.getDevices();
  populateDeviceSelect();
  setScanState('ready', 'Ready');

  // Search input event listener
  if (elements.fileSearch) {
    elements.fileSearch.addEventListener('input', (e) => {
      state.searchQuery = e.target.value.toLowerCase();
      renderFiles();
    });
  }

  // Filter button event listeners
  if (elements.filterPass) {
    elements.filterPass.addEventListener('click', () => {
      state.filterPass = !state.filterPass;
      state.filterFail = false;
      elements.filterPass.classList.toggle('active');
      if (elements.filterFail) elements.filterFail.classList.remove('active');
      renderFiles();
    });
  }

  if (elements.filterFail) {
    elements.filterFail.addEventListener('click', () => {
      state.filterFail = !state.filterFail;
      state.filterPass = false;
      elements.filterFail.classList.toggle('active');
      if (elements.filterPass) elements.filterPass.classList.remove('active');
      renderFiles();
    });
  }

  if (elements.runScan) {
    elements.runScan.addEventListener('click', runScan);
  }
  
  // Menu button listeners
  if (elements.scanFolderBtn) {
    elements.scanFolderBtn.addEventListener('click', openFolderModal);
  }
  
  if (elements.parseXmlBtn) {
    elements.parseXmlBtn.addEventListener('click', async () => {
      const xmlPath = await window.boothunready.selectXml();
      if (!xmlPath) return;

      setScanState('running', 'Parsing XML...');
      if (elements.statusText) elements.statusText.textContent = 'Parsing rekordbox XML and scanning tracks...';

      try {
        const devicesToScan = state.selectedDevices.length > 0 ? state.selectedDevices : null;
        const report = await window.boothunready.runXmlScan(xmlPath, devicesToScan);
        state.results = report.files || [];
        state.selectedIndex = state.results.length > 0 ? 0 : -1;

        state.filterPass = false;
        state.filterFail = false;
        state.searchQuery = '';

        if (elements.filterPass) elements.filterPass.classList.remove('active');
        if (elements.filterFail) elements.filterFail.classList.remove('active');
        if (elements.fileSearch) elements.fileSearch.value = '';

        renderFiles();
        renderDetails();
        if (elements.statusText) {
          elements.statusText.textContent = `XML scan complete. ${state.results.length} file${state.results.length === 1 ? '' : 's'} analyzed.`;
        }
        setScanState('ready', 'Ready');
      } catch (error) {
        if (elements.statusText) elements.statusText.textContent = error.message || 'XML scan failed.';
        setScanState('idle', 'Error');
      }
    });
  }
  
  // Modal button listeners
  if (elements.modalChooseFolder) {
    elements.modalChooseFolder.addEventListener('click', async () => {
      const selected = await window.boothunready.selectFolder();
      if (selected) {
        if (elements.modalFolderPath) elements.modalFolderPath.value = selected;
        selectedFolderPath = selected;
      }
    });
  }
  
  if (elements.confirmFolderBtn) {
    elements.confirmFolderBtn.addEventListener('click', () => {
      if (selectedFolderPath) {
        closeFolderModal();
        runScan();
      } else {
        if (elements.statusText) elements.statusText.textContent = 'Please select a folder first.';
      }
    });
  }
  
  if (elements.cancelFolderBtn) {
    elements.cancelFolderBtn.addEventListener('click', closeFolderModal);
  }
  
  // Close modal when clicking outside
  if (elements.folderModal) {
    elements.folderModal.addEventListener('click', (e) => {
      if (e.target === elements.folderModal) {
        closeFolderModal();
      }
    });
  }
  
  renderFiles();
}

// Window controls
document.getElementById('minimizeBtn')?.addEventListener('click', () => {
  window.boothunready.minimizeWindow();
});

document.getElementById('closeBtn')?.addEventListener('click', () => {
  window.boothunready.closeWindow();
});

document.getElementById('maximizeBtn')?.addEventListener('click', () => {
  window.boothunready.maximizeWindow();
});

async function runScan() {
  if (!selectedFolderPath) {
    if (elements.statusText) elements.statusText.textContent = 'Select a folder before running a scan.';
    openFolderModal();
    return;
  }
  
  setScanState('running', 'Scanning');
  if (elements.statusText) elements.statusText.textContent = 'Scanning files...';

  try {
    const devicesToScan = state.selectedDevices.length > 0 ? state.selectedDevices : null;
    const report = await window.boothunready.runScan(selectedFolderPath, devicesToScan);
    state.results = report.files || [];
    state.selectedIndex = state.results.length > 0 ? 0 : -1;
    
    state.filterPass = false;
    state.filterFail = false;
    state.searchQuery = '';
    
    if (elements.filterPass) elements.filterPass.classList.remove('active');
    if (elements.filterFail) elements.filterFail.classList.remove('active');
    if (elements.fileSearch) elements.fileSearch.value = '';
    
    renderFiles();
    renderDetails();
    if (elements.statusText) {
      elements.statusText.textContent = `Scan complete. ${state.results.length} file${state.results.length === 1 ? '' : 's'} analyzed.`;
    }
    setScanState('ready', 'Ready');
  } catch (error) {
    if (elements.statusText) elements.statusText.textContent = error.message || 'Scan failed.';
    setScanState('idle', 'Error');
  }
}

bootstrap().catch((error) => {
  if (elements.statusText) elements.statusText.textContent = error.message || 'Failed to initialize the app.';
});