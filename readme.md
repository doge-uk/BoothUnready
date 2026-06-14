
# DeckCheck


[![Build](https://img.shields.io/badge/build-passing-brightgreen?style=for-the-badge)](https://github.com/YOUR_USERNAME/DeckCheck/releases)
![GitHub License](https://img.shields.io/github/license/doge-uk/DeckCheck?style=for-the-badge)

**DeckCheck** is a desktop application that scans audio files and verifies compatibility with Pioneer CDJ and XDJ players. It helps DJs ensure their USB drives contain properly formatted audio files before heading to the club or gig.

## Features

- **Multi-Device Support** - Check compatibility against multiple CDJ/XDJ models simultaneously
- **Comprehensive Format Analysis** - Detects bit depth, sample rate, bitrate, and encoding
- **Visual Feedback** - Color-coded results (pass/warn/blocked) for quick assessment
- **Filtering** - Filter files by pass/fail status
- **Search** - Quick file search within scanned results
- **Detailed Reports** - View per-device compatibility issues with specific failure reasons
- **Modern Interface** - Clean, dark-themed UI with responsive design

## Usage

Launch DeckCheck and pick your scan method from the toolbar:

- **Scan Folder** - Check a single folder of audio files. Quick and simple.

- **Parse XML** - Load a rekordbox collection export. Ideal if your library spans multiple folders.
    Export it first: rekordbox → File → Export Collection in XML format.
## Supported Devices

- CDJ-3000X
- CDJ-3000
- CDJ-2000NXS2
- XDJ-1000MK2
- XDJ-700
- CDJ-2000NXS
- XDJ-1000

## Installation

### Download Pre-built Binary

1. Go to the [Releases](https://github.com/YOUR_USERNAME/DeckCheck/releases) page
2. Download the latest `.exe` installer for Windows
3. Run the installer and follow the setup wizard

### Build from Source

#### Prerequisites

- [Node.js](https://nodejs.org/) (v20 or later)
- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [Git](https://git-scm.com/)

#### Steps

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/DeckCheck.git
cd DeckCheck

# Build the Rust backend
cargo build --release

# Install frontend dependencies and build
cd frontend
npm install
npm run dist
```

The built application will be in `frontend/dist/`.

## Usage

1. **Launch DeckCheck** - Run the application from your Start Menu or desktop shortcut

2. **Select Folder** - Click "Browse" and choose the folder containing your audio files (typically your USB drive)

3. **Choose Devices** - Select which Pioneer players you want to check compatibility against

4. **Run Scan** - Click "Search" to begin analyzing all audio files

5. **Review Results** - 
   - Green "All pass" - File is compatible with all selected devices
   - Yellow "X/Y pass" - File works on some devices but not all
   - Red "Blocked" - File won't work on any selected device

6. **Inspect Details** - Click any file to see detailed compatibility information and specific failure reasons

## Development

### Project Structure

```
DeckCheck/
├── frontend/           # Electron frontend application
│   ├── index.html      # Main UI
│   ├── renderer.js     # Frontend logic
│   ├── styles.css      # Styling
│   ├── main.js         # Electron main process
│   └── preload.js      # IPC bridge
├── src/                # Rust backend
│   ├── main.rs         # CLI entry point
│   ├── scanner.rs      # Audio file analysis
│   ├── config.rs       # Device specifications
│   ├── report.rs       # JSON output formatting
│   └── batch.rs        # Batch processing
└── Cargo.toml          # Rust dependencies
```

### Running in Development

```bash
# Terminal 1 - Run Rust backend in watch mode
cargo run

# Terminal 2 - Run Electron frontend
cd frontend
npm start
```

### Building for Production

```bash
# Build Rust release binary
cargo build --release

# Build Electron distributable
cd frontend
npm run dist
```

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Acknowledgments

- [Symphonia](https://github.com/pdeljanov/Symphonia) - Pure Rust audio decoding library
- [Electron](https://www.electronjs.org/) - Cross-platform desktop framework
- [Pioneer DJ](https://www.pioneerdj.com/) - For device specifications

---

**Disclaimer**: DeckCheck is an independent tool and is not affiliated with, endorsed by, or connected to Pioneer DJ Corporation. Device specifications are based on publicly available documentation and community testing.
