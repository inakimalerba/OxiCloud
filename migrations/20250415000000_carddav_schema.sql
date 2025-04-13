-- Create the carddav schema if it doesn't exist
CREATE SCHEMA IF NOT EXISTS carddav;

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

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_contacts_address_book_id ON carddav.contacts(address_book_id);
CREATE INDEX IF NOT EXISTS idx_contacts_uid ON carddav.contacts(uid);
CREATE INDEX IF NOT EXISTS idx_contacts_updated_at ON carddav.contacts(updated_at);
CREATE INDEX IF NOT EXISTS idx_address_books_owner_id ON carddav.address_books(owner_id);
CREATE INDEX IF NOT EXISTS idx_group_memberships_group_id ON carddav.group_memberships(group_id);
CREATE INDEX IF NOT EXISTS idx_group_memberships_contact_id ON carddav.group_memberships(contact_id);