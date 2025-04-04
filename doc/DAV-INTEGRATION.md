# Integración de WebDAV, CalDAV y CardDAV en OxiCloud

Este documento describe el diseño e implementación de los protocolos WebDAV, CalDAV y CardDAV en OxiCloud, extendiendo la plataforma para soportar clientes y dispositivos que utilizan estos estándares.

## Tabla de Contenidos

1. [Introducción](#introducción)
2. [Arquitectura de la Implementación](#arquitectura-de-la-implementación)
3. [WebDAV](#webdav)
4. [CalDAV](#caldav)
5. [CardDAV](#carddav)
6. [Consideraciones de Seguridad](#consideraciones-de-seguridad)
7. [Pruebas y Compatibilidad](#pruebas-y-compatibilidad)

## Introducción

### WebDAV (Web Distributed Authoring and Versioning)
WebDAV es una extensión del protocolo HTTP que permite a los clientes realizar operaciones sobre archivos en un servidor remoto, como crear, modificar, mover y eliminar archivos y directorios.

### CalDAV (Calendaring Extensions to WebDAV)
CalDAV es un protocolo basado en WebDAV que permite a los clientes acceder y gestionar datos de calendario, como eventos y tareas.

### CardDAV (vCard Extensions to WebDAV)
CardDAV es un protocolo que extiende WebDAV para permitir el acceso y gestión de datos de contactos en formato vCard.

## Arquitectura de la Implementación

La implementación de los protocolos DAV se integra en la arquitectura hexagonal existente de OxiCloud:

```
┌────────────────────────────────────────────────────────────────────┐
│                          INTERFACES                                │
│                                                                    │
│  ┌───────────────┐  ┌───────────────┐  ┌───────────────────────┐  │
│  │               │  │               │  │                       │  │
│  │  REST API     │  │  WebDAV API   │  │  CalDAV/CardDAV API   │  │
│  │               │  │               │  │                       │  │
│  └───────┬───────┘  └───────┬───────┘  └───────────┬───────────┘  │
│          │                  │                      │              │
└──────────┼──────────────────┼──────────────────────┼──────────────┘
           │                  │                      │               
           ▼                  ▼                      ▼               
┌──────────────────────────────────────────────────────────────────┐
│                          APLICACIÓN                              │
│                                                                  │
│  ┌───────────┐  ┌────────────┐  ┌───────────┐  ┌──────────────┐ │
│  │           │  │            │  │           │  │              │ │
│  │FileService│  │FolderService│  │CalService │  │ContactService│ │
│  │           │  │            │  │           │  │              │ │
│  └─────┬─────┘  └──────┬─────┘  └─────┬─────┘  └──────┬───────┘ │
│        │               │              │               │         │
└────────┼───────────────┼──────────────┼───────────────┼─────────┘
         │               │              │               │          
         ▼               ▼              ▼               ▼          
┌────────────────────────────────────────────────────────────────┐
│                          DOMINIO                               │
│                                                                │
│  ┌─────────┐  ┌──────────┐  ┌────────────┐  ┌───────────────┐ │
│  │         │  │          │  │            │  │               │ │
│  │  File   │  │  Folder  │  │  Calendar  │  │    Contact    │ │
│  │         │  │          │  │            │  │               │ │
│  └─────────┘  └──────────┘  └────────────┘  └───────────────┘ │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

### Componentes Principales

1. **Adaptadores DAV**: Convertirán entre las especificaciones DAV y los modelos de OxiCloud
2. **Servicios de Aplicación**: Se extenderán para incluir funcionalidades específicas DAV
3. **Modelos de Dominio**: Se añadirán nuevas entidades para Calendar y Contact
4. **Repositorios**: Implementaciones de almacenamiento para calendarios y contactos

## WebDAV

### Endpoints Requeridos

| Método HTTP | Endpoint | Descripción |
|-------------|----------|-------------|
| OPTIONS | /webdav/{path} | Indica las capacidades WebDAV soportadas |
| PROPFIND | /webdav/{path} | Recupera propiedades de recursos |
| PROPPATCH | /webdav/{path} | Modifica propiedades de recursos |
| MKCOL | /webdav/{path} | Crea colecciones (directorios) |
| GET | /webdav/{path} | Recupera contenido de recursos |
| HEAD | /webdav/{path} | Recupera metadatos de recursos |
| PUT | /webdav/{path} | Crea o actualiza recursos |
| DELETE | /webdav/{path} | Elimina recursos |
| COPY | /webdav/{path} | Copia recursos |
| MOVE | /webdav/{path} | Mueve recursos |
| LOCK | /webdav/{path} | Bloquea recursos |
| UNLOCK | /webdav/{path} | Desbloquea recursos |

### Implementación

1. **Manejador WebDAV**:

```rust
// src/interfaces/api/handlers/webdav_handler.rs
use std::sync::Arc;
use axum::{
    Router, 
    routing::get,
    extract::{Path, State, Request, Extension},
    http::StatusCode,
    response::Response,
};
use http::{Method, header};

use crate::common::di::AppState;
use crate::interfaces::middleware::auth::CurrentUser;
use crate::application::ports::file_ports::{FileRetrievalUseCase, FileUploadUseCase};
use crate::application::ports::folder_ports::FolderUseCase;
use crate::common::errors::AppError;

pub fn webdav_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/webdav/*path", get(handle_get))
        .route_with_tsr("/webdav/*path", axum::routing::on(
            Method::OPTIONS, handle_options,
            Method::PROPFIND, handle_propfind,
            Method::PROPPATCH, handle_proppatch,
            Method::MKCOL, handle_mkcol,
            Method::PUT, handle_put,
            Method::DELETE, handle_delete,
            Method::COPY, handle_copy,
            Method::MOVE, handle_move,
            Method::LOCK, handle_lock,
            Method::UNLOCK, handle_unlock,
        ))
}

// Implementar funciones para cada método WebDAV...
```

2. **Adaptador WebDAV**:

```rust
// src/application/adapters/webdav_adapter.rs
use xml::reader::{EventReader, XmlEvent};
use xml::writer::{EventWriter, EmitterConfig, XmlEvent as WriteEvent};
use std::io::{Read, Write};
use crate::application::dtos::file_dto::FileDto;
use crate::application::dtos::folder_dto::FolderDto;

/// Convierte entre objetos de OxiCloud y representaciones WebDAV
pub struct WebDavAdapter;

impl WebDavAdapter {
    /// Convierte una propiedad PROPFIND en XML a un objeto de solicitud
    pub fn parse_propfind<R: Read>(reader: R) -> Result<PropFindRequest, Error> {
        // Implementación...
    }
    
    /// Genera respuesta XML para PROPFIND basada en archivos y carpetas
    pub fn generate_propfind_response<W: Write>(
        writer: W,
        files: &[FileDto],
        folders: &[FolderDto],
        base_url: &str,
    ) -> Result<(), Error> {
        // Implementación...
    }
    
    // Otros métodos para manejar diferentes operaciones WebDAV...
}
```

## CalDAV

### Endpoints Requeridos

| Método HTTP | Endpoint | Descripción |
|-------------|----------|-------------|
| PROPFIND | /caldav/{calendar} | Recupera propiedades del calendario |
| REPORT | /caldav/{calendar} | Consulta eventos del calendario |
| MKCALENDAR | /caldav/{calendar} | Crea un nuevo calendario |
| PUT | /caldav/{calendar}/{event}.ics | Crea o actualiza un evento |
| GET | /caldav/{calendar}/{event}.ics | Recupera un evento |
| DELETE | /caldav/{calendar}/{event}.ics | Elimina un evento |

### Implementación

1. **Nuevas Entidades de Dominio**:

```rust
// src/domain/entities/calendar.rs
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Calendar {
    id: Uuid,
    name: String,
    owner_id: String,
    description: Option<String>,
    color: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

// src/domain/entities/calendar_event.rs
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct CalendarEvent {
    id: Uuid,
    calendar_id: Uuid,
    summary: String,
    description: Option<String>,
    location: Option<String>,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    all_day: bool,
    rrule: Option<String>,  // Regla de recurrencia
    ical_data: String,      // Datos iCalendar completos
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
```

2. **Repositorios**:

```rust
// src/domain/repositories/calendar_repository.rs
use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::entities::calendar::Calendar;
use crate::common::errors::Result;

#[async_trait]
pub trait CalendarRepository: Send + Sync {
    async fn create_calendar(&self, calendar: Calendar) -> Result<Calendar>;
    async fn get_calendar_by_id(&self, id: &Uuid) -> Result<Calendar>;
    async fn get_calendars_by_owner(&self, owner_id: &str) -> Result<Vec<Calendar>>;
    async fn update_calendar(&self, calendar: Calendar) -> Result<Calendar>;
    async fn delete_calendar(&self, id: &Uuid) -> Result<()>;
}

// src/domain/repositories/calendar_event_repository.rs
use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::entities::calendar_event::CalendarEvent;
use crate::common::errors::Result;

#[async_trait]
pub trait CalendarEventRepository: Send + Sync {
    async fn create_event(&self, event: CalendarEvent) -> Result<CalendarEvent>;
    async fn get_event_by_id(&self, id: &Uuid) -> Result<CalendarEvent>;
    async fn get_events_by_calendar(&self, calendar_id: &Uuid) -> Result<Vec<CalendarEvent>>;
    async fn get_events_in_timerange(
        &self,
        calendar_id: &Uuid,
        start: &DateTime<Utc>,
        end: &DateTime<Utc>
    ) -> Result<Vec<CalendarEvent>>;
    async fn update_event(&self, event: CalendarEvent) -> Result<CalendarEvent>;
    async fn delete_event(&self, id: &Uuid) -> Result<()>;
}
```

3. **Servicio CalDAV**:

```rust
// src/application/services/caldav_service.rs
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::repositories::calendar_repository::CalendarRepository;
use crate::domain::repositories::calendar_event_repository::CalendarEventRepository;
use crate::domain::entities::calendar::Calendar;
use crate::domain::entities::calendar_event::CalendarEvent;
use crate::application::dtos::calendar_dto::{CalendarDto, CalendarEventDto};
use crate::common::errors::{Result, DomainError};

pub struct CalDavService {
    calendar_repository: Arc<dyn CalendarRepository>,
    event_repository: Arc<dyn CalendarEventRepository>,
}

impl CalDavService {
    pub fn new(
        calendar_repository: Arc<dyn CalendarRepository>,
        event_repository: Arc<dyn CalendarEventRepository>,
    ) -> Self {
        Self {
            calendar_repository,
            event_repository,
        }
    }
    
    // Implementar métodos para operaciones CalDAV...
}
```

4. **Manejador CalDAV**:

```rust
// src/interfaces/api/handlers/caldav_handler.rs
use std::sync::Arc;
use axum::{
    Router, 
    routing::{get, put, delete},
    extract::{Path, State, Request, Extension},
    http::StatusCode,
    response::Response,
};
use http::{Method, header};

use crate::common::di::AppState;
use crate::interfaces::middleware::auth::CurrentUser;
use crate::application::services::caldav_service::CalDavService;
use crate::common::errors::AppError;

pub fn caldav_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/caldav/", get(get_calendars))
        .route("/caldav/:calendar", get(get_calendar))
        .route_with_tsr("/caldav/:calendar", axum::routing::on(
            Method::PROPFIND, handle_calendar_propfind,
            Method::REPORT, handle_calendar_report,
            Method::MKCALENDAR, handle_mkcalendar,
        ))
        .route("/caldav/:calendar/:event", get(get_event))
        .route("/caldav/:calendar/:event", put(put_event))
        .route("/caldav/:calendar/:event", delete(delete_event))
}

// Implementar funciones para cada método CalDAV...
```

## CardDAV

### Endpoints Requeridos

| Método HTTP | Endpoint | Descripción |
|-------------|----------|-------------|
| PROPFIND | /carddav/addressbooks/{addressbook} | Recupera propiedades de la libreta de direcciones |
| REPORT | /carddav/addressbooks/{addressbook} | Consulta contactos |
| MKCOL | /carddav/addressbooks/{addressbook} | Crea una nueva libreta de direcciones |
| PUT | /carddav/addressbooks/{addressbook}/{contact}.vcf | Crea o actualiza un contacto |
| GET | /carddav/addressbooks/{addressbook}/{contact}.vcf | Recupera un contacto |
| DELETE | /carddav/addressbooks/{addressbook}/{contact}.vcf | Elimina un contacto |

### Implementación

1. **Nuevas Entidades de Dominio**:

```rust
// src/domain/entities/address_book.rs
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct AddressBook {
    id: Uuid,
    name: String,
    owner_id: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

// src/domain/entities/contact.rs
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Contact {
    id: Uuid,
    address_book_id: Uuid,
    full_name: String,
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    address: Option<String>,
    organization: Option<String>,
    vcard_data: String,      // Datos vCard completos
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
```

2. **Repositorios**:

```rust
// src/domain/repositories/address_book_repository.rs
use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::entities::address_book::AddressBook;
use crate::common::errors::Result;

#[async_trait]
pub trait AddressBookRepository: Send + Sync {
    async fn create_address_book(&self, address_book: AddressBook) -> Result<AddressBook>;
    async fn get_address_book_by_id(&self, id: &Uuid) -> Result<AddressBook>;
    async fn get_address_books_by_owner(&self, owner_id: &str) -> Result<Vec<AddressBook>>;
    async fn update_address_book(&self, address_book: AddressBook) -> Result<AddressBook>;
    async fn delete_address_book(&self, id: &Uuid) -> Result<()>;
}

// src/domain/repositories/contact_repository.rs
use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::entities::contact::Contact;
use crate::common::errors::Result;

#[async_trait]
pub trait ContactRepository: Send + Sync {
    async fn create_contact(&self, contact: Contact) -> Result<Contact>;
    async fn get_contact_by_id(&self, id: &Uuid) -> Result<Contact>;
    async fn get_contacts_by_address_book(&self, address_book_id: &Uuid) -> Result<Vec<Contact>>;
    async fn search_contacts(&self, address_book_id: &Uuid, query: &str) -> Result<Vec<Contact>>;
    async fn update_contact(&self, contact: Contact) -> Result<Contact>;
    async fn delete_contact(&self, id: &Uuid) -> Result<()>;
}
```

3. **Servicio CardDAV**:

```rust
// src/application/services/carddav_service.rs
use std::sync::Arc;
use uuid::Uuid;
use crate::domain::repositories::address_book_repository::AddressBookRepository;
use crate::domain::repositories::contact_repository::ContactRepository;
use crate::domain::entities::address_book::AddressBook;
use crate::domain::entities::contact::Contact;
use crate::application::dtos::address_book_dto::{AddressBookDto, ContactDto};
use crate::common::errors::{Result, DomainError};

pub struct CardDavService {
    address_book_repository: Arc<dyn AddressBookRepository>,
    contact_repository: Arc<dyn ContactRepository>,
}

impl CardDavService {
    pub fn new(
        address_book_repository: Arc<dyn AddressBookRepository>,
        contact_repository: Arc<dyn ContactRepository>,
    ) -> Self {
        Self {
            address_book_repository,
            contact_repository,
        }
    }
    
    // Implementar métodos para operaciones CardDAV...
}
```

4. **Manejador CardDAV**:

```rust
// src/interfaces/api/handlers/carddav_handler.rs
use std::sync::Arc;
use axum::{
    Router, 
    routing::{get, put, delete},
    extract::{Path, State, Request, Extension},
    http::StatusCode,
    response::Response,
};
use http::{Method, header};

use crate::common::di::AppState;
use crate::interfaces::middleware::auth::CurrentUser;
use crate::application::services::carddav_service::CardDavService;
use crate::common::errors::AppError;

pub fn carddav_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/carddav/addressbooks/", get(get_address_books))
        .route("/carddav/addressbooks/:addressbook", get(get_address_book))
        .route_with_tsr("/carddav/addressbooks/:addressbook", axum::routing::on(
            Method::PROPFIND, handle_addressbook_propfind,
            Method::REPORT, handle_addressbook_report,
            Method::MKCOL, handle_mkaddressbook,
        ))
        .route("/carddav/addressbooks/:addressbook/:contact", get(get_contact))
        .route("/carddav/addressbooks/:addressbook/:contact", put(put_contact))
        .route("/carddav/addressbooks/:addressbook/:contact", delete(delete_contact))
}

// Implementar funciones para cada método CardDAV...
```

## Esquema de Base de Datos

```sql
-- Esquema para CalDAV
CREATE TABLE calendar (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    owner_id VARCHAR(255) NOT NULL,
    description TEXT,
    color VARCHAR(50),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE calendar_event (
    id UUID PRIMARY KEY,
    calendar_id UUID NOT NULL REFERENCES calendar(id) ON DELETE CASCADE,
    summary VARCHAR(255) NOT NULL,
    description TEXT,
    location TEXT,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    all_day BOOLEAN NOT NULL DEFAULT FALSE,
    rrule TEXT,
    ical_uid VARCHAR(255) NOT NULL,
    ical_data TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Esquema para CardDAV
CREATE TABLE address_book (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    owner_id VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE contact (
    id UUID PRIMARY KEY,
    address_book_id UUID NOT NULL REFERENCES address_book(id) ON DELETE CASCADE,
    full_name VARCHAR(255) NOT NULL,
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    email VARCHAR(255),
    phone VARCHAR(100),
    address TEXT,
    organization VARCHAR(255),
    vcard_uid VARCHAR(255) NOT NULL,
    vcard_data TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Índices para búsqueda eficiente
CREATE INDEX idx_calendar_owner ON calendar(owner_id);
CREATE INDEX idx_calendar_event_calendar ON calendar_event(calendar_id);
CREATE INDEX idx_address_book_owner ON address_book(owner_id);
CREATE INDEX idx_contact_address_book ON contact(address_book_id);
CREATE INDEX idx_contact_name ON contact(full_name);
```

## Consideraciones de Seguridad

1. **Autenticación**
   - Utilizar la autenticación existente de OxiCloud
   - Soportar autenticación HTTP Basic para clientes DAV
   - Implementar el esquema de autenticación Digest si es necesario

2. **Autorización**
   - Verificar permisos de usuario para acceder a recursos
   - Implementar control de acceso basado en propietario y permisos compartidos
   - Asegurar que los usuarios solo puedan acceder a sus propios calendarios y libretas de direcciones

3. **Prevención de Ataques**
   - Validar y sanitizar todas las entradas XML
   - Limitar tamaño máximo de carga útil
   - Implementar rate limiting en endpoints DAV

## Pruebas y Compatibilidad

### Clientes a Probar

1. **WebDAV**
   - Windows Explorer
   - macOS Finder
   - Cyberduck
   - FileZilla (con extensión WebDAV)

2. **CalDAV**
   - Apple Calendar
   - Mozilla Thunderbird (Lightning)
   - Microsoft Outlook (con complemento CalDAV)
   - Google Calendar (mediante sincronización)

3. **CardDAV**
   - Apple Contacts
   - Mozilla Thunderbird
   - Microsoft Outlook (con complemento CardDAV)
   - Google Contacts (mediante sincronización)

### Pruebas de Cumplimiento

- Utilizar la suite de pruebas CalDAVTester para verificar la conformidad con el estándar
- Validar cumplimiento de RFC para cada protocolo
- Pruebas de stress para evaluar rendimiento bajo carga

### Depuración

- Implementar logging detallado para operaciones DAV
- Crear herramientas de diagnóstico para depurar solicitudes DAV complejas
- Proporcionar mensajes de error claros para ayudar en la resolución de problemas