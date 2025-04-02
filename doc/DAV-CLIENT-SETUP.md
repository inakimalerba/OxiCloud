# Configuración de Clientes DAV para OxiCloud

Esta guía proporciona instrucciones para configurar varios clientes que soportan WebDAV, CalDAV y CardDAV para conectar con OxiCloud.

## Tabla de Contenidos

1. [URLs de Conexión](#urls-de-conexión)
2. [Clientes WebDAV](#clientes-webdav)
3. [Clientes CalDAV](#clientes-caldav)
4. [Clientes CardDAV](#clientes-carddav)
5. [Solución de Problemas](#solución-de-problemas)

## URLs de Conexión

Usa las siguientes URLs para conectar tus clientes con OxiCloud:

- **WebDAV**: `https://tu-servidor.com/webdav/`
- **CalDAV**:
  - Principal: `https://tu-servidor.com/caldav/`
  - Calendario específico: `https://tu-servidor.com/caldav/{nombre-calendario}/`
- **CardDAV**:
  - Principal: `https://tu-servidor.com/carddav/addressbooks/`
  - Libreta específica: `https://tu-servidor.com/carddav/addressbooks/{nombre-libreta}/`

## Clientes WebDAV

### Windows

#### Windows Explorer

1. Abre el Explorador de Windows
2. Haz clic derecho en "Este equipo" y selecciona "Agregar una ubicación de red"
3. Haz clic en "Siguiente"
4. Selecciona "Elegir una ubicación de red personalizada" y haz clic en "Siguiente"
5. En el campo de dirección, introduce: `https://tu-servidor.com/webdav/`
6. Haz clic en "Siguiente"
7. Introduce un nombre para la conexión (ej. "OxiCloud")
8. Haz clic en "Siguiente" y luego en "Finalizar"
9. Introduce tus credenciales cuando se te soliciten

#### Problemas Comunes en Windows

- **Error de SSL**: Asegúrate de que tu certificado SSL sea válido y confiable para Windows
- **Bloqueo por WebClient**: Asegúrate de que el servicio "WebClient" de Windows esté activado
- **Límite de tamaño**: Windows limita por defecto las cargas a 50MB, modifica el registro para aumentarlo:

```
[HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\WebClient\Parameters]
"FileSizeLimitInBytes"=dword:00FFFFFF
```

### macOS

#### Finder

1. En Finder, haz clic en "Ir" en la barra de menú
2. Selecciona "Conectar al servidor..." o presiona ⌘+K
3. Introduce `https://tu-servidor.com/webdav/` como dirección del servidor
4. Haz clic en "Conectar"
5. Introduce tus credenciales cuando se te soliciten
6. Selecciona si deseas guardar la contraseña en el llavero

### Linux

#### GNOME Files (Nautilus)

1. Abre Nautilus (Archivos)
2. Haz clic en "Otras ubicaciones" en el panel lateral
3. En la parte inferior, introduce `davs://tu-servidor.com/webdav/` en "Conectar al servidor"
4. Haz clic en "Conectar"
5. Introduce tus credenciales cuando se te soliciten

#### Dolphin (KDE)

1. Abre Dolphin
2. En la barra de dirección, escribe `webdavs://tu-servidor.com/webdav/`
3. Introduce tus credenciales cuando se te soliciten

### Clientes Multiplataforma

#### Cyberduck

1. Descarga e instala [Cyberduck](https://cyberduck.io/)
2. Haz clic en "Nueva conexión"
3. Selecciona "WebDAV (HTTP/SSL)" como tipo de conexión
4. Introduce los siguientes datos:
   - Servidor: `tu-servidor.com`
   - Puerto: `443`
   - Ruta: `/webdav/`
   - Nombre de usuario: tu nombre de usuario
   - Contraseña: tu contraseña
5. Haz clic en "Conectar"

## Clientes CalDAV

### Apple Calendar (macOS/iOS)

#### macOS

1. Abre la aplicación Calendario
2. Haz clic en "Calendario" en la barra de menú
3. Selecciona "Añadir cuenta..."
4. Selecciona "Otra cuenta de CalDAV..."
5. Completa la información:
   - Correo electrónico: tu dirección de correo
   - Contraseña: tu contraseña
   - Dirección del servidor: `tu-servidor.com`
   - Ruta: `/caldav/` (deja en blanco si no funciona)
6. Haz clic en "Iniciar sesión"

#### iOS

1. Ve a Ajustes > Calendario > Cuentas > Añadir cuenta
2. Selecciona "Otra"
3. Selecciona "Añadir cuenta CalDAV"
4. Completa la información:
   - Servidor: `https://tu-servidor.com/caldav/`
   - Nombre de usuario: tu nombre de usuario
   - Contraseña: tu contraseña
   - Descripción: "OxiCloud Calendario"
5. Toca "Siguiente" y luego "Guardar"

### Mozilla Thunderbird con Lightning

1. Instala Thunderbird y la extensión Lightning
2. Haz clic en el botón de calendario en la barra lateral
3. Haz clic derecho en el panel izquierdo y selecciona "Nuevo calendario"
4. Selecciona "En la red" y haz clic en "Siguiente"
5. Selecciona "CalDAV" como formato
6. Introduce `https://tu-servidor.com/caldav/nombre-calendario/` como ubicación
7. Haz clic en "Siguiente", introduce un nombre para el calendario
8. Completa la configuración y haz clic en "Finalizar"
9. Introduce tus credenciales cuando se te soliciten

### Nextcloud Desktop Sync

1. Descarga e instala el cliente de sincronización de Nextcloud
2. Durante la configuración, selecciona "Solo sincronización de calendario y contactos"
3. Introduce `https://tu-servidor.com` como dirección del servidor
4. Introduce tus credenciales
5. En las opciones de sincronización, selecciona los calendarios que deseas sincronizar

## Clientes CardDAV

### Apple Contacts (macOS/iOS)

#### macOS

1. Abre la aplicación Contactos
2. Haz clic en "Contactos" en la barra de menú
3. Selecciona "Añadir cuenta..."
4. Selecciona "Otra cuenta de CardDAV..."
5. Completa la información:
   - Correo electrónico: tu dirección de correo
   - Contraseña: tu contraseña
   - Dirección del servidor: `tu-servidor.com`
   - Ruta: `/carddav/addressbooks/` (deja en blanco si no funciona)
6. Haz clic en "Iniciar sesión"

#### iOS

1. Ve a Ajustes > Contactos > Cuentas > Añadir cuenta
2. Selecciona "Otra"
3. Selecciona "Añadir cuenta CardDAV"
4. Completa la información:
   - Servidor: `https://tu-servidor.com/carddav/addressbooks/`
   - Nombre de usuario: tu nombre de usuario
   - Contraseña: tu contraseña
   - Descripción: "OxiCloud Contactos"
5. Toca "Siguiente" y luego "Guardar"

### Mozilla Thunderbird

1. Instala Thunderbird y la extensión CardBook
2. Abre CardBook desde el menú de Thunderbird
3. Haz clic en "Libreta de direcciones" > "Nuevo" > "Libreta de direcciones remota"
4. Selecciona "CardDAV" como tipo
5. Introduce `https://tu-servidor.com/carddav/addressbooks/nombre-libreta/` como URL
6. Introduce un nombre para la libreta de direcciones
7. Introduce tus credenciales
8. Haz clic en "Validar" y luego en "Aceptar"

### Cliente Evolution (Linux)

1. Abre Evolution
2. Ve a Archivo > Nuevo > Libreta de direcciones
3. Selecciona "CardDAV" como tipo
4. Introduce `https://tu-servidor.com/carddav/addressbooks/nombre-libreta/` como URL
5. Introduce un nombre para la libreta de direcciones
6. Introduce tus credenciales
7. Haz clic en "Aplicar"

## Solución de Problemas

### Problemas Comunes

1. **Error de autenticación**
   - Verifica que estés usando las credenciales correctas
   - Asegúrate de que tu cuenta tenga acceso a los recursos DAV
   - Si usas autenticación de dos factores, es posible que necesites crear una contraseña de aplicación específica

2. **No se pueden encontrar calendarios/libretas**
   - Verifica que hayas creado calendarios o libretas de direcciones en OxiCloud
   - Asegúrate de estar usando la URL correcta, incluyendo la terminación con `/`
   - Verifica los permisos de los recursos

3. **Error SSL/TLS**
   - Asegúrate de que tu certificado SSL sea válido y confiable
   - Verifica que la fecha y hora de tu dispositivo sean correctas
   - En algunos clientes, puede ser necesario confiar manualmente en el certificado

4. **Sincronización lenta**
   - Limita el número de elementos en tus calendarios y libretas de direcciones
   - Verifica la calidad de tu conexión a Internet
   - Algunas operaciones masivas (como importar muchos contactos) pueden tardar tiempo

### Herramientas de Diagnóstico

1. **Verificación de conectividad**: Prueba la conexión básica con:
   ```
   curl -v https://tu-servidor.com/webdav/
   ```

2. **Prueba de autenticación**:
   ```
   curl -v -u usuario:contraseña https://tu-servidor.com/webdav/
   ```

3. **Prueba de funcionalidad WebDAV**:
   ```
   curl -X PROPFIND -H "Depth: 1" -u usuario:contraseña https://tu-servidor.com/webdav/
   ```

4. **Prueba de funcionalidad CalDAV**:
   ```
   curl -X PROPFIND -H "Depth: 1" -u usuario:contraseña https://tu-servidor.com/caldav/
   ```

5. **Logs del servidor**: Si tienes acceso, revisa los logs del servidor para identificar problemas específicos.

### Contacto para Soporte

Si continúas experimentando problemas, contacta con soporte en:
- Email: soporte@ejemplo.com
- Foro: https://ejemplo.com/foro
- Sistema de tickets: https://soporte.ejemplo.com