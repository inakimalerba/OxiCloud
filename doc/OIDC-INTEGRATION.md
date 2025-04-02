# OIDC Integration for OxiCloud

This document outlines the implementation plan for adding OpenID Connect (OIDC) support to OxiCloud, enabling Single Sign-On (SSO) with identity providers like Authentik, Authelia, KeyCloak, and others.

## Overview

OpenID Connect (OIDC) is an identity layer built on top of the OAuth 2.0 protocol. It allows clients to verify the identity of end-users based on the authentication performed by an authorization server, as well as to obtain basic profile information about the end-user.

Implementing OIDC in OxiCloud will:
1. Allow users to authenticate using their existing identity provider (IdP) credentials
2. Reduce the need for separate username/password management in OxiCloud
3. Enhance security by leveraging modern authentication best practices
4. Provide a seamless experience for users already using SSO in their environment

## Implementation Plan

### 1. Add OIDC Configuration Options

Extend the `AuthConfig` struct in `src/common/config.rs`:

```rust
pub struct AuthConfig {
    pub jwt_secret: String,
    pub access_token_expiry_secs: i64,
    pub refresh_token_expiry_secs: i64,
    pub hash_memory_cost: u32,
    pub hash_time_cost: u32,
    
    // New OIDC configuration
    pub enable_oidc: bool,
    pub oidc_providers: Vec<OidcProviderConfig>,
}

pub struct OidcProviderConfig {
    pub name: String,             // Display name (e.g., "Authentik", "KeyCloak")
    pub client_id: String,        // OIDC client ID
    pub client_secret: String,    // OIDC client secret
    pub discovery_url: String,    // OIDC discovery URL (.well-known/openid-configuration)
    pub redirect_uri: String,     // Redirect URI after authentication
    pub scopes: Vec<String>,      // Scopes to request
    pub user_id_attribute: String, // Which claim to use as user ID
    pub default_role: String,     // Default role for new users
    pub auto_create_users: bool,  // Create users on first login
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            // Existing defaults...
            
            // OIDC defaults
            enable_oidc: false,
            oidc_providers: Vec::new(),
        }
    }
}
```

Update the environment variable handling in `AppConfig::from_env()` to include OIDC configurations.

### 2. Create OIDC Service Implementation

Add a new file `src/domain/services/oidc_service.rs`:

```rust
use openid::{Client, Discovered, DiscoveredClient, Options, Token, StandardClaims};
use std::sync::Arc;
use reqwest::Client as HttpClient;
use async_trait::async_trait;
use uuid::Uuid;

use crate::common::config::OidcProviderConfig;
use crate::domain::entities::user::{User, UserRole};
use crate::domain::repositories::user_repository::UserRepository;
use crate::common::errors::{DomainError, ErrorKind};

pub struct OidcService {
    providers: Vec<OidcProvider>,
    user_repository: Arc<dyn UserRepository>,
}

struct OidcProvider {
    config: OidcProviderConfig,
    client: DiscoveredClient,
}

impl OidcService {
    pub async fn new(
        configs: Vec<OidcProviderConfig>,
        user_repository: Arc<dyn UserRepository>,
    ) -> Result<Self, DomainError> {
        let http_client = HttpClient::new();
        let mut providers = Vec::new();
        
        for config in configs {
            let client = openid::Client::discover(
                http_client.clone(),
                &config.client_id,
                &config.client_secret,
                &config.redirect_uri,
                &config.discovery_url,
            )
            .await
            .map_err(|e| DomainError::new(
                ErrorKind::InternalError,
                "OIDC",
                format!("Failed to discover OIDC provider {}: {}", config.name, e)
            ))?;
            
            providers.push(OidcProvider {
                config: config.clone(),
                client,
            });
        }
        
        Ok(Self {
            providers,
            user_repository,
        })
    }
    
    pub fn get_provider(&self, provider_name: &str) -> Option<&OidcProvider> {
        self.providers.iter().find(|p| p.config.name == provider_name)
    }
    
    pub fn get_providers_info(&self) -> Vec<OidcProviderInfo> {
        self.providers.iter().map(|p| OidcProviderInfo {
            name: p.config.name.clone(),
            display_name: p.config.name.clone(),
        }).collect()
    }
    
    pub fn generate_authorization_url(&self, provider_name: &str, state: &str) -> Result<String, DomainError> {
        let provider = self.get_provider(provider_name).ok_or_else(|| DomainError::new(
            ErrorKind::NotFound,
            "OIDC",
            format!("Provider {} not found", provider_name)
        ))?;
        
        let mut options = Options::default();
        options.scope = Some(provider.config.scopes.join(" "));
        
        let auth_url = provider.client.auth_url(&options, Some(state));
        Ok(auth_url.to_string())
    }
    
    pub async fn process_callback(
        &self, 
        provider_name: &str, 
        code: &str, 
        state: &str
    ) -> Result<(User, Token<Discovered, StandardClaims>), DomainError> {
        let provider = self.get_provider(provider_name).ok_or_else(|| DomainError::new(
            ErrorKind::NotFound,
            "OIDC",
            format!("Provider {} not found", provider_name)
        ))?;
        
        // Exchange code for token
        let token = provider.client.request_token(code).await.map_err(|e| DomainError::new(
            ErrorKind::AccessDenied,
            "OIDC",
            format!("Failed to exchange code for token: {}", e)
        ))?;
        
        // Extract user information from claims
        let claims = token.id_token.payload().clone();
        
        // Get user ID from configured attribute
        let user_id_attr = &provider.config.user_id_attribute;
        let external_user_id = match user_id_attr.as_str() {
            "sub" => claims.sub.clone(),
            "email" => claims.email.clone().unwrap_or_default(),
            // Add other standard claims as needed
            _ => claims.additional_claims.get(user_id_attr)
                .and_then(|v| v.as_str().map(|s| s.to_string()))
                .unwrap_or_default(),
        };
        
        if external_user_id.is_empty() {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "OIDC",
                format!("Could not extract user ID from claim '{}'", user_id_attr)
            ));
        }
        
        // Check if user exists with this external ID
        let mapped_user_id = format!("{}:{}", provider_name, external_user_id);
        
        let user = match self.user_repository.get_user_by_external_id(&mapped_user_id).await {
            Ok(existing_user) => existing_user,
            Err(_) => {
                // User doesn't exist, create if allowed
                if !provider.config.auto_create_users {
                    return Err(DomainError::new(
                        ErrorKind::AccessDenied,
                        "OIDC",
                        "User does not exist and auto-creation is disabled"
                    ));
                }
                
                // Get user information from claims
                let email = claims.email.clone().unwrap_or_else(|| 
                    format!("{}@oidc.oxicloud.local", Uuid::new_v4())
                );
                
                let username = claims.preferred_username.clone()
                    .or_else(|| claims.email.clone())
                    .unwrap_or_else(|| format!("user_{}", Uuid::new_v4()));
                
                // Create the user
                let role = match provider.config.default_role.as_str() {
                    "admin" => UserRole::Admin,
                    _ => UserRole::User,
                };
                
                // Default quota
                let quota = 1024 * 1024 * 1024; // 1GB
                
                let mut new_user = User::new(
                    username,
                    email,
                    Uuid::new_v4().to_string(), // Random password, not used for OIDC
                    role,
                    quota,
                )?;
                
                // Set external ID
                new_user.set_external_id(Some(mapped_user_id));
                
                // Save user
                self.user_repository.create_user(new_user).await?
            }
        };
        
        Ok((user, token))
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct OidcProviderInfo {
    pub name: String,
    pub display_name: String,
}
```

### 3. Update User Entity

Modify `src/domain/entities/user.rs` to support external IDs for OIDC users:

```rust
#[derive(Debug, Clone)]
pub struct User {
    // Existing fields...
    external_id: Option<String>, // For OIDC users: "provider:external_id"
}

impl User {
    // Existing methods...
    
    pub fn external_id(&self) -> Option<&str> {
        self.external_id.as_deref()
    }
    
    pub fn set_external_id(&mut self, external_id: Option<String>) {
        self.external_id = external_id;
    }
    
    pub fn is_oidc_user(&self) -> bool {
        self.external_id.is_some()
    }
}
```

### 4. Update Database Schema

Add a new column to the users table in `db/schema.sql`:

```sql
ALTER TABLE auth.users ADD COLUMN IF NOT EXISTS external_id VARCHAR(255) UNIQUE;
```

### 5. Update the Auth Application Service

Modify `src/application/services/auth_application_service.rs` to add OIDC methods:

```rust
use crate::domain::services::oidc_service::{OidcService, OidcProviderInfo};
use crate::application::dtos::user_dto::{OidcAuthUrlDto, OidcCallbackDto, OidcProviderDto};

impl AuthApplicationService {
    // Add OIDC service
    pub fn with_oidc_service(mut self, oidc_service: Arc<OidcService>) -> Self {
        self.oidc_service = Some(oidc_service);
        self
    }
    
    // Get available OIDC providers
    pub fn get_oidc_providers(&self) -> Result<Vec<OidcProviderDto>, DomainError> {
        let oidc_service = self.oidc_service.as_ref()
            .ok_or_else(|| DomainError::new(
                ErrorKind::UnsupportedOperation,
                "Auth",
                "OIDC is not configured"
            ))?;
            
        let providers = oidc_service.get_providers_info();
        Ok(providers.into_iter().map(OidcProviderDto::from).collect())
    }
    
    // Generate authorization URL
    pub fn generate_oidc_auth_url(&self, dto: OidcAuthUrlDto) -> Result<String, DomainError> {
        let oidc_service = self.oidc_service.as_ref()
            .ok_or_else(|| DomainError::new(
                ErrorKind::UnsupportedOperation,
                "Auth",
                "OIDC is not configured"
            ))?;
            
        oidc_service.generate_authorization_url(&dto.provider, &dto.state)
    }
    
    // Process OIDC callback
    pub async fn process_oidc_callback(&self, dto: OidcCallbackDto) -> Result<AuthResponseDto, DomainError> {
        let oidc_service = self.oidc_service.as_ref()
            .ok_or_else(|| DomainError::new(
                ErrorKind::UnsupportedOperation,
                "Auth",
                "OIDC is not configured"
            ))?;
            
        let (user, _token) = oidc_service.process_callback(
            &dto.provider, 
            &dto.code, 
            &dto.state
        ).await?;
        
        // Generate access token
        let access_token = self.auth_service.generate_access_token(&user)
            .map_err(DomainError::from)?;
            
        // Generate refresh token
        let refresh_token = self.auth_service.generate_refresh_token();
        
        // Create session
        let session = Session::new(
            user.id().to_string(),
            refresh_token.clone(),
            None,
            None,
            self.auth_service.refresh_token_expiry_days(),
        );
        
        self.session_storage.create_session(session).await?;
        
        // Return auth response
        Ok(AuthResponseDto {
            user: UserDto::from(user),
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: self.auth_service.refresh_token_expiry_secs(),
        })
    }
}
```

### 6. Add Auth Handler Routes for OIDC

Update `src/interfaces/api/handlers/auth_handler.rs`:

```rust
pub fn auth_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh_token))
        .route("/me", get(get_current_user))
        .route("/change-password", put(change_password))
        .route("/logout", post(logout))
        // Add OIDC routes
        .route("/oidc/providers", get(get_oidc_providers))
        .route("/oidc/auth", post(generate_oidc_auth_url))
        .route("/oidc/callback", post(process_oidc_callback))
}

// Get available OIDC providers
async fn get_oidc_providers(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let auth_service = state.auth_service.as_ref()
        .ok_or_else(|| AppError::internal_error("Servicio de autenticación no configurado"))?;
    
    match auth_service.auth_application_service.get_oidc_providers() {
        Ok(providers) => Ok((StatusCode::OK, Json(providers))),
        Err(err) => Err(err.into()),
    }
}

// Generate OIDC authorization URL
async fn generate_oidc_auth_url(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<OidcAuthUrlDto>,
) -> Result<impl IntoResponse, AppError> {
    let auth_service = state.auth_service.as_ref()
        .ok_or_else(|| AppError::internal_error("Servicio de autenticación no configurado"))?;
    
    match auth_service.auth_application_service.generate_oidc_auth_url(dto) {
        Ok(url) => Ok((StatusCode::OK, Json(json!({ "url": url })))),
        Err(err) => Err(err.into()),
    }
}

// Process OIDC callback
async fn process_oidc_callback(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<OidcCallbackDto>,
) -> Result<impl IntoResponse, AppError> {
    let auth_service = state.auth_service.as_ref()
        .ok_or_else(|| AppError::internal_error("Servicio de autenticación no configurado"))?;
    
    match auth_service.auth_application_service.process_oidc_callback(dto).await {
        Ok(auth_response) => Ok((StatusCode::OK, Json(auth_response))),
        Err(err) => Err(err.into()),
    }
}
```

### 7. Update DTOs for OIDC

Create new DTOs in `src/application/dtos/user_dto.rs`:

```rust
use crate::domain::services::oidc_service::OidcProviderInfo;

#[derive(Debug, Clone, Serialize)]
pub struct OidcProviderDto {
    pub name: String,
    pub display_name: String,
}

impl From<OidcProviderInfo> for OidcProviderDto {
    fn from(info: OidcProviderInfo) -> Self {
        Self {
            name: info.name,
            display_name: info.display_name,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct OidcAuthUrlDto {
    pub provider: String,
    pub state: String,
    pub redirect_uri: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OidcCallbackDto {
    pub provider: String,
    pub code: String,
    pub state: String,
}
```

### 8. Add Frontend Integration

Create a new JavaScript file `static/js/oidcAuth.js`:

```javascript
// OIDC Authentication Module
const oidcAuth = {
  // Get available OIDC providers
  async getProviders() {
    try {
      const response = await fetch('/api/auth/oidc/providers');
      if (!response.ok) {
        throw new Error(`Failed to get OIDC providers: ${response.statusText}`);
      }
      return await response.json();
    } catch (error) {
      console.error('Error fetching OIDC providers:', error);
      return [];
    }
  },

  // Generate random state for CSRF protection
  generateState() {
    const array = new Uint8Array(16);
    window.crypto.getRandomValues(array);
    return Array.from(array, byte => byte.toString(16).padStart(2, '0')).join('');
  },

  // Start OIDC authentication flow
  async startAuth(providerName) {
    try {
      // Generate and store state
      const state = this.generateState();
      localStorage.setItem('oidc_state', state);
      
      // Get authorization URL
      const response = await fetch('/api/auth/oidc/auth', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          provider: providerName,
          state: state,
        }),
      });
      
      if (!response.ok) {
        throw new Error(`Failed to get auth URL: ${response.statusText}`);
      }
      
      const data = await response.json();
      
      // Redirect to authorization URL
      window.location.href = data.url;
    } catch (error) {
      console.error('Error starting OIDC auth:', error);
      alert('Failed to start authentication. Please try again.');
    }
  },

  // Handle OIDC callback
  async handleCallback() {
    // Parse URL parameters
    const urlParams = new URLSearchParams(window.location.search);
    const code = urlParams.get('code');
    const state = urlParams.get('state');
    const error = urlParams.get('error');
    
    // Check for errors
    if (error) {
      console.error('OIDC authentication error:', error);
      alert(`Authentication failed: ${error}`);
      window.location.href = '/login.html';
      return;
    }
    
    // Verify code and state
    if (!code || !state) {
      console.error('Missing code or state in callback');
      alert('Authentication failed: Invalid response');
      window.location.href = '/login.html';
      return;
    }
    
    // Verify state matches
    const savedState = localStorage.getItem('oidc_state');
    if (state !== savedState) {
      console.error('State mismatch - potential CSRF attack');
      alert('Authentication failed: Invalid state');
      window.location.href = '/login.html';
      return;
    }
    
    // Clear stored state
    localStorage.removeItem('oidc_state');
    
    try {
      // Extract provider from URL path or from saved data
      const pathParts = window.location.pathname.split('/');
      const provider = localStorage.getItem('oidc_provider') || 
                       (pathParts.length > 2 ? pathParts[2] : 'default');
      
      // Process callback
      const response = await fetch('/api/auth/oidc/callback', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          provider: provider,
          code: code,
          state: state,
        }),
      });
      
      if (!response.ok) {
        throw new Error(`Failed to process callback: ${response.statusText}`);
      }
      
      const authData = await response.json();
      
      // Store auth data and redirect to dashboard
      localStorage.setItem('auth_token', authData.access_token);
      localStorage.setItem('refresh_token', authData.refresh_token);
      localStorage.setItem('user', JSON.stringify(authData.user));
      
      window.location.href = '/index.html';
    } catch (error) {
      console.error('Error handling OIDC callback:', error);
      alert('Failed to complete authentication. Please try again.');
      window.location.href = '/login.html';
    }
  }
};

// Check if current page is callback page
if (window.location.pathname.includes('/oidc/callback')) {
  document.addEventListener('DOMContentLoaded', () => {
    oidcAuth.handleCallback();
  });
}
```

### 9. Update Login Page

Add OIDC login buttons to `static/login.html`:

```html
<!-- OIDC Login Section -->
<div class="oidc-login">
  <h3>Login with SSO</h3>
  <div id="oidc-providers">
    <!-- OIDC provider buttons will be added here dynamically -->
  </div>
</div>

<script>
  // Load OIDC providers
  async function loadOidcProviders() {
    try {
      const providers = await oidcAuth.getProviders();
      const providersContainer = document.getElementById('oidc-providers');
      
      if (providers.length === 0) {
        providersContainer.innerHTML = '<p>No SSO providers configured.</p>';
        return;
      }
      
      const buttons = providers.map(provider => {
        return `<button 
          class="btn btn-oidc" 
          data-provider="${provider.name}"
          onclick="startOidcAuth('${provider.name}')"
        >
          Login with ${provider.display_name}
        </button>`;
      }).join('');
      
      providersContainer.innerHTML = buttons;
    } catch (error) {
      console.error('Failed to load OIDC providers:', error);
    }
  }
  
  // Start OIDC authentication
  function startOidcAuth(providerName) {
    localStorage.setItem('oidc_provider', providerName);
    oidcAuth.startAuth(providerName);
  }
  
  // Load providers when page loads
  document.addEventListener('DOMContentLoaded', loadOidcProviders);
</script>
```

## Configuration Example

Here's how to configure OxiCloud to use OIDC with KeyCloak:

```yaml
# docker-compose.yml
version: '3'
services:
  oxicloud:
    image: oxicloud:latest
    environment:
      OXICLOUD_ENABLE_OIDC: "true"
      OXICLOUD_OIDC_PROVIDER_KEYCLOAK_NAME: "KeyCloak"
      OXICLOUD_OIDC_PROVIDER_KEYCLOAK_CLIENT_ID: "oxicloud"
      OXICLOUD_OIDC_PROVIDER_KEYCLOAK_CLIENT_SECRET: "your-client-secret"
      OXICLOUD_OIDC_PROVIDER_KEYCLOAK_DISCOVERY_URL: "https://keycloak.example.com/realms/your-realm/.well-known/openid-configuration"
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

## Additional Considerations

1. **Security**: OIDC connections should always use HTTPS. Ensure proper TLS configuration.

2. **User mapping**: Consider how user attributes from OIDC map to your application (roles, groups, etc.).

3. **Multiple providers**: The design supports multiple OIDC providers simultaneously.

4. **Session management**: Implement proper session handling for OIDC users.

5. **Access control**: Review how OIDC integration affects your application's permission model.

6. **Testing**: Create separate test IdP configurations for development and testing.