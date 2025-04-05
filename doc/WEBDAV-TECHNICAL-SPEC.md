# WebDAV Technical Specification for OxiCloud

This document describes the technical details of OxiCloud's WebDAV implementation, including architecture, data flow, and integration with existing components.

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Data Flow](#data-flow)
4. [Integration with OxiCloud](#integration-with-oxicloud)
5. [Request Processing](#request-processing)
6. [Security Considerations](#security-considerations)
7. [Extension Points](#extension-points)
8. [Implementation Status](#implementation-status)

## Overview

OxiCloud's WebDAV implementation follows RFC 4918, enabling clients to perform file operations over HTTP. This allows desktop applications, mobile clients, and other WebDAV-compatible software to interact with OxiCloud as if it were a remote file system.

The implementation supports:
- File and folder browsing
- File uploads and downloads
- Creation, deletion, and movement of resources
- Metadata retrieval and modification

## Architecture

The WebDAV implementation follows OxiCloud's hexagonal architecture pattern:

```
┌────────────────────────────────────────────────────────────────────┐
│                           INTERFACES                               │
│                                                                    │
│  ┌───────────────────────────────────────────────────────────┐    │
│  │                      WebDAV Handler                       │    │
│  │                                                           │    │
│  │  OPTIONS │ PROPFIND │ GET │ PUT │ DELETE │ MOVE │ COPY   │    │
│  └─────────────────────────────┬─────────────────────────────┘    │
│                                 │                                  │
└─────────────────────────────────┼──────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────────┐
│                            APPLICATION                              │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                       WebDAV Adapter                        │   │
│  │                                                             │   │
│  │   XML Processing │ Protocol Translation │ DTOs Conversion   │   │
│  └──────────────────────────────┬──────────────────────────────┘   │
│                                 │                                   │
│                                 ▼                                   │
│                                                                     │
│  ┌──────────────┐  ┌───────────────┐  ┌──────────────┐  ┌───────┐  │
│  │              │  │               │  │              │  │       │  │
│  │ FileService  │  │ FolderService │  │ UsersService │  │ Other │  │
│  │              │  │               │  │              │  │       │  │
│  └──────┬───────┘  └───────┬───────┘  └──────┬───────┘  └───┬───┘  │
│         │                  │                 │              │      │
└─────────┼──────────────────┼─────────────────┼──────────────┼──────┘
          │                  │                 │              │
          ▼                  ▼                 ▼              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                             DOMAIN                                  │
│                                                                     │
│   ┌──────────┐  ┌───────────┐  ┌───────────┐  ┌────────┐  ┌──────┐ │
│   │          │  │           │  │           │  │        │  │      │ │
│   │   File   │  │  Folder   │  │   User    │  │ Share  │  │ etc. │ │
│   │          │  │           │  │           │  │        │  │      │ │
│   └──────────┘  └───────────┘  └───────────┘  └────────┘  └──────┘ │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### Key Components

1. **WebDAV Handler** (`src/interfaces/api/handlers/webdav_handler.rs`):
   - Processes HTTP requests for WebDAV methods
   - Maps WebDAV operations to appropriate service calls
   - Manages response formatting

2. **WebDAV Adapter** (`src/application/adapters/webdav_adapter.rs`):
   - Converts between WebDAV XML and OxiCloud domain objects
   - Handles parsing of WebDAV requests (PROPFIND, PROPPATCH, etc.)
   - Generates WebDAV responses with proper XML formatting

## Data Flow

A typical WebDAV request flows through the system as follows:

1. Client sends a WebDAV request (e.g., PROPFIND for directory listing)
2. `webdav_handler.rs` receives the request and authenticates the user
3. The handler identifies the operation type and passes the request to the `WebDavAdapter`
4. The adapter parses the XML request and converts it to domain objects
5. The handler calls appropriate service methods (e.g., `folder_service.list_folders()`)
6. Domain operations are performed by existing services
7. Results are passed back to the adapter for XML formatting
8. The handler returns the response with proper HTTP headers

## Integration with OxiCloud

The WebDAV implementation integrates with OxiCloud's existing services:

### File Operations
- Uses `FileService` for file uploads, downloads, and management
- Leverages existing file retrieval and storage capabilities

### Folder Operations
- Uses `FolderService` for directory listing and manipulation
- Maintains consistent behavior with the REST API

### Authentication
- Uses the same authentication mechanisms as the rest of OxiCloud
- Supports HTTP Basic Authentication for WebDAV clients

### Trash Integration
- Integrates with the trash system for file/folder deletion
- Allows WebDAV operations to use the trash feature when available

## Request Processing

### PROPFIND (Directory Listing)

```
┌─────────┐     ┌────────────────┐     ┌─────────────────┐     ┌───────────────┐
│         │     │                │     │                 │     │               │
│ Client  │────▶│ WebDAV Handler │────▶│ WebDAV Adapter  │────▶│ FolderService │
│         │     │                │     │                 │     │               │
└─────────┘     └────────────────┘     └─────────────────┘     └───────┬───────┘
                                                                       │
┌─────────┐     ┌────────────────┐     ┌─────────────────┐     ┌───────▼───────┐
│         │     │                │     │                 │     │               │
│ Client  │◀────│ WebDAV Handler │◀────│ WebDAV Adapter  │◀────│ FileService   │
│         │     │                │     │                 │     │               │
└─────────┘     └────────────────┘     └─────────────────┘     └───────────────┘
```

1. Client sends PROPFIND request with Depth header
2. Handler extracts path and depth information
3. Adapter parses the XML to determine requested properties
4. FolderService retrieves the folder and its contents
5. FileService retrieves file information if needed
6. Adapter generates XML response with all properties
7. Handler returns response with 207 Multi-Status code

### PUT (File Upload)

```
┌─────────┐     ┌────────────────┐     ┌─────────────────┐
│         │     │                │     │                 │
│ Client  │────▶│ WebDAV Handler │────▶│ FileService     │
│         │     │                │     │                 │
└─────────┘     └────────────────┘     └─────────────────┘
                                                │
┌─────────┐     ┌────────────────┐     ┌────────▼────────┐
│         │     │                │     │                 │
│ Client  │◀────│ WebDAV Handler │◀────│ Response        │
│         │     │                │     │                 │
└─────────┘     └────────────────┘     └─────────────────┘
```

1. Client sends PUT request with file contents
2. Handler extracts path and parent folder information
3. FileService uploads the file to the proper location
4. Handler returns Created (201) or No Content (204) response

## Security Considerations

1. **Authentication**
   - The WebDAV implementation uses the same authentication mechanisms as the rest of OxiCloud
   - Supports HTTP Basic Authentication for WebDAV clients
   - Enforces the same permissions model as the REST API

2. **Authorization**
   - Users can only access their own files through WebDAV
   - Shared resources maintain the same permissions as in the main application

3. **HTTPS**
   - All WebDAV traffic should be served over HTTPS to protect credentials and content

4. **Input Validation**
   - All XML inputs are strictly validated before processing
   - Path traversal attacks are prevented by proper path normalization

## Extension Points

The WebDAV implementation is designed for future expansion:

1. **Property Storage**
   - Support for custom WebDAV properties can be added via a property database

2. **CalDAV/CardDAV**
   - The architecture allows for extending to CalDAV (calendar) and CardDAV (contacts) 
   - Both protocols build on the WebDAV foundation

3. **Advanced Locking**
   - Full WebDAV locking capabilities can be implemented to support collaborative editing

## Implementation Status

Current implementation status of WebDAV methods:

| Method    | Status    | Notes                                    |
|-----------|-----------|------------------------------------------|
| OPTIONS   | Complete  | Advertises WebDAV capabilities           |
| PROPFIND  | Complete  | Full directory listing with properties   |
| GET       | Complete  | File download fully implemented          | 
| HEAD      | Complete  | Metadata retrieval implemented           |
| PUT       | Complete  | File creation and update implemented     |
| DELETE    | Complete  | Integration with trash features          |
| MKCOL     | Complete  | Directory creation implemented           |
| COPY      | Complete  | File/folder copying implemented          |
| MOVE      | Complete  | File/folder moving/renaming implemented  |
| PROPPATCH | Complete  | Property updates implemented             |
| LOCK      | Complete  | Basic locking capability implemented     |
| UNLOCK    | Complete  | Basic unlocking capability implemented   |

The implementation now provides a complete WebDAV server compatible with all standard clients. All WebDAV methods required by RFC 4918 have been implemented. Advanced features like persistent property storage may be added in future updates.