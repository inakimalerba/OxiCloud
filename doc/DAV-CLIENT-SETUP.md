# DAV Client Setup Guide for OxiCloud

This document provides detailed instructions for connecting clients to OxiCloud using WebDAV, CalDAV, and CardDAV protocols, enabling seamless integration with your operating system's file browser, calendar, and contacts applications.

## Table of Contents

- [WebDAV Setup](#webdav-setup) (File Access)
- [CalDAV Setup](#caldav-setup) (Calendar Synchronization)
- [CardDAV Setup](#carddav-setup) (Contact Synchronization)
- [Troubleshooting](#troubleshooting)

---

## WebDAV Setup

WebDAV (Web Distributed Authoring and Versioning) is an extension of the HTTP protocol that allows users to collaboratively edit and manage files on remote web servers.

### Connection Information

Use the following details to connect to OxiCloud via WebDAV:

- **Server URL**: `https://[your-oxicloud-server]/webdav/`
- **Username**: Your OxiCloud username
- **Password**: Your OxiCloud password

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

---

## CalDAV Setup

CalDAV is an extension of WebDAV specifically designed for calendar access, allowing you to synchronize calendars between different devices and applications.

### Apple Calendar (macOS/iOS)

#### macOS:

1. Open the Calendar app
2. Go to **Calendar** > **Add Account** > **Other CalDAV Account**
3. Enter the following information:
   - **Account Type**: Advanced
   - **Username**: Your OxiCloud username
   - **Password**: Your OxiCloud password
   - **Server Address**: `https://[your-oxicloud-server]/caldav`
4. Click **Sign In**
5. Select the calendars you want to sync and click **Done**

#### iOS:

1. Go to **Settings** > **Calendar** > **Accounts** > **Add Account** > **Other**
2. Tap **Add CalDAV Account**
3. Enter the following information:
   - **Server**: `https://[your-oxicloud-server]/caldav`
   - **Username**: Your OxiCloud username
   - **Password**: Your OxiCloud password
   - **Description**: OxiCloud Calendar (or any name you prefer)
4. Tap **Next**
5. Turn on **Calendars** and tap **Save**

### Thunderbird with Lightning

1. Open Thunderbird and go to the **Calendar** tab
2. Right-click in the left pane and select **New Calendar**
3. Select **On the Network** and click **Next**
4. Choose **CalDAV** as the format
5. Enter the location: `https://[your-oxicloud-server]/caldav/calendars/your-calendar-id`
6. Click **Next**
7. Enter a name for the calendar and choose a color
8. Click **Next** and then **Finish**
9. When prompted, enter your OxiCloud username and password

### Android (DAVx⁵)

1. Install [DAVx⁵](https://play.google.com/store/apps/details?id=at.bitfire.davdroid) from Google Play Store
2. Open DAVx⁵ and tap the **+** button
3. Select **Login with URL and username**
4. Enter the following information:
   - **Base URL**: `https://[your-oxicloud-server]/caldav`
   - **Username**: Your OxiCloud username
   - **Password**: Your OxiCloud password
5. Tap **Connect**
6. Select the calendars you want to sync
7. Tap the checkbox to enable syncing

### Windows (Outlook)

1. Download and install [CalDAV Synchronizer](https://caldavsynchronizer.org/)
2. Open Outlook and navigate to the **CalDAV Synchronizer** tab
3. Click **Synchronization Profiles**
4. Click **Add** to create a new profile
5. Enter the following information:
   - **Profile Name**: OxiCloud Calendar (or any name you prefer)
   - **CalDAV URL**: `https://[your-oxicloud-server]/caldav/calendars/your-calendar-id`
   - **Username**: Your OxiCloud username
   - **Password**: Your OxiCloud password
6. Click **Test or discover settings**
7. Select the Outlook calendar to sync with
8. Click **OK** to save the profile

---

## CardDAV Setup

CardDAV is an extension of WebDAV for address book access, allowing you to synchronize contacts between different devices and applications.

### Apple Contacts (macOS/iOS)

#### macOS:

1. Open the Contacts app
2. Go to **Contacts** > **Add Account** > **Other contacts account**
3. Select **CardDAV account**
4. Enter the following information:
   - **Server**: `https://[your-oxicloud-server]/carddav`
   - **Username**: Your OxiCloud username
   - **Password**: Your OxiCloud password
   - **Description**: OxiCloud Contacts (or any name you prefer)
5. Click **Sign In**

#### iOS:

1. Go to **Settings** > **Contacts** > **Accounts** > **Add Account** > **Other**
2. Tap **Add CardDAV Account**
3. Enter the following information:
   - **Server**: `https://[your-oxicloud-server]/carddav`
   - **Username**: Your OxiCloud username
   - **Password**: Your OxiCloud password
   - **Description**: OxiCloud Contacts (or any name you prefer)
4. Tap **Next**
5. Turn on **Contacts** and tap **Save**

### Thunderbird

1. Open Thunderbird and go to the **Address Book**
2. Click on **Tools** > **Address Book**
3. Go to **File** > **New** > **Remote Address Book**
4. Enter the following information:
   - **Name**: OxiCloud Contacts (or any name you prefer)
   - **URL**: `https://[your-oxicloud-server]/carddav/address-books/your-address-book-id`
5. Click **OK**
6. When prompted, enter your OxiCloud username and password

### Android (DAVx⁵)

1. Install [DAVx⁵](https://play.google.com/store/apps/details?id=at.bitfire.davdroid) from Google Play Store
2. Open DAVx⁵ and tap the **+** button
3. Select **Login with URL and username**
4. Enter the following information:
   - **Base URL**: `https://[your-oxicloud-server]/carddav`
   - **Username**: Your OxiCloud username
   - **Password**: Your OxiCloud password
5. Tap **Connect**
6. Select the address books you want to sync
7. Tap the checkbox to enable syncing

### Windows (Outlook)

1. Download and install [CardDAV Synchronizer](https://caldavsynchronizer.org/) (same tool as for CalDAV)
2. Open Outlook and navigate to the **CardDAV Synchronizer** tab
3. Click **Synchronization Profiles**
4. Click **Add** to create a new profile
5. Select **CardDAV** as the synchronization resource
6. Enter the following information:
   - **Profile Name**: OxiCloud Contacts (or any name you prefer)
   - **CardDAV URL**: `https://[your-oxicloud-server]/carddav/address-books/your-address-book-id`
   - **Username**: Your OxiCloud username
   - **Password**: Your OxiCloud password
7. Click **Test or discover settings**
8. Select the Outlook contacts folder to sync with
9. Click **OK** to save the profile

---

## Troubleshooting

### Common Issues

#### WebDAV Connection Issues

- Verify the server URL is correct and includes the `/webdav/` path
- Ensure your username and password are entered correctly
- Check if your network blocks WebDAV connections (ports 80/443)
- Verify that your OxiCloud server has WebDAV enabled

#### Calendar/Contact Sync Issues

- Verify the server URL is correct and includes the full path (`/caldav` or `/carddav`)
- Check that your OxiCloud server is accessible from your network
- Verify that your username and password are correct
- Check that the calendar or address book ID is correct
- Verify you have proper permissions to access the resource

#### Calendar Not Showing

- Verify the calendar is enabled in your client
- Check if the calendar is shared with your account
- Ensure your client supports the CalDAV protocol version

#### Contact Photos Not Syncing

- Some clients have limitations with contact photo syncing
- Verify the photo is in a supported format (usually JPEG)
- Check the size of the photo (some clients limit photo size)

### Client-Specific Issues

#### Windows File Explorer

For WebDAV issues on Windows:
- Make sure the WebClient service is running
- Increase timeout values in the registry
- Try using a third-party WebDAV client like Cyberduck

#### iOS Devices

- If you're having trouble connecting, try going to **Settings** > **Accounts & Passwords** and manually add the account from there
- For persistent issues, remove the account and add it again

#### Android

- DAVx⁵ requires battery optimization to be disabled for reliable background sync
- Go to **Settings** > **Apps** > **DAVx⁵** > **Battery** > **Unrestricted**

#### Outlook

- Make sure you have the latest version of CalDAV/CardDAV Synchronizer
- The plugin might need to be reactivated after Outlook updates

### Performance Considerations

For optimal performance when using WebDAV:

1. **Large Files**: When working with files larger than 100MB, consider downloading them locally before editing
2. **Slow Connections**: Enable offline caching in your client when available
3. **File Locking**: Some clients support WebDAV locking to prevent conflicts

### Getting Help

If you continue to experience issues, please:

1. Check the OxiCloud logs for error messages
2. Capture screenshots of the error messages
3. Contact support with details about:
   - Your client application and version
   - Steps to reproduce the issue
   - Any error messages displayed