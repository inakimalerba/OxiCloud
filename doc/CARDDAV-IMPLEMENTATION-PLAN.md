# CardDAV Implementation Plan

## Introduction

This document outlines the plan for implementing CardDAV support in OxiCloud. CardDAV is an open protocol for synchronizing address books/contacts between different applications and devices.

## Implementation Roadmap

The implementation will follow these steps:

### Phase 1: Core Infrastructure (Week 1)

#### Database Schema
- Create new migration for CardDAV tables:
  - `address_books` - For storing address book collections
  - `contacts` - For storing contact information
  - `address_book_shares` - For sharing address books between users
  - `contact_groups` - For organizing contacts into groups
  - `group_memberships` - For associating contacts with groups

#### Domain Layer
- Define entity models:
  - `Contact` - Core contact entity
  - `AddressBook` - Collection entity
  - `ContactGroup` - For grouping contacts
- Create repository interfaces:
  - `ContactRepository` - For contact CRUD operations
  - `AddressBookRepository` - For address book management
  - `ContactGroupRepository` - For group management

#### Testing
- Unit tests for entity models
- Repository interface contract tests

### Phase 2: Infrastructure Layer (Week 2)

#### Repository Implementations
- Implement PostgreSQL repositories:
  - `ContactPgRepository`
  - `AddressBookPgRepository`
  - `ContactGroupPgRepository`
- Implement vCard parsing and generation utilities
- Create data migration tools (if needed)

#### Integration
- Update dependency injection system to include new repositories
- Connect with existing auth system

#### Testing
- Repository implementation tests
- vCard parsing/generation tests
- Integration tests with database

### Phase 3: Application Layer (Week 3)

#### Services
- Implement business logic services:
  - `ContactService` - Contact management
  - `AddressBookService` - Address book management
  - `ContactGroupService` - Group management

#### DTOs and Ports
- Create DTOs for contact operations
- Define service interface ports
- Implement request/response mapping

#### CardDAV Adapter
- Create adapter for CardDAV protocol translation
- Implement vCard conversion logic
- Create XML parsing and generation utilities

#### Testing
- Service unit tests
- Integration tests for adapter

### Phase 4: Interface Layer (Week 4)

#### REST API
- Create REST endpoints for address book operations
- Implement contact management endpoints
- Add contact group endpoints
- Document API with OpenAPI

#### CardDAV Protocol Endpoints
- Implement WebDAV method handlers:
  - PROPFIND - For discovery and property retrieval
  - REPORT - For querying contacts
  - MKCOL - For creating address books
  - GET/PUT/DELETE - For contact operations
- Add CardDAV-specific XML handling

#### Integration
- Connect all layers
- Perform end-to-end testing
- Test with various CardDAV clients

#### Testing
- API endpoint tests
- CardDAV protocol compliance tests
- Client compatibility tests

### Phase 5: Refinement and Optimization (Week 5)

#### Performance Optimization
- Add caching for frequently accessed resources
- Optimize database queries
- Implement efficient synchronization mechanisms

#### Security Hardening
- Review authentication and authorization
- Validate input and output
- Add rate limiting

#### Final Testing
- Stress testing with large address books
- Security testing
- User acceptance testing

#### Documentation
- Update API documentation
- Create user guides
- Document client setup procedures

## Technical Specifications

### Database Schema

```sql
-- Address books table
CREATE TABLE IF NOT EXISTS carddav.address_books (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    owner_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    description TEXT,
    color VARCHAR(50),
    is_public BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(owner_id, name)
);

-- Address book sharing
CREATE TABLE IF NOT EXISTS carddav.address_book_shares (
    address_book_id UUID NOT NULL REFERENCES carddav.address_books(id) ON DELETE CASCADE,
    user_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    can_write BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY(address_book_id, user_id)
);

-- Contacts table
CREATE TABLE IF NOT EXISTS carddav.contacts (
    id UUID PRIMARY KEY,
    address_book_id UUID NOT NULL REFERENCES carddav.address_books(id) ON DELETE CASCADE,
    uid VARCHAR(255) NOT NULL,
    full_name VARCHAR(255),
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    nickname VARCHAR(255),
    email JSONB,
    phone JSONB,
    address JSONB,
    organization VARCHAR(255),
    title VARCHAR(255),
    notes TEXT,
    photo_url TEXT,
    birthday DATE,
    anniversary DATE,
    vcard TEXT NOT NULL,
    etag VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(address_book_id, uid)
);

-- Contact groups
CREATE TABLE IF NOT EXISTS carddav.contact_groups (
    id UUID PRIMARY KEY,
    address_book_id UUID NOT NULL REFERENCES carddav.address_books(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Group memberships
CREATE TABLE IF NOT EXISTS carddav.group_memberships (
    group_id UUID NOT NULL REFERENCES carddav.contact_groups(id) ON DELETE CASCADE,
    contact_id UUID NOT NULL REFERENCES carddav.contacts(id) ON DELETE CASCADE,
    PRIMARY KEY(group_id, contact_id)
);
```

### API Endpoints

#### REST API

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/address-books` | List all address books |
| POST | `/api/address-books` | Create a new address book |
| GET | `/api/address-books/:id` | Get a specific address book |
| PUT | `/api/address-books/:id` | Update an address book |
| DELETE | `/api/address-books/:id` | Delete an address book |
| GET | `/api/address-books/:id/contacts` | List contacts in an address book |
| POST | `/api/address-books/:id/contacts` | Create a new contact |
| GET | `/api/address-books/:id/contacts/:contactId` | Get a specific contact |
| PUT | `/api/address-books/:id/contacts/:contactId` | Update a contact |
| DELETE | `/api/address-books/:id/contacts/:contactId` | Delete a contact |

#### CardDAV Protocol Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| PROPFIND | `/carddav/` | List all address books |
| PROPFIND | `/carddav/:addressBookId/` | Get address book information |
| REPORT | `/carddav/:addressBookId/` | Query contacts in an address book |
| GET | `/carddav/:addressBookId/:contactId.vcf` | Get a specific contact (vCard) |
| PUT | `/carddav/:addressBookId/:contactId.vcf` | Create or update a contact |
| DELETE | `/carddav/:addressBookId/:contactId.vcf` | Delete a contact |

### Dependencies

- vCard parsing/generation library (e.g., `vcard-rs` or similar)
- XML processing (for CardDAV protocol)
- Database access (PostgreSQL)
- WebDAV base functionality

## Resources Required

- Developer time: 1 full-time developer for 5 weeks
- Testing resources: Multiple CardDAV clients (Apple Contacts, Thunderbird, Android)
- Server resources: Test environment with PostgreSQL

## Success Criteria

The implementation will be considered successful when:

1. Users can create, update, and delete address books
2. Contacts can be managed within address books
3. Address books can be shared between users
4. Standard CardDAV clients can synchronize with the server
5. Performance is acceptable with large address books (1000+ contacts)
6. Security measures are properly implemented

## Client Setup Guides

After implementation, we will create setup guides for:

- Apple Contacts (macOS/iOS)
- Thunderbird/Evolution
- Android (using DAVx⁵)
- Other common CardDAV clients

## Future Enhancements

After the initial implementation, we may consider:

1. Advanced contact search capabilities
2. Contact merging for duplicate detection
3. Bulk import/export options
4. Contact photo management
5. Extended fields for specialized contact information
6. Integration with other systems (e.g., LDAP directories)