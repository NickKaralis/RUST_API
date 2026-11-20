use mongodb::{bson::doc, options::ClientOptions, Client, Collection}; 
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub Struct 