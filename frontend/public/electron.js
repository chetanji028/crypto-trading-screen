const { app, BrowserWindow } = require('electron');
const path = require('path');

let isDev;

async function initialize() {
  const electronIsDev = await import('electron-is-dev');
  isDev = electronIsDev.default; // Access the default export
}

async function createWindow() {
  await initialize(); // Ensure isDev is set before using it

  const win = new BrowserWindow({
    width: 1200,
    height: 800,
    webPreferences: {
      nodeIntegration: false, // Disable for security
      contextIsolation: true, // Enable for security
      preload: path.join(__dirname, 'preload.js'), // Add preload script
    },
  });

  win.loadURL(
    isDev
      ? 'http://localhost:3000'
      : `file://${path.join(__dirname, '../build/index.html')}`
  );
}

app.whenReady().then(async () => {
  await createWindow();
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

app.on('activate', async () => {
  if (BrowserWindow.getAllWindows().length === 0) {
    await createWindow();
  }
});