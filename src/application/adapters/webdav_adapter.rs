/**
 * WebDAV Adapter Module
 * 
 * This module provides adapters for converting between OxiCloud's domain models 
 * and WebDAV protocol representations. It handles XML parsing and generation
 * for all WebDAV operations (PROPFIND, PROPPATCH, etc.) according to RFC 4918.
 * 
 * The adapter serves as a translation layer between the WebDAV protocol's XML-based
 * communication format and OxiCloud's internal data models, ensuring proper
 * serialization and deserialization of WebDAV requests and responses.
 */

use std::io::{Read, Write};
use quick_xml::{Reader, Writer, events::{Event, BytesStart, BytesEnd, BytesText}};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use thiserror::Error;

use crate::application::dtos::file_dto::FileDto;
use crate::application::dtos::folder_dto::FolderDto;

/**
 * Error types specific to WebDAV operations.
 * These errors encapsulate the various failure modes during WebDAV processing.
 */
#[derive(Error, Debug)]
pub enum WebDavError {
    /// Error during XML parsing or generation
    #[error("XML error: {0}")]
    XmlError(String),
    
    /// Error related to property handling
    #[error("Property error: {0}")]
    PropertyError(String),
    
    /// Error in the request format or content
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    
    /// I/O error during reading or writing
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    
    /// Other WebDAV related errors
    #[error("WebDAV error: {0}")]
    WebDavError(String),
}

/// Type alias for WebDAV operation results
pub type Result<T> = std::result::Result<T, WebDavError>;

/**
 * Property namespace and name, used to identify WebDAV properties.
 * WebDAV properties are identified by a combination of namespace and name.
 */
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyName {
    /// XML namespace for the property (e.g., "DAV:")
    pub namespace: String,
    /// Local name of the property (e.g., "displayname")
    pub name: String,
}

/**
 * Represents a WebDAV property with its name and value.
 * WebDAV properties contain metadata about resources.
 */
#[derive(Debug, Clone)]
pub struct Property {
    /// The qualified name of the property
    pub name: PropertyName,
    /// The property value, if any
    pub value: Option<String>,
}

/**
 * Represents a PROPFIND request as defined in RFC 4918.
 * PROPFIND requests can ask for all properties, named properties,
 * or property names only.
 */
#[derive(Debug, Clone)]
pub enum PropFindRequest {
    /// Request all properties
    AllProps,
    /// Request specific properties by name
    PropNames(Vec<PropertyName>),
    /// Request only property names without values
    PropNameOnly,
}

/**
 * Adapter for WebDAV operations, providing XML serialization and deserialization.
 * This struct contains methods for parsing WebDAV requests and generating
 * appropriate responses according to the WebDAV specification.
 */
pub struct WebDavAdapter;

impl WebDavAdapter {
    // XML namespaces used in WebDAV
    const DAV_NS: &'static str = "DAV:";
    
    /**
     * Parses a PROPFIND request body into a structured representation.
     * 
     * Processes the XML body of a PROPFIND request to determine which
     * properties are being requested (allprop, propname, or specific props).
     * 
     * @param reader Source providing XML content to parse
     * @return Result containing the parsed PropFindRequest or an error
     */
    pub fn parse_propfind<R: Read>(reader: R) -> Result<PropFindRequest> {
        let mut xml_reader = Reader::from_reader(reader);
        xml_reader.trim_text(true);
        
        let mut buf = Vec::new();
        let mut inside_propfind = false;
        let mut prop_names = Vec::new();
        let mut result = None;
        
        loop {
            match xml_reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let name = e.name();
                    let name_str = std::str::from_utf8(name).map_err(|_| {
                        WebDavError::XmlError("Invalid XML element name".to_string())
                    })?;
                    
                    if name_str == "propfind" {
                        inside_propfind = true;
                    } else if inside_propfind {
                        match name_str {
                            "allprop" => {
                                result = Some(PropFindRequest::AllProps);
                            },
                            "propname" => {
                                result = Some(PropFindRequest::PropNameOnly);
                            },
                            "prop" => {
                                // Will collect property names in subsequent iterations
                            },
                            _ if inside_propfind => {
                                // Handle property names within prop element
                                let namespace = Self::get_namespace_from_element(e)?;
                                prop_names.push(PropertyName {
                                    namespace: namespace.unwrap_or_else(|| Self::DAV_NS.to_string()),
                                    name: name_str.to_string(),
                                });
                            },
                            _ => {}
                        }
                    }
                },
                Ok(Event::Empty(ref e)) => {
                    // Handle self-closing tags
                    let name = e.name();
                    let name_str = std::str::from_utf8(name).map_err(|_| {
                        WebDavError::XmlError("Invalid XML element name".to_string())
                    })?;
                    
                    if inside_propfind && name_str != "prop" {
                        let namespace = Self::get_namespace_from_element(e)?;
                        prop_names.push(PropertyName {
                            namespace: namespace.unwrap_or_else(|| Self::DAV_NS.to_string()),
                            name: name_str.to_string(),
                        });
                    }
                },
                Ok(Event::End(ref e)) => {
                    let name = e.name();
                    let name_str = std::str::from_utf8(name).map_err(|_| {
                        WebDavError::XmlError("Invalid XML element name".to_string())
                    })?;
                    
                    if name_str == "propfind" {
                        inside_propfind = false;
                    }
                },
                Ok(Event::Eof) => break,
                Err(e) => return Err(WebDavError::XmlError(format!("Error parsing XML: {}", e))),
                _ => (),
            }
            
            buf.clear();
        }
        
        if !prop_names.is_empty() {
            return Ok(PropFindRequest::PropNames(prop_names));
        }
        
        result.ok_or_else(|| WebDavError::InvalidRequest("Invalid or missing propfind request".to_string()))
    }
    
    /**
     * Extracts the namespace from an XML element.
     * 
     * @param element The XML element to extract namespace from
     * @return Result containing the optional namespace or an error
     */
    fn get_namespace_from_element(element: &BytesStart) -> Result<Option<String>> {
        // Extract namespace from qualified name (e.g., "d:prop" -> "d")
        let name = std::str::from_utf8(element.name()).map_err(|_| {
            WebDavError::XmlError("Invalid XML element name".to_string())
        })?;
        
        if let Some(pos) = name.find(':') {
            let prefix = &name[..pos];
            
            // Find namespace declaration for this prefix
            for attr in element.attributes() {
                let attr = attr.map_err(|e| WebDavError::XmlError(format!("Invalid attribute: {}", e)))?;
                let key = std::str::from_utf8(attr.key).map_err(|_| {
                    WebDavError::XmlError("Invalid attribute name".to_string())
                })?;
                
                if key == format!("xmlns:{}", prefix) {
                    let value = std::str::from_utf8(&attr.value).map_err(|_| {
                        WebDavError::XmlError("Invalid attribute value".to_string())
                    })?;
                    return Ok(Some(value.to_string()));
                }
            }
        }
        
        Ok(None)
    }
    
    /**
     * Generates a PROPFIND response for a file.
     * 
     * Creates an XML response containing the requested properties
     * for a single file resource.
     * 
     * @param writer The output destination for the generated XML
     * @param file The file DTO containing the resource data
     * @param request The original PROPFIND request specifying which properties to include
     * @param depth The requested depth (0, 1, or infinity)
     * @param href The URL of the resource
     * @return Result indicating success or containing an error
     */
    pub fn generate_propfind_response_for_file<W: Write>(
        writer: W,
        file: &FileDto,
        request: &PropFindRequest,
        depth: &str,
        href: &str,
    ) -> Result<()> {
        let mut xml_writer = Writer::new(writer);
        
        // Start multistatus response
        let mut multistatus = BytesStart::owned(b"d:multistatus".to_vec(), "d:multistatus".len());
        multistatus.push_attribute(("xmlns:d", "DAV:"));
        xml_writer.write_event(Event::Start(multistatus)).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write multistatus start: {}", e))
        })?;
        
        // Generate response for the file
        Self::write_resource_properties(
            &mut xml_writer,
            href,
            file.updated_at,
            file.size as u64,
            false, // is_collection
            request,
        )?;
        
        // End multistatus
        xml_writer.write_event(Event::End(BytesEnd::borrowed(b"d:multistatus"))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write multistatus end: {}", e))
        })?;
        
        Ok(())
    }
    
    /**
     * Generates a PROPFIND response for a directory and its contents.
     * 
     * Creates an XML response containing the requested properties for a
     * directory and its children (files and subdirectories) based on the depth.
     * 
     * @param writer The output destination for the generated XML
     * @param folder The folder DTO (or None for root)
     * @param files List of file DTOs contained in the folder
     * @param subfolders List of subfolder DTOs contained in the folder
     * @param request The original PROPFIND request specifying which properties to include
     * @param depth The requested depth (0, 1, or infinity)
     * @param base_href The base URL of the resource
     * @return Result indicating success or containing an error
     */
    pub fn generate_propfind_response<W: Write>(
        writer: W,
        folder: Option<&FolderDto>,
        files: &[FileDto],
        subfolders: &[FolderDto],
        request: &PropFindRequest,
        depth: &str,
        base_href: &str,
    ) -> Result<()> {
        let mut xml_writer = Writer::new(writer);
        
        // Start multistatus response
        let mut multistatus = BytesStart::owned(b"d:multistatus".to_vec(), "d:multistatus".len());
        multistatus.push_attribute(("xmlns:d", "DAV:"));
        xml_writer.write_event(Event::Start(multistatus)).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write multistatus start: {}", e))
        })?;
        
        // Add folder properties
        if let Some(folder) = folder {
            Self::write_resource_properties(
                &mut xml_writer,
                base_href,
                folder.updated_at,
                0, // Size for directories is typically 0
                true, // is_collection
                request,
            )?;
        }
        
        // If depth > 0, include children
        if depth != "0" {
            // Add files
            for file in files {
                let file_href = format!("{}{}", base_href, file.name);
                Self::write_resource_properties(
                    &mut xml_writer,
                    &file_href,
                    file.updated_at,
                    file.size as u64,
                    false, // is_collection
                    request,
                )?;
            }
            
            // Add subfolders
            for subfolder in subfolders {
                let folder_href = format!("{}{}/", base_href, subfolder.name);
                Self::write_resource_properties(
                    &mut xml_writer,
                    &folder_href,
                    subfolder.updated_at,
                    0, // Size for directories is typically 0
                    true, // is_collection
                    request,
                )?;
            }
        }
        
        // End multistatus
        xml_writer.write_event(Event::End(BytesEnd::borrowed(b"d:multistatus"))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write multistatus end: {}", e))
        })?;
        
        Ok(())
    }
    
    /**
     * Writes the properties for a single resource in a PROPFIND response.
     * 
     * Helper method to generate XML for a single resource's properties,
     * used by both file and directory PROPFIND responses.
     * 
     * @param writer The XML writer to output to
     * @param href The URL of the resource
     * @param last_modified Last modification timestamp of the resource
     * @param size Size of the resource in bytes
     * @param is_collection Whether the resource is a collection (directory)
     * @param request The original PROPFIND request specifying which properties to include
     * @return Result indicating success or containing an error
     */
    fn write_resource_properties<W: Write>(
        xml_writer: &mut Writer<W>,
        href: &str,
        last_modified: DateTime<Utc>,
        size: u64,
        is_collection: bool,
        request: &PropFindRequest,
    ) -> Result<()> {
        // Start response element
        xml_writer.write_event(Event::Start(BytesStart::borrowed(b"d:response", "d:response".len()))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write response start: {}", e))
        })?;
        
        // Write href
        xml_writer.write_event(Event::Start(BytesStart::borrowed(b"d:href", "d:href".len()))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write href start: {}", e))
        })?;
        xml_writer.write_event(Event::Text(BytesText::from_plain_str(href))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write href text: {}", e))
        })?;
        xml_writer.write_event(Event::End(BytesEnd::borrowed(b"d:href"))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write href end: {}", e))
        })?;
        
        // Start propstat
        xml_writer.write_event(Event::Start(BytesStart::borrowed(b"d:propstat", "d:propstat".len()))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write propstat start: {}", e))
        })?;
        
        // Start prop
        xml_writer.write_event(Event::Start(BytesStart::borrowed(b"d:prop", "d:prop".len()))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write prop start: {}", e))
        })?;
        
        // Determine which properties to include based on the request
        match request {
            PropFindRequest::AllProps => {
                // Include standard properties
                Self::write_standard_properties(xml_writer, last_modified, size, is_collection)?;
            },
            PropFindRequest::PropNames(props) => {
                // Include only the requested properties
                for prop_name in props {
                    if prop_name.namespace == Self::DAV_NS {
                        match prop_name.name.as_str() {
                            "resourcetype" => Self::write_resourcetype(xml_writer, is_collection)?,
                            "getcontentlength" => {
                                if !is_collection {
                                    Self::write_getcontentlength(xml_writer, size)?;
                                }
                            },
                            "getlastmodified" => Self::write_getlastmodified(xml_writer, last_modified)?,
                            "creationdate" => Self::write_creationdate(xml_writer, last_modified)?,
                            "displayname" => {
                                // Extract displayname from href
                                let display_name = href.split('/').last().unwrap_or(href);
                                Self::write_displayname(xml_writer, display_name)?;
                            },
                            "getcontenttype" => {
                                if !is_collection {
                                    // For files, try to determine MIME type
                                    let content_type = if is_collection {
                                        "httpd/unix-directory"
                                    } else {
                                        mime_guess::from_path(href)
                                            .first_or_octet_stream()
                                            .as_ref()
                                    };
                                    Self::write_getcontenttype(xml_writer, content_type)?;
                                }
                            },
                            // Add other standard properties as needed
                            _ => {
                                // Unknown property - return empty element
                                xml_writer.write_event(Event::Empty(BytesStart::borrowed(
                                    format!("d:{}", prop_name.name).as_bytes(),
                                    prop_name.name.len() + 2,
                                ))).map_err(|e| {
                                    WebDavError::XmlError(format!("Failed to write property: {}", e))
                                })?;
                            }
                        }
                    }
                }
            },
            PropFindRequest::PropNameOnly => {
                // Just include empty property elements
                xml_writer.write_event(Event::Empty(BytesStart::borrowed(b"d:resourcetype", "d:resourcetype".len()))).map_err(|e| {
                    WebDavError::XmlError(format!("Failed to write resourcetype: {}", e))
                })?;
                
                if !is_collection {
                    xml_writer.write_event(Event::Empty(BytesStart::borrowed(b"d:getcontentlength", "d:getcontentlength".len()))).map_err(|e| {
                        WebDavError::XmlError(format!("Failed to write getcontentlength: {}", e))
                    })?;
                }
                
                xml_writer.write_event(Event::Empty(BytesStart::borrowed(b"d:getlastmodified", "d:getlastmodified".len()))).map_err(|e| {
                    WebDavError::XmlError(format!("Failed to write getlastmodified: {}", e))
                })?;
                
                xml_writer.write_event(Event::Empty(BytesStart::borrowed(b"d:creationdate", "d:creationdate".len()))).map_err(|e| {
                    WebDavError::XmlError(format!("Failed to write creationdate: {}", e))
                })?;
                
                xml_writer.write_event(Event::Empty(BytesStart::borrowed(b"d:displayname", "d:displayname".len()))).map_err(|e| {
                    WebDavError::XmlError(format!("Failed to write displayname: {}", e))
                })?;
                
                if !is_collection {
                    xml_writer.write_event(Event::Empty(BytesStart::borrowed(b"d:getcontenttype", "d:getcontenttype".len()))).map_err(|e| {
                        WebDavError::XmlError(format!("Failed to write getcontenttype: {}", e))
                    })?;
                }
            }
        }
        
        // End prop
        xml_writer.write_event(Event::End(BytesEnd::borrowed(b"d:prop"))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write prop end: {}", e))
        })?;
        
        // Write status
        xml_writer.write_event(Event::Start(BytesStart::borrowed(b"d:status", "d:status".len()))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write status start: {}", e))
        })?;
        xml_writer.write_event(Event::Text(BytesText::from_plain_str("HTTP/1.1 200 OK"))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write status text: {}", e))
        })?;
        xml_writer.write_event(Event::End(BytesEnd::borrowed(b"d:status"))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write status end: {}", e))
        })?;
        
        // End propstat
        xml_writer.write_event(Event::End(BytesEnd::borrowed(b"d:propstat"))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write propstat end: {}", e))
        })?;
        
        // End response
        xml_writer.write_event(Event::End(BytesEnd::borrowed(b"d:response"))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write response end: {}", e))
        })?;
        
        Ok(())
    }
    
    /**
     * Writes all standard WebDAV properties for a resource.
     * 
     * Helper method to write the core set of WebDAV properties that
     * most clients expect.
     * 
     * @param writer The XML writer to output to
     * @param last_modified Last modification timestamp of the resource
     * @param size Size of the resource in bytes
     * @param is_collection Whether the resource is a collection (directory)
     * @return Result indicating success or containing an error
     */
    fn write_standard_properties<W: Write>(
        xml_writer: &mut Writer<W>,
        last_modified: DateTime<Utc>,
        size: u64,
        is_collection: bool,
    ) -> Result<()> {
        // Write resourcetype (collection or not)
        Self::write_resourcetype(xml_writer, is_collection)?;
        
        // Write content length for files
        if !is_collection {
            Self::write_getcontentlength(xml_writer, size)?;
        }
        
        // Write last modified date
        Self::write_getlastmodified(xml_writer, last_modified)?;
        
        // Write creation date (using last modified as fallback)
        Self::write_creationdate(xml_writer, last_modified)?;
        
        // Add other standard properties as needed
        
        Ok(())
    }
    
    // Helper methods for writing specific properties
    
    /**
     * Writes the resourcetype property.
     * Indicates whether the resource is a collection (directory) or regular resource.
     */
    fn write_resourcetype<W: Write>(xml_writer: &mut Writer<W>, is_collection: bool) -> Result<()> {
        xml_writer.write_event(Event::Start(BytesStart::borrowed(b"d:resourcetype", "d:resourcetype".len()))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write resourcetype start: {}", e))
        })?;
        
        if is_collection {
            xml_writer.write_event(Event::Empty(BytesStart::borrowed(b"d:collection", "d:collection".len()))).map_err(|e| {
                WebDavError::XmlError(format!("Failed to write collection: {}", e))
            })?;
        }
        
        xml_writer.write_event(Event::End(BytesEnd::borrowed(b"d:resourcetype"))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write resourcetype end: {}", e))
        })?;
        
        Ok(())
    }
    
    /**
     * Writes the getcontentlength property.
     * Contains the size of the resource in bytes.
     */
    fn write_getcontentlength<W: Write>(xml_writer: &mut Writer<W>, size: u64) -> Result<()> {
        xml_writer.write_event(Event::Start(BytesStart::borrowed(b"d:getcontentlength", "d:getcontentlength".len()))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write getcontentlength start: {}", e))
        })?;
        
        xml_writer.write_event(Event::Text(BytesText::from_plain_str(&size.to_string()))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write getcontentlength text: {}", e))
        })?;
        
        xml_writer.write_event(Event::End(BytesEnd::borrowed(b"d:getcontentlength"))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write getcontentlength end: {}", e))
        })?;
        
        Ok(())
    }
    
    /**
     * Writes the getlastmodified property.
     * Contains the last modification date in RFC 822 format.
     */
    fn write_getlastmodified<W: Write>(xml_writer: &mut Writer<W>, last_modified: DateTime<Utc>) -> Result<()> {
        xml_writer.write_event(Event::Start(BytesStart::borrowed(b"d:getlastmodified", "d:getlastmodified".len()))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write getlastmodified start: {}", e))
        })?;
        
        // Format as RFC 822 date as required by WebDAV
        let formatted_date = last_modified.to_rfc2822();
        xml_writer.write_event(Event::Text(BytesText::from_plain_str(&formatted_date))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write getlastmodified text: {}", e))
        })?;
        
        xml_writer.write_event(Event::End(BytesEnd::borrowed(b"d:getlastmodified"))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write getlastmodified end: {}", e))
        })?;
        
        Ok(())
    }
    
    /**
     * Writes the creationdate property.
     * Contains the creation date in ISO 8601 format.
     */
    fn write_creationdate<W: Write>(xml_writer: &mut Writer<W>, creation_date: DateTime<Utc>) -> Result<()> {
        xml_writer.write_event(Event::Start(BytesStart::borrowed(b"d:creationdate", "d:creationdate".len()))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write creationdate start: {}", e))
        })?;
        
        // Format as ISO 8601 date as required by WebDAV
        let formatted_date = creation_date.to_rfc3339();
        xml_writer.write_event(Event::Text(BytesText::from_plain_str(&formatted_date))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write creationdate text: {}", e))
        })?;
        
        xml_writer.write_event(Event::End(BytesEnd::borrowed(b"d:creationdate"))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write creationdate end: {}", e))
        })?;
        
        Ok(())
    }
    
    /**
     * Writes the displayname property.
     * Contains the human-readable name of the resource.
     */
    fn write_displayname<W: Write>(xml_writer: &mut Writer<W>, display_name: &str) -> Result<()> {
        xml_writer.write_event(Event::Start(BytesStart::borrowed(b"d:displayname", "d:displayname".len()))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write displayname start: {}", e))
        })?;
        
        xml_writer.write_event(Event::Text(BytesText::from_plain_str(display_name))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write displayname text: {}", e))
        })?;
        
        xml_writer.write_event(Event::End(BytesEnd::borrowed(b"d:displayname"))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write displayname end: {}", e))
        })?;
        
        Ok(())
    }
    
    /**
     * Writes the getcontenttype property.
     * Contains the MIME type of the resource.
     */
    fn write_getcontenttype<W: Write>(xml_writer: &mut Writer<W>, content_type: &str) -> Result<()> {
        xml_writer.write_event(Event::Start(BytesStart::borrowed(b"d:getcontenttype", "d:getcontenttype".len()))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write getcontenttype start: {}", e))
        })?;
        
        xml_writer.write_event(Event::Text(BytesText::from_plain_str(content_type))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write getcontenttype text: {}", e))
        })?;
        
        xml_writer.write_event(Event::End(BytesEnd::borrowed(b"d:getcontenttype"))).map_err(|e| {
            WebDavError::XmlError(format!("Failed to write getcontenttype end: {}", e))
        })?;
        
        Ok(())
    }
    
    // Additional helper methods for other WebDAV operations
    
    // ... (PROPPATCH, LOCK, UNLOCK, etc. implementations would go here)
}