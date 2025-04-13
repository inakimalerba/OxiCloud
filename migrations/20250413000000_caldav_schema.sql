-- OxiCloud CalDAV Schema Migration
-- Migration 003: CalDAV Schema

-- Create schema for CalDAV-related tables
CREATE SCHEMA IF NOT EXISTS caldav;

-- Calendar table
CREATE TABLE IF NOT EXISTS caldav.calendars (
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

-- Calendar properties for custom properties (for extended CalDAV support)
CREATE TABLE IF NOT EXISTS caldav.calendar_properties (
    id SERIAL PRIMARY KEY,
    calendar_id UUID NOT NULL REFERENCES caldav.calendars(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    value TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(calendar_id, name)
);

-- Calendar events table
CREATE TABLE IF NOT EXISTS caldav.calendar_events (
    id UUID PRIMARY KEY,
    calendar_id UUID NOT NULL REFERENCES caldav.calendars(id) ON DELETE CASCADE,
    summary VARCHAR(255) NOT NULL,
    description TEXT,
    location TEXT,
    start_time TIMESTAMP WITH TIME ZONE NOT NULL,
    end_time TIMESTAMP WITH TIME ZONE NOT NULL,
    all_day BOOLEAN NOT NULL DEFAULT FALSE,
    rrule TEXT, -- Recurrence rule
    ical_uid VARCHAR(255) NOT NULL, -- UID from iCalendar format
    ical_data TEXT NOT NULL, -- Complete iCalendar data
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(calendar_id, ical_uid)
);

-- Calendar sharing table
CREATE TABLE IF NOT EXISTS caldav.calendar_shares (
    id SERIAL PRIMARY KEY,
    calendar_id UUID NOT NULL REFERENCES caldav.calendars(id) ON DELETE CASCADE,
    user_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    access_level VARCHAR(50) NOT NULL, -- 'read', 'write', 'owner'
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(calendar_id, user_id)
);

-- Create indexes for efficient querying
CREATE INDEX IF NOT EXISTS idx_calendar_owner ON caldav.calendars(owner_id);
CREATE INDEX IF NOT EXISTS idx_calendar_public ON caldav.calendars(is_public);
CREATE INDEX IF NOT EXISTS idx_calendar_properties_calendar ON caldav.calendar_properties(calendar_id);
CREATE INDEX IF NOT EXISTS idx_calendar_event_calendar ON caldav.calendar_events(calendar_id);
CREATE INDEX IF NOT EXISTS idx_calendar_event_time_range ON caldav.calendar_events(start_time, end_time);
CREATE INDEX IF NOT EXISTS idx_calendar_event_uid ON caldav.calendar_events(ical_uid);
CREATE INDEX IF NOT EXISTS idx_calendar_shares_calendar ON caldav.calendar_shares(calendar_id);
CREATE INDEX IF NOT EXISTS idx_calendar_shares_user ON caldav.calendar_shares(user_id);

COMMENT ON TABLE caldav.calendars IS 'Stores calendar information for CalDAV support';
COMMENT ON TABLE caldav.calendar_properties IS 'Stores custom properties for calendars';
COMMENT ON TABLE caldav.calendar_events IS 'Stores calendar events with iCalendar data';
COMMENT ON TABLE caldav.calendar_shares IS 'Tracks calendar sharing between users';