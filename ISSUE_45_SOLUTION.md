# Solución para el error "El usuario 'admin' ya existe"

## Problema

Cuando se intenta crear un usuario administrador en una instalación nueva de OxiCloud, aparece el siguiente error:

```
oxicloud-1 | 2025-04-12T10:47:26.643669Z ERROR oxicloud::interfaces::api::handlers::auth_handler: Registration failed for user admin: Already Exists: El usuario 'admin' ya existe
```

Este error ocurre porque las migraciones de la base de datos ya crean un usuario administrador por defecto como parte del proceso de inicialización.

## Solución implementada

Hemos mejorado el sistema para que maneje mejor el registro de usuarios administradores:

1. **En una instalación nueva**: 
   - Si registras cualquier usuario como administrador (sea cual sea su nombre), el sistema detectará que es una instalación nueva y eliminará automáticamente el usuario admin predeterminado.
   - Esto te permite crear tu propio usuario administrador con el nombre que prefieras desde el principio.

2. **En un sistema en uso**: 
   - No se permite crear nuevos usuarios administradores desde la página de registro una vez que ya existe un administrador en el sistema
   - Esto previene la creación no autorizada de usuarios con permisos elevados
   - El sistema tampoco permite tener múltiples usuarios con el mismo nombre (incluido "admin")

### Detalles técnicos

La solución implementa:

1. Detección inteligente de instalaciones nuevas basada en:
   - Verificación del número total de usuarios en el sistema
   - Verificación del número de usuarios administradores
   
2. Reconocimiento de usuarios administradores:
   - Un usuario es administrador si su nombre es "admin"
   - Un usuario es administrador si se proporciona un rol "admin" explícitamente
   - Los administradores reciben automáticamente una cuota de 100GB
   
3. Eliminación segura del usuario admin predeterminado:
   - Se detecta al inicio del registro si es una instalación nueva
   - Se elimina el admin predeterminado antes de continuar con el registro
   
4. Prevención de creación de múltiples administradores:
   - Una vez que existe un usuario administrador en el sistema, no se permite crear más administradores desde la página de registro
   - Solo se puede crear un administrador desde la página de registro durante la instalación inicial
   - Esto protege el sistema contra la creación no autorizada de usuarios con privilegios elevados

## Cómo usar esta funcionalidad

### En una instalación nueva:

1. Inicia OxiCloud por primera vez (las migraciones crearán automáticamente un usuario admin predeterminado)
2. Ve a la pantalla de registro y crea un usuario con:
   - **Nombre de usuario**: Cualquier nombre que prefieras (por ejemplo, "torrefacto")
   - **Contraseña**: La que tú quieras
   - **Email**: Tu correo electrónico
3. El sistema detectará automáticamente que se trata de una instalación nueva
4. Si es un usuario administrador (porque el nombre es "admin" o porque explícitamente quieres que sea admin), el sistema eliminará el admin predeterminado antes de continuar
5. Tu nuevo usuario se creará y podrás iniciar sesión con él

### Si necesitas restablecer el usuario administrador:

Si ya tienes un sistema en uso y necesitas restablecer el usuario administrador:

#### Opción 1: Usar el script proporcionado
```bash
cat scripts/reset_admin.sql | docker exec -i oxicloud-postgres-1 psql -U postgres -d oxicloud
```

#### Opción 2: Hacerlo manualmente
```bash
docker exec -it oxicloud-postgres-1 psql -U postgres -d oxicloud
```

```sql
SET search_path TO auth;
DELETE FROM auth.users WHERE username = 'admin';
SELECT username, email, role FROM auth.users;
```

Luego registra un nuevo usuario admin a través de la interfaz web.

## Nota técnica

El usuario administrador predeterminado se crea durante las migraciones con estos valores:

```sql
INSERT INTO auth.users (
    id, 
    username, 
    email, 
    password_hash, 
    role, 
    storage_quota_bytes
) VALUES (
    '00000000-0000-0000-0000-000000000000',
    'admin',
    'admin@oxicloud.local',
    '$argon2id$v=19$m=65536,t=3,p=4$c2FsdHNhbHRzYWx0c2FsdA$H3VxE8LL2qPT31DM3loTg6D+O4MSc2sD7GjlQ5h7Jkw', -- Admin123!
    'admin',
    107374182400  -- 100GB for admin
);
```
