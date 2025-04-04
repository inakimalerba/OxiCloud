# Ejemplos de Configuración de OIDC para OxiCloud

Esta guía proporciona ejemplos de configuración para integrar OxiCloud con diferentes proveedores OIDC (OpenID Connect).

## Índice

1. [Configuración General de OIDC](#configuración-general-de-oidc)
2. [Authentik](#authentik)
3. [Authelia](#authelia)
4. [KeyCloak](#keycloak)
5. [Resolución de Problemas](#resolución-de-problemas)

## Configuración General de OIDC

Para habilitar la integración OIDC en OxiCloud, necesitará establecer las siguientes variables de entorno:

```bash
# Habilitar OIDC
OXICLOUD_ENABLE_OIDC=true

# Configuración para cada proveedor OIDC (puede configurar múltiples proveedores)
OXICLOUD_OIDC_PROVIDER_<NOMBRE>_NAME="Nombre Visible"
OXICLOUD_OIDC_PROVIDER_<NOMBRE>_CLIENT_ID="su-client-id"
OXICLOUD_OIDC_PROVIDER_<NOMBRE>_CLIENT_SECRET="su-client-secret"
OXICLOUD_OIDC_PROVIDER_<NOMBRE>_DISCOVERY_URL="https://proveedor.example.com/.well-known/openid-configuration"
OXICLOUD_OIDC_PROVIDER_<NOMBRE>_REDIRECT_URI="https://su-oxicloud.example.com/oidc/callback/<nombre>"
OXICLOUD_OIDC_PROVIDER_<NOMBRE>_SCOPES="openid profile email"
OXICLOUD_OIDC_PROVIDER_<NOMBRE>_USER_ID_ATTRIBUTE="sub"
OXICLOUD_OIDC_PROVIDER_<NOMBRE>_DEFAULT_ROLE="user"
OXICLOUD_OIDC_PROVIDER_<NOMBRE>_AUTO_CREATE_USERS="true"
```

## Authentik

[Authentik](https://goauthentik.io/) es una plataforma de identidad de código abierto que proporciona autenticación, autorización y gestión de usuarios.

### 1. Configurar una aplicación en Authentik

1. Inicia sesión en tu panel de administración de Authentik
2. Ve a "Applications" → "Create"
3. Introduce un nombre para tu aplicación (ej. "OxiCloud")
4. Selecciona "OAuth2/OpenID Provider" como tipo de proveedor
5. En la configuración de OAuth2:
   - **Redirect URI/Callback URL**: `https://su-oxicloud.example.com/oidc/callback/authentik`
   - **Client Type**: Confidential
   - **Client ID**: Se generará automáticamente (anótalo)
   - **Client Secret**: Se generará automáticamente (anótalo)
   - **Scopes**: openid, email, profile
6. En la configuración de UI:
   - **Launch URL**: `https://su-oxicloud.example.com/`
   - **Icon**: Opcional, puedes subir un icono para OxiCloud

### 2. Configurar OxiCloud para Authentik

```yaml
# docker-compose.yml
version: '3'
services:
  oxicloud:
    image: oxicloud:latest
    environment:
      # Configuración general
      OXICLOUD_ENABLE_OIDC: "true"
      
      # Configuración de Authentik
      OXICLOUD_OIDC_PROVIDER_AUTHENTIK_NAME: "Authentik"
      OXICLOUD_OIDC_PROVIDER_AUTHENTIK_CLIENT_ID: "tu-client-id-de-authentik"
      OXICLOUD_OIDC_PROVIDER_AUTHENTIK_CLIENT_SECRET: "tu-client-secret-de-authentik"
      OXICLOUD_OIDC_PROVIDER_AUTHENTIK_DISCOVERY_URL: "https://authentik.example.com/application/o/oxicloud/.well-known/openid-configuration"
      OXICLOUD_OIDC_PROVIDER_AUTHENTIK_REDIRECT_URI: "https://oxicloud.example.com/oidc/callback/authentik"
      OXICLOUD_OIDC_PROVIDER_AUTHENTIK_SCOPES: "openid profile email"
      OXICLOUD_OIDC_PROVIDER_AUTHENTIK_USER_ID_ATTRIBUTE: "sub"
      OXICLOUD_OIDC_PROVIDER_AUTHENTIK_DEFAULT_ROLE: "user"
      OXICLOUD_OIDC_PROVIDER_AUTHENTIK_AUTO_CREATE_USERS: "true"
    ports:
      - "8085:8085"
    volumes:
      - ./storage:/app/storage
```

## Authelia

[Authelia](https://www.authelia.com/) es una solución de autenticación multi-factor de código abierto.

### 1. Configurar Authelia para OxiCloud

Edita tu configuración de Authelia (`configuration.yml`):

```yaml
identity_providers:
  oidc:
    hmac_secret: tu-secreto-seguro  # Cambia esto por un valor aleatorio seguro
    issuer_private_key: /config/private.pem  # Ruta a tu clave privada
    cors:
      endpoints: ['authorization', 'token', 'revocation', 'introspection']
      allowed_origins:
        - https://oxicloud.example.com
    clients:
      - id: oxicloud
        description: OxiCloud
        secret: tu-client-secret-seguro  # Cambia esto
        public: false
        authorization_policy: two_factor
        redirect_uris:
          - https://oxicloud.example.com/oidc/callback/authelia
        scopes: ['openid', 'profile', 'email', 'groups']
        userinfo_signing_algorithm: none
```

### 2. Configurar OxiCloud para Authelia

```yaml
# docker-compose.yml
version: '3'
services:
  oxicloud:
    image: oxicloud:latest
    environment:
      # Configuración general
      OXICLOUD_ENABLE_OIDC: "true"
      
      # Configuración de Authelia
      OXICLOUD_OIDC_PROVIDER_AUTHELIA_NAME: "Authelia"
      OXICLOUD_OIDC_PROVIDER_AUTHELIA_CLIENT_ID: "oxicloud"
      OXICLOUD_OIDC_PROVIDER_AUTHELIA_CLIENT_SECRET: "tu-client-secret-seguro"
      OXICLOUD_OIDC_PROVIDER_AUTHELIA_DISCOVERY_URL: "https://authelia.example.com/.well-known/openid-configuration"
      OXICLOUD_OIDC_PROVIDER_AUTHELIA_REDIRECT_URI: "https://oxicloud.example.com/oidc/callback/authelia"
      OXICLOUD_OIDC_PROVIDER_AUTHELIA_SCOPES: "openid profile email groups"
      OXICLOUD_OIDC_PROVIDER_AUTHELIA_USER_ID_ATTRIBUTE: "sub"
      OXICLOUD_OIDC_PROVIDER_AUTHELIA_DEFAULT_ROLE: "user"
      OXICLOUD_OIDC_PROVIDER_AUTHELIA_AUTO_CREATE_USERS: "true"
    ports:
      - "8085:8085"
    volumes:
      - ./storage:/app/storage
```

## KeyCloak

[KeyCloak](https://www.keycloak.org/) es una solución de gestión de identidad y acceso de código abierto.

### 1. Configurar un cliente en KeyCloak

1. Inicia sesión en la consola de administración de KeyCloak
2. Selecciona tu Reino (Realm)
3. Ve a "Clients" → "Create"
4. Completa el formulario:
   - **Client ID**: `oxicloud`
   - **Client Protocol**: `openid-connect`
   - **Root URL**: `https://oxicloud.example.com`
5. En la configuración del cliente:
   - **Access Type**: `confidential`
   - **Valid Redirect URIs**: `https://oxicloud.example.com/oidc/callback/keycloak`
   - **Web Origins**: `https://oxicloud.example.com` (o `+` para permitir todos los orígenes)
6. Guarda la configuración
7. Ve a la pestaña "Credentials" y copia el "Secret" generado

### 2. Configurar OxiCloud para KeyCloak

```yaml
# docker-compose.yml
version: '3'
services:
  oxicloud:
    image: oxicloud:latest
    environment:
      # Configuración general
      OXICLOUD_ENABLE_OIDC: "true"
      
      # Configuración de KeyCloak
      OXICLOUD_OIDC_PROVIDER_KEYCLOAK_NAME: "KeyCloak"
      OXICLOUD_OIDC_PROVIDER_KEYCLOAK_CLIENT_ID: "oxicloud"
      OXICLOUD_OIDC_PROVIDER_KEYCLOAK_CLIENT_SECRET: "tu-client-secret-de-keycloak"
      OXICLOUD_OIDC_PROVIDER_KEYCLOAK_DISCOVERY_URL: "https://keycloak.example.com/realms/tu-realm/.well-known/openid-configuration"
      OXICLOUD_OIDC_PROVIDER_KEYCLOAK_REDIRECT_URI: "https://oxicloud.example.com/oidc/callback/keycloak"
      OXICLOUD_OIDC_PROVIDER_KEYCLOAK_SCOPES: "openid profile email"
      OXICLOUD_OIDC_PROVIDER_KEYCLOAK_USER_ID_ATTRIBUTE: "sub"
      OXICLOUD_OIDC_PROVIDER_KEYCLOAK_DEFAULT_ROLE: "user"
      OXICLOUD_OIDC_PROVIDER_KEYCLOAK_AUTO_CREATE_USERS: "true"
    ports:
      - "8085:8085"
    volumes:
      - ./storage:/app/storage
```

## Resolución de Problemas

### Error: "Failed to discover OIDC provider"

Este error ocurre cuando OxiCloud no puede acceder al punto de descubrimiento del proveedor OIDC.

**Soluciones:**
1. Verifica que la URL de descubrimiento sea correcta
2. Asegúrate de que OxiCloud pueda acceder a la URL (verifique firewalls, DNS, etc.)
3. Si tu proveedor utiliza un certificado autofirmado, asegúrate de configurar la confianza adecuada

### Error: "Invalid redirect URI"

Tu proveedor OIDC rechaza la URI de redirección.

**Soluciones:**
1. Asegúrate de que la URI de redirección configurada en OxiCloud coincida exactamente con la registrada en tu proveedor OIDC
2. Verifica que no haya diferencias en protocolo (http vs https), puerto o ruta

### Error: "User does not exist and auto-creation is disabled"

**Soluciones:**
1. Habilita la creación automática de usuarios: `OXICLOUD_OIDC_PROVIDER_<NOMBRE>_AUTO_CREATE_USERS="true"`
2. O crea manualmente el usuario en OxiCloud antes de intentar iniciar sesión con OIDC

### Error: "Could not extract user ID from claim"

OxiCloud no puede encontrar el atributo de ID de usuario especificado en los claims del token.

**Soluciones:**
1. Verifica que el atributo configurado (`USER_ID_ATTRIBUTE`) exista en los claims del token
2. Prueba con un atributo diferente, como "sub", "email" o "preferred_username"
3. Configura tu proveedor OIDC para incluir el atributo necesario en los tokens