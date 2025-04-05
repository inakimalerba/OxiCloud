# WebDAV Client Setup Guide for OxiCloud

This document provides detailed instructions for connecting desktop clients to OxiCloud using WebDAV, enabling seamless integration with your operating system's file browser.

## What is WebDAV?

WebDAV (Web Distributed Authoring and Versioning) is an extension of the HTTP protocol that allows users to collaboratively edit and manage files on remote web servers. OxiCloud implements WebDAV to provide desktop access to your files and folders.

## Connection Information

Use the following details to connect to OxiCloud via WebDAV:

- **Server URL**: `https://[your-oxicloud-server]/webdav/`
- **Username**: Your OxiCloud username
- **Password**: Your OxiCloud password

## Client Setup Instructions

### Windows

#### Windows 10/11 (File Explorer)

1. Open File Explorer
2. Right-click on "This PC" and select "Add a network location"
3. Click "Next"
4. Select "Choose a custom network location" and click "Next"
5. Enter the WebDAV URL: `https://[your-oxicloud-server]/webdav/`
6. When prompted, enter your OxiCloud username and password
7. Give the connection a name (e.g., "OxiCloud") and click "Next"
8. Click "Finish"

Your OxiCloud files will now appear as a network drive in File Explorer.

#### Alternative Method: Map Network Drive

1. Open File Explorer
2. Right-click on "This PC" and select "Map network drive"
3. Choose a drive letter
4. Enter the WebDAV URL: `https://[your-oxicloud-server]/webdav/`
5. Check "Connect using different credentials"
6. Click "Finish"
7. Enter your OxiCloud username and password

**Troubleshooting Windows Connections:**

If you experience issues connecting on Windows:

1. Open Registry Editor (regedit.exe)
2. Navigate to `HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\WebClient\Parameters`
3. Modify `BasicAuthLevel` to value `2`
4. Restart the WebClient service or restart your computer

You may also need to increase the file size limit:
1. In the same registry location, modify `FileSizeLimitInBytes` to a higher value (e.g., `4294967295` for 4GB)
2. Restart the WebClient service

### macOS

#### Finder

1. Open Finder
2. From the menu bar, click "Go" > "Connect to Server" (or press ⌘K)
3. Enter the WebDAV URL: `https://[your-oxicloud-server]/webdav/`
4. Click "Connect"
5. Enter your OxiCloud username and password
6. Click "Connect"

Your OxiCloud files will now appear as a mounted drive in Finder.

### Linux

#### GNOME (Nautilus File Manager)

1. Open Files (Nautilus)
2. Click the "+" button in the sidebar or press Ctrl+L
3. Enter the WebDAV URL: `davs://[your-oxicloud-server]/webdav/`
4. Enter your credentials when prompted
5. Click "Connect"

#### KDE (Dolphin File Manager)

1. Open Dolphin
2. In the address bar, enter: `webdavs://[your-oxicloud-server]/webdav/`
3. Enter your credentials when prompted
4. Click "Connect"

#### Command Line (davfs2)

1. Install davfs2: `sudo apt-get install davfs2` (Debian/Ubuntu) or equivalent for your distribution
2. Create a mount point: `sudo mkdir /mnt/oxicloud`
3. Edit `/etc/davfs2/secrets` and add: `/mnt/oxicloud [username] [password]`
4. Mount the WebDAV share: `sudo mount -t davfs https://[your-oxicloud-server]/webdav/ /mnt/oxicloud`

To automatically mount at boot, add to `/etc/fstab`:
```
https://[your-oxicloud-server]/webdav/ /mnt/oxicloud davfs user,rw,auto 0 0
```

### Mobile Devices

#### Android

Several apps support WebDAV connections on Android:

1. **X-plore File Manager**:
   - Install from Google Play Store
   - Tap the globe icon (Network)
   - Select "New Connection" > "WebDAV"
   - Enter server details and credentials

2. **Total Commander** with WebDAV plugin:
   - Install both from Google Play Store
   - Open the app and tap the folder icon
   - Choose "LAN/Cloud" > "WebDAV"
   - Enter your server details and credentials

#### iOS

1. **Documents by Readdle**:
   - Install from App Store
   - Tap the "+" button
   - Select "Add Connection" > "WebDAV Server"
   - Enter server URL and credentials

2. **FileBrowser**:
   - Install from App Store
   - Tap "+" to add a new connection
   - Select "WebDAV"
   - Enter server details and credentials

## Third-Party Applications

### Microsoft Office

1. Open any Office application
2. Go to File > Open
3. Click "Add a Place" and select "Office.com" or "SharePoint"
4. Enter the WebDAV URL: `https://[your-oxicloud-server]/webdav/`
5. Enter your credentials when prompted

### LibreOffice

1. Go to File > Open
2. In the file dialog, enter the WebDAV URL in the location bar
3. Enter credentials when prompted

### Desktop WebDAV Clients

- **Cyberduck** (Windows, macOS): Free, open-source WebDAV client
- **WinSCP** (Windows): Primarily an FTP client, but supports WebDAV
- **FileZilla Pro** (Windows, macOS, Linux): Supports WebDAV in the Pro version

## Performance Considerations

For optimal performance when using WebDAV:

1. **Large Files**: When working with files larger than 100MB, consider downloading them locally before editing
2. **Slow Connections**: Enable offline caching in your client when available
3. **File Locking**: Some clients support WebDAV locking to prevent conflicts

## Limitations and Known Issues

- **File Locking**: The current implementation does not support WebDAV locking operations (LOCK and UNLOCK)
- **Performance**: WebDAV may be slower than native sync clients for large file transfers
- **File Size**: Some WebDAV clients may have limitations on file sizes

## Security Considerations

WebDAV connections to OxiCloud use the same authentication mechanisms as the web interface. For enhanced security:

1. Always use HTTPS connections
2. Consider setting up two-factor authentication if supported
3. Don't save credentials on shared or public computers

## Troubleshooting

### Connection Issues

- Verify the server URL is correct and includes the `/webdav/` path
- Ensure your username and password are entered correctly
- Check if your network blocks WebDAV connections (ports 80/443)
- Verify that your OxiCloud server has WebDAV enabled

### File Operation Issues

- If you cannot upload files, check if you have write permissions
- If files appear to be corrupted, try using a different WebDAV client
- For timeout errors, increase the client timeout settings if possible

### Client-Specific Issues

For client-specific issues, consult the documentation for your WebDAV client or operating system.

## Getting Help

If you encounter issues not covered in this document, please contact your OxiCloud administrator or refer to the main OxiCloud documentation.