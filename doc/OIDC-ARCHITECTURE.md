# Arquitectura de Integración OIDC en OxiCloud

Este documento describe la arquitectura y el flujo de autenticación OpenID Connect (OIDC) en OxiCloud.

## Diagrama de Arquitectura

```
┌─────────────────────────────────────────────────────────────────────────┐
│                                                                         │
│                          PROVEEDOR DE IDENTIDAD                         │
│                                                                         │
│    ┌───────────────┐      ┌───────────────┐      ┌───────────────┐     │
│    │               │      │               │      │               │     │
│    │   Authentik   │      │   Authelia    │      │   KeyCloak    │     │
│    │               │      │               │      │               │     │
│    └───────┬───────┘      └───────┬───────┘      └───────┬───────┘     │
│            │                      │                      │             │
└────────────┼──────────────────────┼──────────────────────┼─────────────┘
             │                      │                      │              
             │                      │                      │              
             │                      │                      │              
             │                     OIDC                    │              
             │                      │                      │              
             │                      │                      │              
┌────────────┼──────────────────────┼──────────────────────┼─────────────┐
│            │                      │                      │             │
│            ▼                      ▼                      ▼             │
│    ┌───────────────────────────────────────────────────────────────┐   │
│    │                                                               │   │
│    │                          OXICLOUD                             │   │
│    │                                                               │   │
│    │   ┌───────────────┐      ┌───────────────┐                    │   │
│    │   │               │      │               │                    │   │
│    │   │ OidcService   │◄────►│ AuthService   │                    │   │
│    │   │               │      │               │                    │   │
│    │   └───────┬───────┘      └───────┬───────┘                    │   │
│    │           │                      │                            │   │
│    │           ▼                      ▼                            │   │
│    │   ┌───────────────────────────────────────────┐               │   │
│    │   │                                           │               │   │
│    │   │       AuthApplicationService              │               │   │
│    │   │                                           │               │   │
│    │   └───────────────────┬───────────────────────┘               │   │
│    │                       │                                        │   │
│    │                       ▼                                        │   │
│    │   ┌───────────────────────────────────────────┐               │   │
│    │   │                                           │               │   │
│    │   │             Auth Handler                  │               │   │
│    │   │                                           │               │   │
│    │   └───────────────────────────────────────────┘               │   │
│    │                                                               │   │
│    └───────────────────────────────────────────────────────────────┘   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
                           ▲
                           │
                           │ HTTP/HTTPS
                           │
                           │
┌────────────────────────────────────────────────────────────────────────┐
│                                                                        │
│                           NAVEGADOR WEB                                │
│                                                                        │
│    ┌───────────────────────────────────────────────────────────────┐   │
│    │                                                               │   │
│    │                    Interfaz de Usuario                        │   │
│    │                                                               │   │
│    │    ┌──────────────┐        ┌──────────────┐                   │   │
│    │    │              │        │              │                   │   │
│    │    │ Login.html   │        │ oidcAuth.js  │                   │   │
│    │    │              │        │              │                   │   │
│    │    └──────────────┘        └──────────────┘                   │   │
│    │                                                               │   │
│    └───────────────────────────────────────────────────────────────┘   │
│                                                                        │
└────────────────────────────────────────────────────────────────────────┘
```

## Flujo de Autenticación OIDC

El flujo de autenticación OIDC en OxiCloud sigue el flujo de código de autorización (Authorization Code Flow):

1. **Inicio de la Autenticación**:
   - El usuario hace clic en "Login con [Proveedor]" en la página de inicio de sesión.
   - El frontend genera un estado aleatorio para protección CSRF.
   - El frontend solicita a OxiCloud una URL de autorización.

2. **Redirección al Proveedor de Identidad**:
   - OxiCloud genera una URL de autorización y la devuelve al frontend.
   - El navegador redirige al usuario a la página de inicio de sesión del proveedor de identidad.

3. **Autenticación en el Proveedor**:
   - El usuario se autentica en el proveedor de identidad (con contraseña, 2FA, etc.).
   - El proveedor redirige al usuario de vuelta a OxiCloud con un código de autorización.

4. **Intercambio del Código de Autorización**:
   - El frontend de OxiCloud recibe el código de autorización y lo envía al backend.
   - OxiCloud intercambia el código por tokens de acceso e ID con el proveedor de identidad.
   - OxiCloud verifica el token de ID y extrae la información del usuario.

5. **Creación/Recuperación de Usuario**:
   - OxiCloud busca un usuario existente con el ID externo del proveedor.
   - Si no existe y la creación automática está habilitada, se crea un nuevo usuario.
   - Si no existe y la creación automática está deshabilitada, se devuelve un error.

6. **Generación de Tokens de Sesión**:
   - OxiCloud genera sus propios tokens de acceso y actualización para el usuario.
   - Estos tokens se utilizan para autenticar las solicitudes subsiguientes a la API de OxiCloud.

7. **Respuesta al Cliente**:
   - OxiCloud devuelve los tokens y la información del usuario al frontend.
   - El frontend almacena los tokens y redirige al usuario a la página principal.

## Componentes Principales

### 1. OidcService

Este servicio gestiona la comunicación con los proveedores OIDC:
- Descubre los endpoints OIDC de los proveedores
- Genera URLs de autorización
- Intercambia códigos de autorización por tokens
- Verifica tokens y extrae información de usuario

### 2. AuthApplicationService

Coordina el proceso de autenticación:
- Proporciona una interfaz entre la capa de API y los servicios de dominio
- Gestiona el proceso de creación/recuperación de usuarios
- Coordina la generación de tokens de acceso para OxiCloud

### 3. Auth Handler

Expone endpoints HTTP para el flujo de autenticación OIDC:
- `/api/auth/oidc/providers` - Lista los proveedores OIDC disponibles
- `/api/auth/oidc/auth` - Genera una URL de autorización para un proveedor
- `/api/auth/oidc/callback` - Procesa la respuesta del proveedor y completa la autenticación

### 4. Frontend (oidcAuth.js)

Gestiona la parte del cliente del flujo de autenticación:
- Muestra botones para los proveedores OIDC
- Inicia el flujo de autenticación
- Maneja la redirección de retorno del proveedor
- Procesa y almacena los tokens de sesión

## Configuración Multi-Proveedor

OxiCloud permite configurar múltiples proveedores OIDC simultáneamente:

1. **Configuración Separada**: Cada proveedor tiene su propia configuración independiente.
2. **Selección de Proveedor**: Los usuarios pueden elegir con qué proveedor autenticarse.
3. **Mapeo de Identidades**: OxiCloud mapea identidades de diferentes proveedores a usuarios internos.

## Seguridad

La implementación OIDC en OxiCloud incluye varias medidas de seguridad:

1. **Protección CSRF**: Utiliza un estado aleatorio para prevenir ataques CSRF.
2. **Validación de Tokens**: Verifica firmas y vigencia de los tokens JWT.
3. **Código de Autorización**: Utiliza el flujo de código de autorización, que es más seguro que el flujo implícito.
4. **HTTPS**: Requiere conexiones HTTPS para todas las comunicaciones OIDC.
5. **Secretos del Cliente**: Los secretos del cliente se almacenan de forma segura y nunca se exponen al frontend.