const { app, BrowserWindow, dialog, ipcMain } = require('electron');
const { spawn } = require('child_process');
const fs = require('fs');
const path = require('path');

const DEVICE_NAMES = [
  'CDJ-3000X',
  'CDJ-3000',
  'CDJ-2000NXS2',
  'XDJ-1000MK2',
  'XDJ-700',
  'CDJ-2000NXS',
  'XDJ-1000'
];

function createWindow() {
  const win = new BrowserWindow({
    width: 1480,
    height: 920,
    minWidth: 1200,
    minHeight: 760,
    backgroundColor: '#0b1020',
    title: 'DeckCheck',
    titleBarStyle: 'hidden', // or 'hiddenInset' on macOS
    frame: false,            // removes native frame entirely
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
      contextIsolation: true,
      nodeIntegration: false,
      sandbox: false
    }
  });

  win.loadFile(path.join(__dirname, 'index.html'));

  // Handle window controls
  ipcMain.on('minimize-window', () => {
    win.minimize();
  });

  ipcMain.on('close-window', () => {
    win.close();
  });

  // Optional: maximize/unmaximize
  ipcMain.on('maximize-window', () => {
    if (win.isMaximized()) {
      win.unmaximize();
    } else {
      win.maximize();
    }
  });
}

function resolveBackendCommand() {
  const explicit = process.env.BOOTHUNREADY_BACKEND_PATH;
  if (explicit && fs.existsSync(explicit)) {
    return { command: explicit, args: [] };
  }

  if (app.isPackaged) {
    const packagedExe = path.join(process.resourcesPath, 'deckcheck.exe');
    if (fs.existsSync(packagedExe)) {
      return { command: packagedExe, args: [] };
    }

    throw new Error(
      'Packaged backend not found. Rebuild with `npm run dist` after compiling the Rust release binary.'
    );
  }

  const debugExe = path.resolve(__dirname, '..', 'target', 'debug', 'deckcheck.exe');
  const debugUnix = path.resolve(__dirname, '..', 'target', 'debug', 'deckcheck');
  if (fs.existsSync(debugExe)) {
    return { command: debugExe, args: [] };
  }
  if (fs.existsSync(debugUnix)) {
    return { command: debugUnix, args: [] };
  }

  return { command: 'cargo', args: ['run', '--quiet', '--'] };
}

function runBackendScan(folderPath, deviceName) {
  return new Promise((resolve, reject) => {
    const backend = resolveBackendCommand();
    const args = backend.args.concat(['--path', folderPath]);
    if (deviceName) {
      args.push('--device', deviceName);
    }

    const child = spawn(backend.command, args, {
      cwd: path.resolve(__dirname, '..'),
      windowsHide: true,
      shell: false
    });

    let stdout = '';
    let stderr = '';

    child.stdout.on('data', (chunk) => {
      stdout += chunk.toString();
    });

    child.stderr.on('data', (chunk) => {
      stderr += chunk.toString();
    });

    child.on('error', reject);
    child.on('close', (code) => {
      if (code !== 0) {
        reject(new Error(stderr.trim() || `Backend exited with code ${code}`));
        return;
      }

      try {
        resolve(JSON.parse(stdout));
      } catch (error) {
        reject(new Error(`Failed to parse backend JSON: ${error.message}\n${stdout}\n${stderr}`));
      }
    });
  });
}

function runBackendScanXml(xmlPath, deviceName) {
  return new Promise((resolve, reject) => {
    const backend = resolveBackendCommand();
    const args = backend.args.concat(['--xml', xmlPath]);
    if (deviceName) {
      args.push('--device', deviceName);
    }

    const child = spawn(backend.command, args, {
      cwd: path.resolve(__dirname, '..'),
      windowsHide: true,
      shell: false
    });

    let stdout = '';
    let stderr = '';

    child.stdout.on('data', (chunk) => {
      stdout += chunk.toString();
    });

    child.stderr.on('data', (chunk) => {
      stderr += chunk.toString();
    });

    child.on('error', reject);
    child.on('close', (code) => {
      if (code !== 0) {
        reject(new Error(stderr.trim() || `Backend exited with code ${code}`));
        return;
      }

      try {
        resolve(JSON.parse(stdout));
      } catch (error) {
        reject(new Error(`Failed to parse backend JSON: ${error.message}\n${stdout}\n${stderr}`));
      }
    });
  });
}


ipcMain.handle('select-folder', async () => {
  const result = await dialog.showOpenDialog({
    properties: ['openDirectory', 'createDirectory']
  });

  if (result.canceled || result.filePaths.length === 0) {
    return null;
  }

  return result.filePaths[0];
});

ipcMain.handle('get-devices', async () => DEVICE_NAMES);

ipcMain.handle('run-scan', async (_event, folderPath, deviceName) => {
  return runBackendScan(folderPath, deviceName || null);
});

ipcMain.handle('select-xml', async () => {
  const result = await dialog.showOpenDialog({
    properties: ['openFile'],
    filters: [{ name: 'rekordbox XML', extensions: ['xml'] }]
  });

  if (result.canceled || result.filePaths.length === 0) {
    return null;
  }

  return result.filePaths[0];
});

ipcMain.handle('run-xml-scan', async (_event, xmlPath, deviceName) => {
  return runBackendScanXml(xmlPath, deviceName || null);
});


app.whenReady().then(() => {
  createWindow();

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow();
    }
  });
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});