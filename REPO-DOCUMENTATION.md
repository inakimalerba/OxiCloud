# OxiCloud Code Documentation

This document provides a detailed description of each file in the OxiCloud codebase, organized by architectural layers according to the clean hexagonal architecture pattern.

## Domain Layer

The domain layer forms the core of the application, containing business entities and repository interfaces.

### Entities

**File Entity (`src/domain/entities/file.rs`)**
- Core domain entity representing files in the system
- Implements an immutable design pattern for file operations
- Provides validation, creation, and manipulation methods for files
- Maintains both physical storage information and logical metadata
- Includes error handling via `FileError` for validation failures

**Folder Entity (`src/domain/entities/folder.rs`)**
- Represents folders/directories in the domain model
- Supports hierarchical structure with parent-child relationships
- Provides validation, creation, and update operations
- Implements immutable pattern with methods returning new instances
- Handles path resolution for proper folder hierarchy

**User Entity (`src/domain/entities/user.rs`)**
- Manages user accounts and authentication
- Provides secure password handling with Argon2 hashing
- Supports roles (Admin, User) with appropriate permissions
- Tracks storage usage and quotas
- Includes account management functions (activation, deactivation, login tracking)

**Share Entity (`src/domain/entities/share.rs`)**
- Implements file and folder sharing functionality
- Supports various permission levels (read, write, reshare)
- Provides password protection for shared resources
- Handles expiration dates for temporary sharing
- Tracks access statistics for shared resources

**Calendar Entity (`src/domain/entities/calendar.rs`)**
- Supports calendar functionality for CalDAV integration
- Manages calendar properties like name, color, and description
- Provides ownership and access control
- Supports custom properties for extended CalDAV compatibility
- Handles validation of calendar data

**Calendar Event Entity (`src/domain/entities/calendar_event.rs`)**
- Represents calendar events with properties like title, description, location
- Handles date/time management with recurrence rules
- Supports reminders and notifications
- Provides validation for event data
- Implements custom properties for CalDAV compatibility

**Trashed Item Entity (`src/domain/entities/trashed_item.rs`)**
- Manages files and folders in the trash
- Tracks original locations for restoration
- Implements automatic cleanup based on retention policies
- Provides restoration and permanent deletion functionality

### Repositories (Interfaces)

**File Repository (`src/domain/repositories/file_repository.rs`)**
- Defines the contract for file storage operations
- Abstracts storage implementation details from the domain
- Supports file creation, retrieval, updating, and deletion
- Provides methods for content streaming and file movement
- Includes trash functionality for file lifecycle management

**Folder Repository (`src/domain/repositories/folder_repository.rs`)**
- Defines the interface for folder manipulation
- Abstracts storage implementation details for directories
- Handles folder creation, listing, and hierarchy management
- Supports moving folders and retrieving path information
- Includes trash operations for folders

**User Repository (`src/domain/repositories/user_repository.rs`)**
- Defines the interface for user data persistence
- Supports user creation, retrieval, and management
- Provides authentication and session management
- Handles user preferences and settings storage
- Manages user quotas and storage usage tracking

**Share Repository (`src/domain/repositories/share_repository.rs`)**
- Defines the interface for share record management
- Handles creation and validation of share records
- Tracks permissions and expiration settings
- Provides access verification for shared resources
- Manages share revocation and updates

**Trash Repository (`src/domain/repositories/trash_repository.rs`)**
- Defines the interface for trash operations
- Manages soft deletion and restoration of resources
- Handles retention policies and automatic cleanup
- Provides listing of trashed items with metadata
- Supports permanent deletion operations

### Domain Services

**Auth Service (`src/domain/services/auth_service.rs`)**
- Provides domain-level authentication logic
- Implements password validation and hashing
- Defines authentication policies and rules
- Manages token generation and validation
- Handles security-related domain operations

**I18n Service (`src/domain/services/i18n_service.rs`)**
- Defines domain-level internationalization interface
- Provides translation lookup capabilities
- Manages localization strategies
- Supports multiple languages and fallbacks
- Handles format localization for dates, numbers, etc.

**Path Service (`src/domain/services/path_service.rs`)**
- Manages domain-level path abstractions
- Provides path validation and normalization
- Handles path traversal and resolution
- Implements path security measures
- Supports different path formats and conventions

## Application Layer

The application layer orchestrates use cases by coordinating domain objects and providing services to the interfaces layer.

### Services

**File Service (`src/application/services/file_service.rs`)**
- Implements file-related use cases
- Coordinates between repositories for file operations
- Provides file upload, download, and listing functionality
- Handles error translation between layers
- Contains business logic for file operations

**Folder Service (`src/application/services/folder_service.rs`)**
- Implements folder management use cases
- Manages folder creation, listing, and hierarchy
- Coordinates between repositories for folder operations
- Maintains folder structure integrity
- Handles error translation for folder operations

**Auth Application Service (`src/application/services/auth_application_service.rs`)**
- Manages user authentication flows
- Implements login, logout, and session management
- Handles token generation and validation
- Coordinates with user repository for verification
- Manages password reset and account recovery

**File Management Service (`src/application/services/file_management_service.rs`)**
- Provides higher-level file operations
- Manages file uploads, versions, and metadata
- Handles file operations across repositories
- Coordinates transactional file operations
- Provides advanced file searching and filtering

**File Retrieval Service (`src/application/services/file_retrieval_service.rs`)**
- Specialized service for file content retrieval
- Optimizes file reading operations
- Provides streaming and download functionality
- Implements read-specific error handling
- Supports different retrieval patterns (whole file, ranges)

**File Upload Service (`src/application/services/file_upload_service.rs`)**
- Specialized service for handling file uploads
- Manages chunked and multipart uploads
- Provides validation during upload
- Handles large file uploads efficiently
- Supports upload resumption and integrity verification

**Search Service (`src/application/services/search_service.rs`)**
- Implements file and folder search functionality
- Provides text-based content searching
- Handles metadata-based filtering
- Supports sorting and pagination of results
- Optimizes search operations for performance

**Share Service (`src/application/services/share_service.rs`)**
- Implements file and folder sharing functionality
- Creates and manages share links
- Handles permission checking for shared resources
- Manages password protection for shares
- Processes access requests for shared content

**Trash Service (`src/application/services/trash_service.rs`)**
- Implements trash can functionality
- Manages moving items to trash and restoration
- Handles automatic cleanup of expired trash
- Coordinates with repositories for trash operations
- Maintains metadata for trashed items

**Recent Service (`src/application/services/recent_service.rs`)**
- Tracks recently accessed files
- Manages user-specific recent file lists
- Handles expiration of old entries
- Provides sorting and filtering of recent files
- Coordinates with file repository for metadata

**Favorites Service (`src/application/services/favorites_service.rs`)**
- Manages user favorite files and folders
- Provides adding and removing favorites
- Handles listing and sorting of favorites
- Coordinates with repositories for data consistency
- Maintains user-specific favorite lists

**I18n Application Service (`src/application/services/i18n_application_service.rs`)**
- Handles internationalization and localization
- Provides translation lookups for UI components
- Manages locale detection and setting
- Coordinates with i18n domain service
- Supports dynamic language switching

**Storage Mediator (`src/application/services/storage_mediator.rs`)**
- Coordinates between different storage repositories
- Manages transaction coordination
- Handles path resolution between storage layers
- Provides unified view of storage subsystems
- Optimizes operations across storage types

**Batch Operations (`src/application/services/batch_operations.rs`)**
- Implements batch processing for file operations
- Handles atomic multi-file operations
- Provides transaction support for batch operations
- Manages failure handling and partial success
- Optimizes performance for bulk operations

### Ports

**Inbound Ports (`src/application/ports/inbound.rs`)**
- Defines interfaces for external systems to use
- Contains use case interfaces for application services
- Specifies contracts for UI and API interactions
- Provides clear boundaries for application functionality
- Forms the primary API for interfaces layer

**Outbound Ports (`src/application/ports/outbound.rs`)**
- Defines interfaces used by application services
- Specifies contracts that infrastructure must implement
- Allows swapping infrastructure implementations
- Maintains dependency inversion principle
- Protects application layer from external dependencies

**Auth Ports (`src/application/ports/auth_ports.rs`)**
- Defines interfaces for authentication operations
- Specifies contracts for login, validation, and sessions
- Handles token generation and verification
- Provides user identity management
- Supports different authentication methods

**Storage Ports (`src/application/ports/storage_ports.rs`)**
- Defines interfaces for storage operations
- Specifies contracts for accessing persistent storage
- Handles file system and database interactions
- Provides transaction support for storage operations
- Supports different storage backends

**File Ports (`src/application/ports/file_ports.rs`)**
- Defines interfaces for file operations
- Contains file upload and retrieval use cases
- Specifies contracts for file management
- Handles file-specific error conditions
- Supports various file operation patterns

**Favorites Ports (`src/application/ports/favorites_ports.rs`)**
- Defines interfaces for favorites functionality
- Specifies contracts for favorite management
- Handles favorite-specific operations
- Provides user-specific favorite management
- Supports different favorite organization structures

**Recent Ports (`src/application/ports/recent_ports.rs`)**
- Defines interfaces for recent files functionality
- Specifies contracts for recent file tracking
- Handles history and access patterns
- Provides user-specific recent file handling
- Supports different recency algorithms

**Share Ports (`src/application/ports/share_ports.rs`)**
- Defines interfaces for sharing functionality
- Specifies contracts for share creation and access
- Handles permission verification for shares
- Provides link generation and management
- Supports different sharing models

**Trash Ports (`src/application/ports/trash_ports.rs`)**
- Defines interfaces for trash functionality
- Specifies contracts for trash operations
- Handles trash-specific workflows
- Provides retention and cleanup interfaces
- Supports different trash implementation strategies

### DTOs

**File DTO (`src/application/dtos/file_dto.rs`)**
- Data transfer object for file entities
- Provides serialization and API representation
- Translates between domain model and external interfaces
- Includes conversions to/from domain entities
- Contains file metadata for API responses

**Folder DTO (`src/application/dtos/folder_dto.rs`)**
- Data transfer object for folder entities
- Provides folder data for API responses
- Handles serialization and API representation
- Includes conversions to/from domain entities
- Contains folder structure information

**User DTO (`src/application/dtos/user_dto.rs`)**
- Data transfer object for user information
- Provides user data for API responses
- Handles serialization with sensitive data protection
- Includes conversions to/from domain entities
- Contains user profile information

**Share DTO (`src/application/dtos/share_dto.rs`)**
- Data transfer object for share information
- Provides share data for API responses
- Handles serialization of sharing details
- Includes conversions to/from domain entities
- Contains share link and permission data

**Trash DTO (`src/application/dtos/trash_dto.rs`)**
- Data transfer object for trashed items
- Provides trash information for API responses
- Handles serialization of trash metadata
- Includes conversions to/from domain entities
- Contains restoration information

**Pagination DTO (`src/application/dtos/pagination.rs`)**
- Handles pagination for list responses
- Provides page size and number information
- Supports offset and cursor-based pagination
- Includes metadata for total items and pages
- Facilitates consistent pagination across APIs

**Favorites DTO (`src/application/dtos/favorites_dto.rs`)**
- Data transfer object for favorites
- Provides favorites data for API responses
- Handles serialization of favorite items
- Includes conversions to/from domain entities
- Contains favorite metadata and organization

**Recent DTO (`src/application/dtos/recent_dto.rs`)**
- Data transfer object for recent files
- Provides recent items data for API responses
- Handles serialization of access history
- Includes conversions to/from domain entities
- Contains timing and access metadata

**Search DTO (`src/application/dtos/search_dto.rs`)**
- Data transfer object for search results
- Provides search data for API responses
- Handles serialization of search results
- Includes query and result metadata
- Contains relevance and ranking information

**I18n DTO (`src/application/dtos/i18n_dto.rs`)**
- Data transfer object for internationalization
- Provides language and translation data
- Handles serialization of language resources
- Includes locale and preference information
- Contains translation bundle structures

### Adapters

**WebDAV Adapter (`src/application/adapters/webdav_adapter.rs`)**
- Adapts between OxiCloud domain models and WebDAV protocol
- Handles XML parsing and generation for WebDAV operations
- Implements property handling for WebDAV (PROPFIND, PROPPATCH)
- Provides WebDAV-specific error handling
- Translates between file operations and WebDAV methods

### Transactions

**Storage Transaction (`src/application/transactions/storage_transaction.rs`)**
- Manages transactional operations for storage
- Implements transaction boundaries and commits
- Provides rollback capabilities on failure
- Ensures consistency across multiple operations
- Handles transaction isolation levels

## Infrastructure Layer

This layer provides concrete implementations of repository interfaces and technical services.

### Repositories (Implementations)

**File FS Repository (`src/infrastructure/repositories/file_fs_repository.rs`)**
- Implements FileRepository interface for filesystem storage
- Manages physical file operations on disk
- Handles file content reading and writing
- Implements optimized large file handling
- Provides metadata caching for performance

**File FS Read Repository (`src/infrastructure/repositories/file_fs_read_repository.rs`)**
- Specialized repository for read-only file operations
- Optimized for high-performance file retrieval
- Implements caching for frequently accessed files
- Supports streaming of large files
- Handles content type detection and verification

**File FS Write Repository (`src/infrastructure/repositories/file_fs_write_repository.rs`)**
- Specialized repository for file write operations
- Handles atomic file writes with transaction support
- Implements optimized large file writes
- Manages file locking for concurrent writes
- Provides integrity verification for written files

**File FS Repository Trash (`src/infrastructure/repositories/file_fs_repository_trash.rs`)**
- Extends file repository with trash functionality
- Implements soft delete operations for files
- Manages restoration from trash
- Handles automatic cleanup of expired trash
- Maintains metadata for trashed files

**Folder FS Repository (`src/infrastructure/repositories/folder_fs_repository.rs`)**
- Implements FolderRepository interface for filesystem
- Creates and manages directory structures
- Handles folder listing and hierarchy traversal
- Implements folder permissions and ownership
- Provides optimization for deep folder structures

**Folder FS Repository Trash (`src/infrastructure/repositories/folder_fs_repository_trash.rs`)**
- Extends folder repository with trash functionality
- Implements soft delete for directories
- Handles recursive trash operations for folders
- Manages restoration of folder hierarchies
- Maintains metadata for trashed folders

**Share FS Repository (`src/infrastructure/repositories/share_fs_repository.rs`)**
- Implements ShareRepository for filesystem-based sharing
- Manages share records and permissions
- Handles link generation and validation
- Provides access control for shared resources
- Supports share expiration and revocation

**Trash FS Repository (`src/infrastructure/repositories/trash_fs_repository.rs`)**
- Implements TrashRepository for filesystem
- Manages trash directory structure
- Handles metadata for trashed items
- Implements cleanup policies for expired trash
- Supports permanent deletion operations

**Session PG Repository (`src/infrastructure/repositories/pg/session_pg_repository.rs`)**
- Implements session storage using PostgreSQL
- Manages user sessions and authentication state
- Handles session creation, validation, and expiration
- Provides secure token management
- Supports multiple concurrent sessions

**User PG Repository (`src/infrastructure/repositories/pg/user_pg_repository.rs`)**
- Implements UserRepository with PostgreSQL
- Stores user accounts and profile information
- Handles user queries and updates
- Manages user roles and permissions
- Supports user search and filtering

**File Metadata Manager (`src/infrastructure/repositories/file_metadata_manager.rs`)**
- Manages file metadata independently of content
- Handles extended attributes for files
- Provides caching for frequently accessed metadata
- Optimizes metadata operations
- Supports custom metadata fields

**File Path Resolver (`src/infrastructure/repositories/file_path_resolver.rs`)**
- Resolves logical paths to physical storage locations
- Handles path normalization and validation
- Provides path translation between different systems
- Supports virtual paths and redirections
- Optimizes path resolution for nested structures

**Parallel File Processor (`src/infrastructure/repositories/parallel_file_processor.rs`)**
- Implements parallel processing for large files
- Optimizes file operations with multi-threading
- Provides chunked reading and writing
- Handles load balancing for file operations
- Implements backpressure mechanisms

### Services

**ID Mapping Service (`src/infrastructure/services/id_mapping_service.rs`)**
- Manages mapping between UUIDs and filesystem paths
- Provides persistent ID generation and lookup
- Handles path changes while maintaining stable IDs
- Implements caching for frequently accessed mappings
- Ensures consistency between IDs and paths

**Buffer Pool (`src/infrastructure/services/buffer_pool.rs`)**
- Manages memory buffers for file operations
- Implements pooling for optimal memory usage
- Provides buffer recycling to reduce allocations
- Handles buffer sizing for different operations
- Implements thread-safe buffer management

**Cache Manager (`src/infrastructure/services/cache_manager.rs`)**
- Provides application-wide caching services
- Implements multiple cache levels (memory, disk)
- Handles cache invalidation and consistency
- Manages cache size limits and eviction
- Provides statistics for cache performance

**Compression Service (`src/infrastructure/services/compression_service.rs`)**
- Implements data compression for files and responses
- Supports multiple compression algorithms
- Provides on-the-fly compression for API responses
- Handles selective compression based on file types
- Optimizes compression levels for different content

**File System I18n Service (`src/infrastructure/services/file_system_i18n_service.rs`)**
- Implements I18n service using filesystem storage
- Loads translations from JSON files
- Handles language detection and fallbacks
- Provides translation lookups for UI components
- Supports dynamic language switching

**File Metadata Cache (`src/infrastructure/services/file_metadata_cache.rs`)**
- Provides caching for file metadata
- Optimizes repeated metadata access
- Implements cache invalidation strategies
- Handles concurrent access to metadata
- Supports different cache levels (memory, persistent)

**ID Mapping Optimizer (`src/infrastructure/services/id_mapping_optimizer.rs`)**
- Optimizes ID-to-path mapping operations
- Implements batch processing for mapping updates
- Provides preloading for frequently accessed mappings
- Handles compaction of mapping storage
- Optimizes lookup performance for large mappings

**Zip Service (`src/infrastructure/services/zip_service.rs`)**
- Provides ZIP archive creation and extraction
- Supports on-the-fly compression for downloads
- Handles large directory archiving
- Implements streaming ZIP generation
- Provides progress tracking for large operations

**Trash Cleanup Service (`src/infrastructure/services/trash_cleanup_service.rs`)**
- Manages automatic cleanup of expired trash items
- Implements retention policy enforcement
- Provides scheduled cleanup operations
- Handles graceful cleanup with resource limits
- Supports custom cleanup rules

## Interfaces Layer

This layer handles external communication, including API endpoints and web interfaces.

### API Handlers

**File Handler (`src/interfaces/api/handlers/file_handler.rs`)**
- Handles HTTP requests for file operations
- Processes file uploads with multipart support
- Provides file downloads with optional compression
- Implements CRUD operations for files
- Manages error responses and status codes

**Folder Handler (`src/interfaces/api/handlers/folder_handler.rs`)**
- Handles HTTP requests for folder operations
- Processes folder creation and listing
- Implements CRUD operations for directories
- Provides folder hierarchy navigation
- Manages error responses for folder operations

**Auth Handler (`src/interfaces/api/handlers/auth_handler.rs`)**
- Handles authentication-related API endpoints
- Processes login, logout, and registration
- Manages session tokens and refresh
- Implements password reset functionality
- Provides authentication status information

**Share Handler (`src/interfaces/api/handlers/share_handler.rs`)**
- Handles file and folder sharing endpoints
- Processes share creation and management
- Provides access to shared resources
- Handles permission verification
- Manages share links and passwords

**Trash Handler (`src/interfaces/api/handlers/trash_handler.rs`)**
- Handles trash-related API endpoints
- Processes moving items to trash
- Provides trash listing and filtering
- Handles restoration from trash
- Manages permanent deletion operations

**Search Handler (`src/interfaces/api/handlers/search_handler.rs`)**
- Handles search-related API endpoints
- Processes text search queries
- Provides filtering and sorting options
- Handles pagination for search results
- Manages relevance scoring for results

**Recent Handler (`src/interfaces/api/handlers/recent_handler.rs`)**
- Handles recently accessed files endpoints
- Provides listing and filtering of recent files
- Manages user-specific recent history
- Handles pagination for recent items
- Provides sorting options for recent files

**Favorites Handler (`src/interfaces/api/handlers/favorites_handler.rs`)**
- Handles user favorites endpoints
- Processes adding and removing favorites
- Provides listing and filtering of favorites
- Manages user-specific favorite collections
- Handles sorting and organization of favorites

**I18n Handler (`src/interfaces/api/handlers/i18n_handler.rs`)**
- Handles internationalization endpoints
- Provides language selection and detection
- Serves translation resources
- Manages locale settings
- Handles language preference persistence

**Batch Handler (`src/interfaces/api/handlers/batch_handler.rs`)**
- Handles batch operation endpoints
- Processes multiple operations in a single request
- Provides transaction support for batches
- Handles partial success scenarios
- Manages comprehensive error reporting

**WebDAV Handler (`src/interfaces/api/handlers/webdav_handler.rs`)**
- Implements WebDAV protocol (RFC 4918) endpoints
- Handles WebDAV methods (PROPFIND, PROPPATCH, etc.)
- Provides file system access via HTTP
- Manages WebDAV properties and locks
- Supports third-party WebDAV clients

### API Routes

**Routes (`src/interfaces/api/routes.rs`)**
- Defines API routes and URL structure
- Maps endpoints to appropriate handlers
- Configures middleware for routes
- Handles versioning for API endpoints
- Provides documentation integration

### Middleware

**Auth Middleware (`src/interfaces/middleware/auth.rs`)**
- Handles authentication for API requests
- Verifies tokens and sessions
- Provides user context for handlers
- Manages authentication errors
- Supports different authentication methods

**Cache Middleware (`src/interfaces/middleware/cache.rs`)**
- Implements response caching
- Handles cache headers and validation
- Provides conditional request processing
- Manages cache invalidation
- Optimizes for different content types

**Redirect Middleware (`src/interfaces/middleware/redirect.rs`)**
- Handles HTTP redirects
- Manages URL normalization
- Provides permanent and temporary redirects
- Handles protocol upgrades (HTTP to HTTPS)
- Supports path-based redirections

### Web Interface

**Web Module (`src/interfaces/web/mod.rs`)**
- Coordinates web interface components
- Manages static file serving
- Provides web application integration
- Handles web-specific middleware
- Supports single-page application routing

## Common Layer

This layer provides shared utilities and configurations used across the application.

**Config (`src/common/config.rs`)**
- Manages application configuration
- Loads settings from environment and files
- Provides typed configuration access
- Handles configuration validation
- Supports different environments (dev, prod)

**Errors (`src/common/errors.rs`)**
- Defines error types and handling
- Provides consistent error formatting
- Implements error context and wrapping
- Handles error translation between layers
- Supports error categorization and logging

**DI (`src/common/di.rs`)**
- Implements dependency injection
- Manages service lifecycles
- Provides application state container
- Handles service resolution and registration
- Supports scoped service instances

**DB (`src/common/db.rs`)**
- Manages database connections
- Provides connection pooling
- Handles database migrations
- Implements query helpers
- Supports transaction management

**Cache (`src/common/cache.rs`)**
- Provides generic caching facilities
- Implements different cache strategies
- Handles cache key generation
- Manages cache invalidation
- Supports distributed caching

**Auth Factory (`src/common/auth_factory.rs`)**
- Creates authentication components
- Configures auth providers based on settings
- Provides factory methods for auth services
- Handles auth strategy selection
- Supports multiple authentication methods